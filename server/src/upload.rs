use actix_web::{middleware, web, App, HttpResponse, HttpServer, http::header::ContentType, Responder, Result, error};
use log::{debug, error, info};
use std::{env, fs::File, io::Write};
use futures_util::stream::StreamExt as _;

pub struct KeyFileData {
    pub data: std::collections::HashMap<String, std::path::PathBuf>,
    pub upload_folder: std::path::PathBuf,
}

#[actix_web::post("/upload")]
async fn upload_handler(
    mut file: actix_multipart::Multipart,
    data: actix_web::web::Data<std::sync::Mutex<KeyFileData>>,
) -> HttpResponse {
    debug!("Reading file from request...");
    if let Some(Ok(mut file)) = file.next().await {
        println!("{:?}", file.headers());
        debug!("Ok. File exists!");
        debug!("Locking data...");

        if let Ok(mut key_data_file) = data.lock() {
            debug!("Ok. Data locked!");

            let new_key = uuid::Uuid::new_v4();
            info!("New key for file: {}", new_key.to_string());

            let path = key_data_file
                .upload_folder
                .join(format!("{:?}", file.headers().keys()));

            debug!("New file path: {}", path.to_str().unwrap_or(""));
            debug!("Saving file...");

            if let Ok(mut f) = File::create(&path) {
                while let Some(Ok(chunk)) = file.next().await {
                    f.write(&chunk).unwrap(); // Trust
                }
                info!("File was successfully uploaded!");
            } else {
                error!("Failed to create file!");
                return HttpResponse::InternalServerError().body("Failed to write file!");
            }

            key_data_file.data.insert(new_key.to_string(), path);
            println!("{:?}", key_data_file.data);

            return HttpResponse::Ok().body(new_key.to_string());
        }
        return HttpResponse::InternalServerError().body("Failed to lock data");
    }
    HttpResponse::BadRequest().body("File was not provided!")
}

#[actix_web::get("/get")]
async fn get_files(
    data: actix_web::web::Data<std::sync::Mutex<KeyFileData>>,
) -> Result<impl Responder> {
    if let Ok(key_data_file) = data.lock() {
        let data: Vec<String> = key_data_file.data.keys().map(|e|e.clone()).collect();
        return Ok(web::Json(data));
    }
    return Err(error::ErrorExpectationFailed("Failed to lock data!"));
}

#[actix_web::get("/get/{key}")]
async fn render_file_handler(
    key: web::Path<String>,
    data: actix_web::web::Data<std::sync::Mutex<KeyFileData>>,
) -> HttpResponse {
    debug!("Locking data...");

    if let Ok(key_data_file) = data.lock() {
        debug!("Ok. Data locked!");
        let key = key.into_inner().clone();

        if key_data_file.data.contains_key(&key) {
            debug!("Key exists!");
            if let Ok(file) = std::fs::read_to_string(key_data_file.data.get(&key).unwrap()) {
                debug!("Sending file...");
                return HttpResponse::Ok().content_type("").body(file);
            } else {
                error!("Failed to open file reading...");
                return HttpResponse::InternalServerError().body("Faile to open file for reading!");
            }
        } else {
            error!("Key not found");
            return HttpResponse::NotFound().body("Key was not found!");
        }
    }
    error!("Failed to lock data!");
    return HttpResponse::InternalServerError().body("Failed to lock data!");
}