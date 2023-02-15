mod storage;
use crate::storage::*;
use actix_web::{guard, web, web::Data, App, HttpRequest, HttpResponse, HttpServer, Result};
use async_graphql::*;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};

async fn index(schema: web::Data<DataSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
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
                .endpoint("http://127.0.0.1:8000")
                .subscription_endpoint("ws://127.0.0.1:8000")
                .finish(),
        ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(QueryRoot, Set, SubscriptionRoot)
        .data(Storage::default())
        .finish();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(index_ws),
            )
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
