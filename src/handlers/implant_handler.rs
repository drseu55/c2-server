use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::prelude::*;

use crate::db::connect::Pool;
use crate::errors::ServerError;
use crate::models::implant::{AllImplants, Implant, ImplantInfoReduced};

#[get("/implants")]
pub async fn implants_get(db: web::Data<Pool>) -> Result<impl Responder, ServerError> {
    // Get all tasks for all implants from db
    let implants = web::block(move || get_all_implants(db)).await??;

    let response = AllImplants::new(implants);

    // Return json with all tasks in vector
    Ok(HttpResponse::Ok().json(response))
}

fn get_all_implants(db: web::Data<Pool>) -> Result<Vec<ImplantInfoReduced>, ServerError> {
    use crate::schema::implants::dsl::implants;

    let conn = db.get()?;

    let all_implants = implants.load::<Implant>(&conn)?;

    let mut implant_info_vec: Vec<ImplantInfoReduced> = vec![];

    for implant in all_implants {
        let implant_info = ImplantInfoReduced::new(
            implant.implant_id,
            implant.created_at,
            implant.external_ip_address,
            implant.internal_ip_address,
            implant.os_type,
            implant.machine_user,
            implant.machine_name,
            implant.process_name,
            implant.pid,
            implant.architecture,
        );

        implant_info_vec.push(implant_info);
    }

    Ok(implant_info_vec)
}
