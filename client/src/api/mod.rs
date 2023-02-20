use gloo::{storage::{LocalStorage, Storage}, utils::window};
use lazy_static::lazy_static;
use reqwest::{Client, RequestBuilder};

pub mod data;
pub mod login;

const PORT: &str = ":8008";

lazy_static! {
    pub static ref REQ: Client = init_request();
    static ref URL: String = init_url();
}

pub fn init_request() -> Client {
    reqwest::Client::new()
}

pub fn init_url() -> String {
    format!("http://{}", window().location().hostname().unwrap().replace("http://", "") + PORT)
}

pub fn request() -> RequestBuilder {
    let token: String = LocalStorage::get("token").unwrap_or_else(|_| "".to_string());
    REQ.post(URL.to_string()).bearer_auth(token)
}
