extern crate rocket;
use crate::engines::SearchEngine;
use crate::models::search::{EngineResult, SearchResult};
use anyhow::Context;
use async_graphql::Result;
use serde_json::Value;

pub struct WikimediaEngine;

#[async_trait]
impl SearchEngine for WikimediaEngine {
    async fn search(&self, query: &str) -> Result<EngineResult, anyhow::Error> {
        let client = mwapi::Client::builder("https://en.wikipedia.org/w/api.php")
            .set_user_agent("Roxy Search v0.0.1")
            .build()
            .await?;

        let response: Value = client
            .get(&[
                ("action", "query"),
                ("list", "search"),
                ("srsearch", query),
                ("srprop", "snippet|titlescore"),
                ("format", "json"),
            ])
            .await?;

        let search_results = response["query"]["search"]
            .as_array()
            .context("Unexpected answer format")?;

        Ok(EngineResult {
            engine: "wikimedia".into(),
            results: search_results
                .iter()
                .map(|r| SearchResult {
                    title: r["title"].as_str().unwrap_or("").into(),
                    link: format!(
                        "https://en.wikipedia.org/wiki/{}",
                        urlencoding::encode(r["title"].as_str().unwrap_or(""))
                    ),
                    description: r["snippet"]
                        .as_str()
                        .unwrap_or("No description available")
                        .replace("<span class=\"searchmatch\">", "")
                        .replace("</span>", "")
                        .into(),
                })
                .collect(),
            count: search_results.len() as i32,
        })
    }
}
