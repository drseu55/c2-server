use crate::schema::*;
use chrono;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub user_id: uuid::Uuid,
    pub username: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
}

impl User {
    pub fn from_details<S: Into<String>, T: Into<String>>(username: S, password: T) -> Self {
        User {
            user_id: uuid::Uuid::new_v4(),
            username: username.into(),
            password: password.into(),
            created_at: chrono::Local::now().naive_local(),
        }
    }
}
