use crate::schema::*;
use chrono;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "implants"]
pub struct Implant {
    pub implant_id: uuid::Uuid,
    pub public_key: String,
    pub server_private_key: String,
    pub created_at: chrono::NaiveDateTime,
}

impl Implant {
    pub fn new(public_key: String, server_private_key: String) -> Self {
        Implant {
            implant_id: uuid::Uuid::new_v4(),
            public_key,
            server_private_key,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}
