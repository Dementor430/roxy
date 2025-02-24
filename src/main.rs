#[macro_use]
extern crate rocket;
mod engines;
mod models;

use async_graphql::http::GraphiQLSource;
use async_graphql::{Context, EmptyMutation, EmptySubscription, Error, Object, Schema};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use rocket::State;

use crate::models::search::{SearchRequest, SearchResponse};
use chrono::Utc;
use fern::colors::{Color, ColoredLevelConfig};
use fern::{DateBased, Dispatch};
use log::LevelFilter;
use rocket::futures::future::join_all;
use rocket::response::content::RawHtml;

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn search(
        &self,
        _ctx: &Context<'_>,
        request: SearchRequest,
    ) -> Result<SearchResponse, Error> {
        let timestamp = Utc::now().to_rfc3339();
        let futures = request
            .engines
            .iter()
            .map(|engine| engines::EngineManager::search(engine, &request.query));

        let results = join_all(futures)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(SearchResponse {
            query: request.query,
            timestamp,
            results,
        })
    }
}

type SchemaType = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[get("/")]
fn graphiql() -> RawHtml<String> {
    RawHtml(
        GraphiQLSource::build()
            .endpoint("http://localhost:8000/graphql")
            .finish(),
    )
}

#[get("/graphql?<query..>")]
async fn graphql_query(schema: &State<SchemaType>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema.inner()).await
}

#[post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_request(schema: &State<SchemaType>, request: GraphQLRequest) -> GraphQLResponse {
    request.execute(schema.inner()).await
}

fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
    // Configure Fern logger
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red);

    Dispatch::new()
        .chain(DateBased::new("logs/app-", "%Y-%m-%d.log"))
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] {} {}: {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        .level(LevelFilter::Info)
        .level_for("rocket", LevelFilter::Warn)
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}

#[launch]
fn rocket() -> _ {
    setup_logging().expect("Logging provider Fern was not initialized.");
    rocket::build()
        .manage(Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish())
        .mount("/", routes![graphiql, graphql_query, graphql_request])
}
