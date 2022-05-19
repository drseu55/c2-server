use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::prelude::*;

use crate::db::connect::Pool;
use crate::errors::ServerError;
use crate::models::task;

#[get("/api/tasks/{implant_id}")]
pub async fn implant_tasks(db: web::Data<Pool>) -> impl Responder {
    // TODO: Get implant id from URL
    // TODO: Get only tasks from db that belongs to received implant id
    // TODO: Implement XChaCha20-Poly1305 for encrypted communication
    // TODO: Implement response structure
    HttpResponse::Ok().body("implant tasks get")
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

    let task = task::Task::new(req_body.task, "created".to_owned(), req_body.implant_id);

    diesel::dsl::insert_into(tasks)
        .values(&task)
        .execute(&conn)?;

    Ok(())
}
