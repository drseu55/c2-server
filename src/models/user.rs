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
    pub fn new(username: String, password: String) -> Self {
        User {
            user_id: uuid::Uuid::new_v4(),
            username,
            password,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}

#[derive(Deserialize)]
pub struct UserAuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserAuthResponse {
    pub token: String,
}

impl UserAuthResponse {
    pub fn new(token: String) -> Self {
        UserAuthResponse { token }
    }
}
