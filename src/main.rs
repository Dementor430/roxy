#[macro_use]
extern crate rocket;
mod engines;
mod models;

use async_graphql::{Context, EmptyMutation, EmptySubscription, Error, Object, Schema};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use rocket::State;

use chrono::Utc;


struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn search(&self, _ctx: &Context<'_>, request: models::search::SearchRequest) -> Result<models::search::SearchResponse, Error> {
        let mut results = Vec::new();

        for engine in &request.engines {
            let result = match engine.as_str() {
                "wikimedia" => engines::wikimedia::wikimedia_search(&request.query).await,
                "duckduckgo" => engines::duckduckgo::duckduckgo_search(&request.query).await,
                _ => continue  // Alte Engines bleiben simuliert
            };

            match result {
                Ok(res) => results.push(res),
                Err(e) => tracing::error!("Engine {} failed: {}", engine, e)
            }
        }

        Ok(models::search::SearchResponse {
            query: request.query,
            timestamp: Utc::now().to_rfc3339(),
            results,
        })
    }
}


type SchemaType = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[rocket::get("/graphql?<query..>")]
async fn graphql_query(
    schema: &State<SchemaType>,
    query: GraphQLQuery,
) -> GraphQLResponse {
    query.execute(schema.inner()).await
}

#[rocket::post("/graphql", data = "<request>")]
async fn graphql_request(
    schema: &State<SchemaType>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    request.execute(schema.inner()).await
}

#[launch]
fn rocket() -> _ {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .finish();

    rocket::build()
        .manage(schema)
        .mount("/", routes![
            graphql_query,
            graphql_request,
        ])
}
