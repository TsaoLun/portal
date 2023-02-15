use graphql_client::GraphQLQuery;
use gloo::storage::{LocalStorage, Storage};
use super::post::*;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/schema/schema.graphql",
    query_path = "src/api/schema/login.graphql",
    response_derives = "Debug, Clone",
    variables_derives = "Deserialize, Debug"
)]
pub struct Login;

pub async fn login(username: String, password: String) -> Result<login::ResponseData,String> {
    let body = Login::build_query(login::Variables{
        username,
        password,
    });
    
    let resp = post::<login::Variables, login::ResponseData>(body).await;

    match resp {
        Ok(data) => {
            // TODO: Handle unwrap yeah?
            LocalStorage::set("token", data.login.token.clone()).unwrap();
            Ok(data)
        },
        Err(err) => Err(err),
    }
}