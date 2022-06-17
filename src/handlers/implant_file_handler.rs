use actix_files::NamedFile;
use actix_web::{get, HttpRequest};
use std::path::PathBuf;

use crate::errors::ServerError;

const IMPLANT_FILE_NAME: &str = "linuximplant";

#[get("/{linuximplant:.*}")]
pub async fn implant_file_get(req: HttpRequest) -> Result<NamedFile, ServerError> {
    let path: PathBuf = req
        .match_info()
        .query(IMPLANT_FILE_NAME)
        .parse()
        .expect("File is missing");

    Ok(NamedFile::open(path)?)
}
