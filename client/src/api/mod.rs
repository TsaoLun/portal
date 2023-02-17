use gloo::storage::{LocalStorage, Storage};
use graphql_client::QueryBody;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

pub mod login;
pub mod data;

const url: &str = "http://127.0.0.1:8000";

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GraphQLError {
    pub message: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseWrapper<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<GraphQLError>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}

pub fn init_request() -> reqwest::RequestBuilder {
    let token: Option<String> = LocalStorage::get("token").unwrap_or_else(|_|None);
    if let Some(token) = token {
        reqwest::Client::new().post(url).bearer_auth(token)

    } else {
        reqwest::Client::new().post(url)
    }
}