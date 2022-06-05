#[macro_use]
extern crate dotenv_codegen;
extern crate dotenv;

pub mod graphql;

use actix_web::{get, guard, web, web::Data, App, HttpResponse, HttpServer, Responder, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::*;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};

use graphql::{mutation::Mutation, query::Query, CarnivalSchema};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn handle_request(schema: web::Data<CarnivalSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn gql_playground() -> Result<HttpResponse> {
    let source = playground_source(
        GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
    );
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url: &str = dotenv!("DATABASE_URL");

    let connection = sea_orm::Database::connect(database_url)
        .await
        .expect("Unable to connect to DB.");
    Migrator::up(&connection, None)
        .await
        .expect("Unable to run migrations");

    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(connection)
        .finish();

    println!("Server running at: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(hello)
            .service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .to(handle_request),
            )
            .service(
                web::resource("/graphql")
                    .guard(guard::Get())
                    .to(gql_playground),
            )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
