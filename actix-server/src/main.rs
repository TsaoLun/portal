mod middlewares;
mod storage;
use std::env;

use crate::storage::*;
use actix_cors::Cors;
use actix_web::{guard, web, web::Data, App, HttpRequest, HttpResponse, HttpServer, Result};
use async_graphql::*;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use dotenv::dotenv;
use lazy_static::lazy_static;
use middlewares::get_token_from_headers;

lazy_static! {
    pub static ref SERVER_URL: String = init_url();
}

fn init_url() -> String {
    env::var("SERVER_URL")
        .unwrap_or("0.0.0.0:8008".to_string())
        .replace("http://", "")
}

async fn index(
    schema: web::Data<DataSchema>,
    req: HttpRequest,
    gql_req: GraphQLRequest,
) -> GraphQLResponse {
    let token = get_token_from_headers(req.headers());
    let mut request = gql_req.into_inner();
    if let Some(token) = token {
        request = request.data(token);
    }
    schema.execute(request).await.into()
}

async fn index_ws(
    schema: web::Data<DataSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    GraphQLSubscription::new(Schema::clone(&*schema)).start(&req, payload)
}

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            http::GraphiQLSource::build()
                .endpoint(&format!("http://{}/graphql/", *SERVER_URL))
                .subscription_endpoint(&format!("ws://{}/graphql/", *SERVER_URL))
                .finish(),
        ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(Storage::default())
        .finish();
    println!("\n> server run at http://{}.", *SERVER_URL);
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allowed_methods(vec!["POST"])
                    .allowed_origin(&format!("http://{}", *SERVER_URL))
                    .allowed_origin("http://127.0.0.1:8008")
                    .allowed_origin("http://127.0.0.1:8080")
                    .allowed_origin("http://0.0.0.0:8080"),
            )
            .service(web::resource("/graphql/").guard(guard::Post()).to(index))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(index_ws),
            )
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind(SERVER_URL.to_string())?
    .run()
    .await
}
