use gloo::console::log;
use graphql_client::{GraphQLQuery, Response};
use reqwest::RequestBuilder;
use std::error::Error;
use wasm_bindgen::UnwrapThrowExt;

use crate::utils::api_response::{ApiResponse, ErrData};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/schema/schema.graphql",
    query_path = "src/api/schema/set.graphql",
    response_derives = "Debug, Clone",
    variables_derives = "Debug"
)]
pub struct Set;

pub async fn set_mutation(
    request: RequestBuilder,
    data: String,
) -> Result<ApiResponse<ErrData>, Box<dyn Error>> {
    let request_body = Set::build_query(set::Variables { data });
    let response_body: Response<set::ResponseData> =
        request.json(&request_body).send().await?.json().await?;
    if response_body.errors.is_some() {
        let err = response_body.errors.unwrap();
        let ext_err = err
            .get(0)
            .unwrap()
            .extensions
            .as_deref_mut()
            .and_then(|e| e.get("code").and_then(|code| Some(serde_json::from_value(*code))));
        return Ok(ApiResponse(Some(ErrData {
            message: err.get(0).unwrap().message.to_string(),
            code: ext_err.unwrap(),
        })));
    }
    if response_body.data.and_then(|r| Some(r.set)) == Some(true) {
        Ok(ApiResponse(None))
    } else {
        Ok(ApiResponse(Some(ErrData {
            message: "服务器异常".into(),
            code: None,
        })))
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/schema/schema.graphql",
    query_path = "src/api/schema/get.graphql",
    response_derives = "Debug, Clone",
    variables_derives = "Debug"
)]
pub struct Get;

pub async fn get_query(request: RequestBuilder) -> Result<String, Box<dyn Error>> {
    let request_body = Get::build_query(get::Variables);
    let response_body: Response<get::ResponseData> =
        request.json(&request_body).send().await?.json().await?;
    if response_body.errors.is_some() {
        let err = response_body.errors.unwrap().get(0).unwrap().to_string();
        return Err(err.into());
    }
    Ok(response_body.data.expect_throw("response data err").get)
}
