use gloo::storage::{LocalStorage, Storage};

pub mod data;
pub mod login;

const URL: &str = "http://127.0.0.1:8000";

pub fn init_request() -> reqwest::RequestBuilder {
    let token: Option<String> = LocalStorage::get("token").unwrap_or_else(|_| None);
    if let Some(token) = token {
        reqwest::Client::new().post(URL).bearer_auth(token)
    } else {
        reqwest::Client::new().post(URL)
    }
}
