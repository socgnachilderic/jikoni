use std::sync::Arc;

use actix_web::{middleware, web, App, HttpServer};

use jikoni::configs::{get_cors, Env};
use jikoni::graphql::{build_schema, get_graphql_endpoints};
use jikoni::{get_file, AppState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    let env = Env::initialize_environnement();
    let conn = sea_orm::Database::connect(env.db_url.as_ref())
        .await
        .unwrap();
    let server_url = &env.get_server_url();
    let app_state = AppState {
        conn: Arc::new(conn),
        env: Arc::new(env),
    };
    let schema = build_schema(app_state.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .app_data(web::Data::new(schema.clone()))
            .wrap(get_cors())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .configure(get_graphql_endpoints)
            .service(get_file)
    })
    .bind(server_url)?
    .run()
    .await
}
