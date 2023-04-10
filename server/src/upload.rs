use actix_web::{
    error, http::header::ContentType, middleware, web, App, HttpResponse, HttpServer, Responder,
    Result,
};
use futures_util::stream::StreamExt as _;
use log::{debug, error, info};
use std::{env, fs::File, io::Write};

pub struct KeyFileData {
    pub data: std::path::PathBuf,
}

#[actix_web::post("/upload")]
async fn upload_handler(
    mut file: actix_multipart::Multipart,
    data: actix_web::web::Data<std::sync::Mutex<KeyFileData>>,
) -> HttpResponse {
    debug!("Reading file from request...");
    if let Some(Ok(mut file)) = file.next().await {
        debug!("Ok. File exists!");
        debug!("Locking data...");

        if let Ok(mut key_data_file) = data.lock() {
            debug!("Ok. Data locked!");

            let new_key = uuid::Uuid::new_v4();
            info!("New key for file: {}", new_key.to_string());

            let path = key_data_file.data.join(format!(
                "upload/{}.{}",
                new_key,
                file.headers()
                    .get("content-type")
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .split('/')
                    .collect::<Vec<&str>>()[1]
            ));

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

            key_data_file.data = path;
            println!("{:?}", key_data_file.data);

            return HttpResponse::Ok().body(new_key.to_string());
        }
        return HttpResponse::InternalServerError().body("Failed to lock data");
    }
    HttpResponse::BadRequest().body("File was not provided!")
}

#[actix_web::get("/getFile")]
async fn get_files(
    data: actix_web::web::Data<std::sync::Mutex<KeyFileData>>,
) -> Result<impl Responder> {
    if let Ok(key_data_file) = data.lock() {
        println!("{:?}", key_data_file.data);
        if let Some(f)  = key_data_file.data.clone().to_str() {
            return Ok(web::Json(f.to_owned()));
        } else {
            return Ok(web::Json("".to_owned()));
        }
    }
    return Err(error::ErrorExpectationFailed("Failed to lock data!"));
}

#[actix_web::get("/get/{key}")]
async fn render_file_handler(key: web::Path<String>) -> HttpResponse {
    debug!("Locking data...");

    debug!("Ok. Data locked!");
    let key = key.into_inner().clone();
    debug!("{}", key);
    if let Ok(file) = std::fs::read(format!("upload/{}", key)) {
        debug!("Sending file...");
        return HttpResponse::Ok().content_type("").body(file);
    } else {
        error!("Failed to open file reading...");
        return HttpResponse::InternalServerError().body("Faile to open file for reading!");
    }
}
