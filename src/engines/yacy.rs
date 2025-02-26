// src/engines/yacy.rs
use crate::models::search::{EngineResult, SearchResult};
use anyhow::{anyhow, Context};
use async_graphql::Result;
use serde_json::Value;
use urlencoding::encode;
use crate::engines::SearchEngine;

pub struct YacyEngine;

#[async_trait]
impl SearchEngine for YacyEngine {
    async fn search(&self, query: &str) -> Result<EngineResult, anyhow::Error> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;

        let url = format!(
            "http://localhost:8090/yacysearch.json?query={}&count=100",
            encode(query)
        );

        let response: Value = client
            .get(&url)
            .send()
            .await?
            .json()
            .await
            .context("Failed to parse YaCy JSON response")?;

        let results = response["channels"][0]["items"]
            .as_array()
            .ok_or_else(|| anyhow!("Invalid YaCy response format"))?;

        Ok(EngineResult {
            engine: "yacy".into(),
            results: results
                .iter()
                .filter_map(|item| {
                    Some(SearchResult {
                        title: item["title"].as_str()?.to_string(),
                        link: item["link"].as_str()?.to_string(),
                        description: item["description"].as_str().unwrap_or_default().to_string(),
                    })
                })
                .collect(),
            count: results.len() as i32,
        })
    }
}
