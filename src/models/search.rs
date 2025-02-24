use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

#[derive(Debug, InputObject, SimpleObject, Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    pub engines: Vec<String>,
}

#[derive(Debug, SimpleObject, Serialize, Deserialize)]
pub struct SearchResponse {
    pub query: String,
    pub timestamp: String,
    pub results: Vec<EngineResult>,
}

#[derive(Debug, SimpleObject, Serialize, Deserialize)]
pub struct EngineResult {
    pub engine: String,
    pub count: i32,
    pub results: Vec<SearchResult>,
}

#[derive(Debug, SimpleObject, Serialize, Deserialize, Clone)]
pub struct SearchResult {
    pub title: String,
    pub link: String,
    pub description: String,
}
