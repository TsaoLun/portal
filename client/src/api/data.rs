use graphql_client::{GraphQLQuery, Response};
use reqwest::RequestBuilder;

use crate::utils::api_response::{get_err, AppError, PARSER_ERROR, SERVER_ERROR};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/schema/schema.graphql",
    query_path = "src/api/schema/set.graphql",
    response_derives = "Debug, Clone",
    variables_derives = "Debug"
)]
pub struct Set;

pub async fn set_mutation(request: RequestBuilder, data: String) -> Result<bool, AppError> {
    let request_body = Set::build_query(set::Variables { data });
    let response_body: Response<set::ResponseData> = request
        .json(&request_body)
        .send()
        .await
        .map_err(|_| AppError::AnyError(SERVER_ERROR.into()))?
        .json()
        .await
        .map_err(|_| AppError::AnyError(PARSER_ERROR.into()))?;

    let v = response_body.errors.map(|e| e);
    if let Some(v) = v {
        get_err(&v)?;
    }

    let t = response_body.data.map(|r| r.set);
    if let Some(t) = t {
        Ok(t)
    } else {
        Ok(false)
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

pub async fn get_query(request: RequestBuilder) -> Result<String, AppError> {
    let request_body = Get::build_query(get::Variables);
    let response_body: Response<get::ResponseData> = request
        .json(&request_body)
        .send()
        .await
        .map_err(|_| AppError::AnyError(SERVER_ERROR.into()))?
        .json()
        .await
        .map_err(|_| AppError::AnyError(PARSER_ERROR.into()))?;
    let v = response_body.errors.map(|e| e);
    if let Some(v) = v {
        get_err(&v)?;
    }
    match response_body.data {
        Some(e) => Ok(e.get),
        None => Err(AppError::AnyError(SERVER_ERROR.into())),
    }
}
