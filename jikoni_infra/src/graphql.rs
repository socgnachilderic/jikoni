use actix_web::{guard, web, HttpResponse, Result};
use async_graphql::http::{graphiql_source, playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, MergedObject, Object, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::recipes::recipe_schemas::{RecipeMutation, RecipeQuery};
use crate::AppState;

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(MergedObject, Default)]
pub struct QueryRoot(AppQuery, RecipeQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(RecipeMutation);

#[derive(Default)]
struct AppQuery;

#[Object]
impl AppQuery {
    async fn hello(&self) -> String {
        "hello world".to_string()
    }

    async fn api_version(&self) -> String {
        "v1".to_string()
    }
}

pub fn get_graphql_endpoints(config: &mut web::ServiceConfig) {
    config
        .service(
            web::resource("/graphql")
                .route(web::get().to(build_graphql_controller))
                .route(web::post().to(build_graphql_controller)),
        )
        .service(
            web::resource("/playground")
                .guard(guard::Get())
                .to(build_playground_controller),
        )
        .service(
            web::resource("/graphiql")
                .guard(guard::Get())
                .to(build_graphiql_controller),
        );
}

pub fn build_schema(app_state: AppState) -> AppSchema {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(app_state)
    .finish()
}

async fn build_graphql_controller(
    schema: web::Data<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn build_playground_controller() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/graphql"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

async fn build_graphiql_controller() -> Result<HttpResponse> {
    let source = graphiql_source("/graphql", None);
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}
