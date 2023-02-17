use graphql_client::{GraphQLQuery, Response};
use reqwest::RequestBuilder;
use std::error::Error;
use wasm_bindgen::UnwrapThrowExt;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/schema/schema.graphql",
    query_path = "src/api/schema/set.graphql",
    response_derives = "Debug, Clone",
    variables_derives = "Debug"
)]
pub struct Set;

pub async fn set_mutation(request: RequestBuilder, data: String) -> Result<bool, Box<dyn Error>> {
    let request_body = Set::build_query(set::Variables { data });
    let response_body: Response<set::ResponseData> =
        request.json(&request_body).send().await?.json().await?;
    Ok(response_body.data.expect_throw("response data err").set)
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
    Ok(response_body.data.expect_throw("response data err").get)
}
