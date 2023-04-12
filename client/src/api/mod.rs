use gloo::{
    storage::{LocalStorage, Storage},
    utils::window,
};
use lazy_static::lazy_static;
use reqwest::{Client, RequestBuilder, StatusCode};
use web_sys::Blob;
use js_sys::{
    ArrayBuffer,
    Uint8Array,
};
use wasm_bindgen_futures::*;
use futures::StreamExt;

pub mod data;
pub mod login;
pub mod upload;

lazy_static! {
    pub static ref REQ: Client = init_request();
    static ref URL: String = init_url();
}

pub fn init_request() -> Client {
    reqwest::Client::new()
}

pub fn init_url() -> String {
    format!("{}/graphql", window().location().origin().unwrap())
}

pub fn request() -> RequestBuilder {
    let token: String = LocalStorage::get("token").unwrap_or_else(|_| "".to_string());
    REQ.post(URL.to_string()).bearer_auth(token)
}

pub async fn upload_to_server_inner_task(file: web_sys::File) -> Result<StatusCode, anyhow::Error> {
    let fname = file.name();
    let blob: Blob = file.into();
    let blob =
        Uint8Array::new(
            JsFuture::from(blob.array_buffer()).await.unwrap().dyn_ref::<ArrayBuffer>().unwrap(),
        ).to_vec();
    let client = Client::new();
    let req =
        client
            .post(format!("{}/upload", window().location().origin().unwrap()))
            .header("Content-Length", &blob.len().to_string())
            .header("Content-Type", "application/octet-stream")
            .body(blob)
            .send()
            .await?;
    Ok(req.status())
}