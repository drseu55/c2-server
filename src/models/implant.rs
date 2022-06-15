use crate::schema::*;
use chrono;
use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid;

#[derive(Debug, Identifiable, Serialize, Deserialize, Queryable, Insertable)]
#[primary_key(implant_id)]
#[table_name = "implants"]
pub struct Implant {
    pub implant_id: uuid::Uuid,
    pub public_key: String,
    pub server_private_key: String,
    pub created_at: chrono::NaiveDateTime,
    pub external_ip_address: Option<String>,
    pub internal_ip_address: Option<String>,
    pub os_type: Option<String>,
    pub machine_user: Option<String>,
    pub machine_name: Option<String>,
    pub process_name: Option<String>,
    pub pid: Option<i32>,
    pub architecture: Option<String>,
}

impl Implant {
    pub fn new(public_key: String, server_private_key: String) -> Self {
        Implant {
            implant_id: uuid::Uuid::new_v4(),
            public_key,
            server_private_key,
            created_at: chrono::Local::now().naive_local(),
            external_ip_address: None,
            internal_ip_address: None,
            os_type: None,
            machine_user: None,
            machine_name: None,
            process_name: None,
            pid: None,
            architecture: None,
        }
    }
}

#[derive(Serialize)]
pub struct ImplantInfoReduced {
    pub implant_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub external_ip_address: Option<String>,
    pub internal_ip_address: Option<String>,
    pub os_type: Option<String>,
    pub machine_user: Option<String>,
    pub machine_name: Option<String>,
    pub process_name: Option<String>,
    pub pid: Option<i32>,
    pub architecture: Option<String>,
}

impl ImplantInfoReduced {
    pub fn new(
        implant_id: uuid::Uuid,
        created_at: chrono::NaiveDateTime,
        external_ip_address: Option<String>,
        internal_ip_address: Option<String>,
        os_type: Option<String>,
        machine_user: Option<String>,
        machine_name: Option<String>,
        process_name: Option<String>,
        pid: Option<i32>,
        architecture: Option<String>,
    ) -> Self {
        ImplantInfoReduced {
            implant_id,
            created_at,
            external_ip_address,
            internal_ip_address,
            os_type,
            machine_user,
            machine_name,
            process_name,
            pid,
            architecture,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SystemInfo {
    pub external_ip_address: String,
    pub internal_ip_address: String,
    pub os_type: String,
    pub machine_user: String,
    pub machine_name: String,
    pub process_name: String,
    pub pid: i32,
    pub architecture: String,
    pub implant_id: String,
}

#[derive(Serialize)]
pub struct AllImplants {
    pub implants: Vec<ImplantInfoReduced>,
}

impl AllImplants {
    pub fn new(implants: Vec<ImplantInfoReduced>) -> Self {
        AllImplants { implants }
    }
}
