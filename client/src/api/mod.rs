use gloo::storage::{LocalStorage, Storage};
use lazy_static::lazy_static;
use reqwest::{Client, RequestBuilder};

pub mod data;
pub mod login;

const URL: &str = "http://127.0.0.1:8000";

lazy_static! {
    pub static ref REQ: Client = init_request();
}

pub fn init_request() -> Client {
    reqwest::Client::new()
}

pub fn request() -> RequestBuilder {
    let token: String = LocalStorage::get("token").unwrap_or_else(|_| "".to_string());
    REQ.post(URL).bearer_auth(token)
}
