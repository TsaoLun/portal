use graphql_client::{GraphQLQuery, Response};
use reqwest::RequestBuilder;
use std::error::Error;

use crate::utils::api_response::{get_err, ErrData, ResData};

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
) -> Result<Option<ErrData>, Box<dyn Error>> {
    let request_body = Set::build_query(set::Variables { data });
    let response_body: Response<set::ResponseData> =
        request.json(&request_body).send().await?.json().await?;
    if response_body.errors.is_some() {
        let data = get_err(response_body);
        return Ok(Some(ErrData {
            message: data.0,
            code: data.1,
        }));
    }
    if response_body.data.and_then(|r| Some(r.set)) == Some(true) {
        Ok(None)
    } else {
        Ok(Some(ErrData {
            message: "服务器异常".into(),
            code: None,
        }))
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

pub async fn get_query(request: RequestBuilder) -> Result<ResData<String>, Box<dyn Error>> {
    let request_body = Get::build_query(get::Variables);
    let response_body: Response<get::ResponseData> =
        request.json(&request_body).send().await?.json().await?;
    if response_body.errors.is_some() {
        let data = get_err(response_body);
        return Ok(ResData {
            data: None,
            err: Some(ErrData {
                message: data.0,
                code: data.1,
            }),
        });
    }
    // Ok(response_body.data.expect_throw("response data err").get)
    match response_body.data {
        Some(e) => Ok(ResData {
            data: Some(e.get),
            err: None,
        }),
        None => Ok(ResData {
            err: Some(ErrData {
                message: "服务器异常".into(),
                code: None,
            }),
            data: None,
        }),
    }
}
