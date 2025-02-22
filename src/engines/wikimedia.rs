extern crate rocket;
use crate::models::search::{EngineResult, SearchResult};
use anyhow::Context;
use async_graphql::Result;
use serde_json::Value;

pub async fn wikimedia_search(query: &str) -> Result<EngineResult, anyhow::Error> {
    let client = mwapi::Client::builder("https://en.wikipedia.org/w/api.php")
        .set_user_agent("SearchService/1.0")
        .build()
        .await?;

    let response: Value = client.get(&[
        ("action", "query"),
        ("list", "search"),
        ("srsearch", query),
        ("srprop", "snippet|titlescore"),
        ("format", "json")
    ]).await?;

    let search_results = response["query"]["search"]
        .as_array()
        .context("Unerwartetes Antwortformat")?;

    Ok(EngineResult {
        engine: "wikimedia".into(),
        results: search_results.iter().map(|r| SearchResult {
            title: r["title"].as_str().unwrap_or("").into(),
            link: format!("https://en.wikipedia.org/wiki/{}",
                          urlencoding::encode(r["title"].as_str().unwrap_or(""))),
            description: r["snippet"].as_str()
                .unwrap_or("Keine Beschreibung verfügbar")
                .replace("<span class=\"searchmatch\">", "")
                .replace("</span>", "")
                .into(),
        }).collect(),
        count: search_results.len() as i32,
    })
}
