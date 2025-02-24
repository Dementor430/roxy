extern crate rocket;

use std::time::Duration;
use crate::models::search::{EngineResult, SearchResult};
use anyhow::anyhow;
use async_graphql::Result;
use lazy_static::lazy_static;
use regex::Regex;
use url::Url;
use urlencoding::decode;

lazy_static! {
    static ref CLEAN_REGEX: Regex = Regex::new(r"(<a\b[^>]*>|</a>|&#\d+;|&[a-z]+;)").unwrap();
}

use crate::engines::SearchEngine;
pub use scraper::{Html, Selector};
use urlencoding::encode;

pub struct DuckDuckGoEngine;

#[async_trait]
impl SearchEngine for DuckDuckGoEngine {
    async fn search(&self, query: &str) -> Result<EngineResult, anyhow::Error> {
        let client = reqwest::Client::builder()
            .pool_idle_timeout(Duration::from_secs(120))
            .pool_max_idle_per_host(20)
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
            .http2_keep_alive_interval(Duration::from_secs(15))
            .build()
            .unwrap();

        let url = format!("https://duckduckgo.com/html/?q={}", encode(query));
        let response = client.get(&url).send().await?;
        let body = response.text().await?;
        let document = Html::parse_document(&body);

        let result_selector = Selector::parse(".web-result").unwrap();
        let title_selector = Selector::parse(".result__a").unwrap();
        let url_selector = Selector::parse(".result__url").unwrap();
        let snippet_selector = Selector::parse(".result__snippet").unwrap();

        let results: Vec<SearchResult> = document
            .select(&result_selector)
            .filter_map(|result| {
                let title = result
                    .select(&title_selector)
                    .next()?
                    .text()
                    .collect::<String>();
                let raw_url = result.select(&url_selector).next()?.value().attr("href")?;

                let clean_url = {
                    let parsed_url = Url::parse(&format!("https://duckduckgo.com{}", raw_url))
                        .map_err(|e| anyhow!("URL-Parsing-Error: {}", e))
                        .ok()?;

                    parsed_url
                        .query_pairs()
                        .find(|(k, _)| k == "uddg")
                        .and_then(|(_, v)| decode(&v.into_owned()).map(|s| s.into_owned()).ok())
                        .unwrap_or_else(|| raw_url.to_string())
                };

                let snippet = result
                    .select(&snippet_selector)
                    .next()?
                    .text()
                    .collect::<String>();

                Some(SearchResult {
                    title: title.trim().to_string(),
                    link: clean_url.to_string(),
                    description: snippet.trim().to_string(),
                })
            })
            .collect();

        let count = results.len() as i32; // Ownership-Fix

        Ok(EngineResult {
            engine: "duckduckgo".into(),
            results,
            count,
        })
    }
}
