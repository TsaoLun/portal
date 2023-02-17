use std::error::Error;

use gloo::storage::{LocalStorage, Storage};
use graphql_client::{GraphQLQuery, Response};
use reqwest::RequestBuilder;
use wasm_bindgen::UnwrapThrowExt;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/schema/schema.graphql",
    query_path = "src/api/schema/login.graphql",
    response_derives = "Debug, Clone",
    variables_derives = "Debug"
)]
pub struct Login;

pub async fn login(
    request: RequestBuilder,
    username: String,
    password: String,
) -> Result<(), Box<dyn Error>> {
    let body = Login::build_query(login::Variables { username, password });

    let response_body: Response<login::ResponseData> =
        request.json(&body).send().await?.json().await?;

    let token = response_body
        .data
        .expect_throw("login response err")
        .login
        .token;
    LocalStorage::set("token", token).unwrap();
    Ok(())
}
