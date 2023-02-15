use gloo::{storage::{LocalStorage, Storage}, console::*};
use graphql_client::QueryBody;
use reqwasm::http::*;
use serde::{Deserialize, Serialize};
use serde_json::to_string;

//use crate::api::login::*;

use super::ResponseWrapper;

const SERVER: &str = "http://127.0.0.1:8000/";


pub async fn post<V, T> (variables: QueryBody<V>) -> Result<T, String>
where
    for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
    V: Serialize + std::fmt::Debug,
{
    let body = to_string(&variables).unwrap();
    let token: Option<String> = LocalStorage::get("token").unwrap_or_else(|_| None);

    let mut builder = Request::post(SERVER)
        .body(body)
        .header("Content-Type", "application/json")
        .mode(RequestMode::Cors);

    if let Some(token) = token {
        let bearer = format!{"Bearer {}", token};
        builder = builder.header("Authorization", &bearer);
    } else {
        info!("No token, not sending auth");
    }

    let resp = builder.send().await;

    match resp {
        Ok(resp) => {
            if resp.ok() {
                let data: ResponseWrapper<T> = resp.json().await.unwrap();

                if let Some(errors) = data.errors {
                    let error_message = errors.first().unwrap().message.clone();
                    return Err(error_message);
                } else {
                    return Ok(data.data.unwrap());
                }
            } else {
                match resp.status() {
                    401 => return Err("Unauthorized".to_string()),
                    403 => return Err("Forbidden".to_string()),
                    404 => return Err("Not Found".to_string()),
                    500 => return Err("Internal Server Error".to_string()),
                    // TODO: 422 => return Err(Error::UnprocessableEntity(resp.text())),
                    _ => return Err("Http Request Error".to_string())
                }
            }
        }
        Err(error) => {
            error!(error.to_string());
            return Err("Http Request Error".to_string())
        }
    }
}