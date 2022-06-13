#[macro_use]
extern crate dotenv_codegen;
extern crate dotenv;

pub mod auth;
pub mod graphql;
pub mod logger;

use actix_web::Responder;
use actix_web::{get, guard, web, web::Data, App, HttpRequest, HttpResponse, HttpServer, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::*;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use dotenv::dotenv;
use log::debug;
use logger::setup_logger;
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;

use crate::auth::{authenticate, check_api_key};
use graphql::{mutation::Mutation, query::Query, CarnivalSchema};

#[get("/")]
async fn home_page() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(format!(
        "Welcome to the <a href=\"/graphql\">Carnival GraphQL API</a>"
    ))
}

async fn handle_request(
    schema: Data<CarnivalSchema>,
    db: Data<DatabaseConnection>,
    req: HttpRequest,
    gql_request: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = gql_request.into_inner();
    let mut log_message = format!(
        "GraphQL request: {}",
        request
            .operation_name
            .clone()
            .unwrap_or_else(|| "query".to_string())
    );
    let user = authenticate(req.headers().clone(), &db).await;
    request = request.data(user.clone());
    if let Some(user) = user {
        log_message.push_str(&format!("\nAuthorized User: {}", user.username));
    }
    let api_info = check_api_key(req.headers().clone(), &db).await;
    request = request.data(api_info.clone());
    if let Some(game) = api_info {
        log_message.push_str(&format!("\nAuthorized Game: {}", game.title));
    }
    if req.headers().contains_key("x-replit-user-id") {
        let user_id = req
            .headers()
            .get("x-replit-user-id")
            .unwrap()
            .to_str()
            .unwrap()
            .parse::<i32>();
        if let Ok(user_id) = user_id {
            request = request.data(user_id);
            log_message.push_str(&format!("\nUser ID: {}", user_id));
        }
    }
    debug!("{}", log_message);
    schema.execute(request).await.into()
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

    setup_logger().expect("Unable to setup logger");

    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(connection.clone())
        .finish();

    println!("Server running at: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .app_data(Data::new(connection.clone()))
            .service(home_page)
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
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
