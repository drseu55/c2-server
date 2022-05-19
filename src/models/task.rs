use crate::schema::*;
use chrono;
use diesel::{Associations, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid;

use crate::models::implant::Implant;

#[derive(Debug, Identifiable, Associations, Serialize, Deserialize, Insertable, Queryable)]
#[primary_key(task_id)]
#[belongs_to(Implant)]
#[table_name = "tasks"]
pub struct Task {
    pub task_id: uuid::Uuid,
    pub task: String,
    pub created_at: chrono::NaiveDateTime,
    pub status: String,
    pub implant_id: uuid::Uuid,
}

impl Task {
    pub fn new(task: String, status: String, implant_id: uuid::Uuid) -> Self {
        Task {
            task_id: uuid::Uuid::new_v4(),
            task,
            created_at: chrono::Local::now().naive_local(),
            status,
            implant_id,
        }
    }
}

#[derive(Deserialize)]
pub struct AddTaskRequest {
    pub implant_id: uuid::Uuid,
    pub task: String,
}

#[derive(Serialize)]
pub struct AllTasksResponse {
    pub tasks: Vec<Task>,
}

impl AllTasksResponse {
    pub fn new(tasks: Vec<Task>) -> Self {
        AllTasksResponse { tasks }
    }
}
