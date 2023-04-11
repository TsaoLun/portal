use graphql_client::GraphQLQuery;
use serde::Serialize;

#[derive(GraphQLQuery, Debug, Serialize)]
#[graphql(
    schema_path = "src/api/schema/schema.graphql",
    query_path = "src/api/schema/upload.graphql",
    response_derives = "Debug, Clone",
    variables_derives = "Debug"
)]
pub struct Upload;
