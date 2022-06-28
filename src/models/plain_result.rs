use crate::schema::*;
use chrono;
use diesel::{Associations, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid;

use crate::models::task::Task;

#[derive(
    Debug, Identifiable, Associations, Serialize, Deserialize, Insertable, Queryable, Clone,
)]
#[primary_key(plain_result_id)]
#[belongs_to(Task)]
#[table_name = "plain_results"]
pub struct PlainResult {
    pub plain_result_id: uuid::Uuid,
    pub plain_result_content: Vec<u8>,
    pub plain_result_created_at: chrono::NaiveDateTime,
    pub image_url: Option<String>,
    pub task_id: uuid::Uuid,
}

impl PlainResult {
    pub fn new(
        plain_result_content: Vec<u8>,
        image_url: Option<String>,
        task_id: uuid::Uuid,
    ) -> Self {
        PlainResult {
            plain_result_id: uuid::Uuid::new_v4(),
            plain_result_content,
            plain_result_created_at: chrono::Local::now().naive_local(),
            image_url,
            task_id,
        }
    }
}
