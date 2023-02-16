use graphql_client::GraphQLQuery;
use gloo::{storage::{LocalStorage, Storage}, console::*};
use self::set::Variables;

use super::{post::*, GraphQLError};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/schema/schema.graphql",
    query_path = "src/api/schema/set.graphql",
    response_derives = "Debug, Clone",
    variables_derives = "Debug"
)]
pub struct Set;

pub async fn set(data: String) -> Result<set::ResponseData, String> {

    let body = Set::build_query(set::Variables{
        data,
    });


    let resp = post::<set::Variables, set::ResponseData>(body).await;
    match resp {
        Ok(data) => {
            Ok(data)
        },
        Err(err) => Err(err),
    }
}