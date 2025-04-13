mod schema;

use actix_web::{
    App, HttpResponse, HttpServer, guard,
    web::{self, Data},
};
use async_graphql::http::GraphiQLSource;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use schema::context::{Context, Schema, create_schema};
use std::sync::Arc;

// TODO:
// 1. Get environment variables and commands setup
// 2. Figure out an "architecture" type to build this around
// 3. Design pattern look-ups
// 4. Basic endpoint setup for now to get data "displayed"
//  - Get Agent
//  - Get Status
//  - Register New Agent
//  - List Contract(s)
//  - Get Contract
//  - List Ships
//  - Get Ship

async fn index(schema: web::Data<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

// TODO: proper dev flag for this
async fn graphiql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("GraphiQL Playground");

    // Get env files
    let _ = dotenvy::dotenv().expect("Failed to load .env file");

    let account_token = std::env::var("ACCOUNT_TOKEN").expect("ACCOUNT_TOKEN expected to be set");

    let context = Arc::new(Context { account_token });
    let schema = Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .app_data(Data::new(context.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .to(graphiql_playground),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
