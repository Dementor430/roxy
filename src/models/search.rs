use async_graphql::{InputObject, SimpleObject};

#[derive(InputObject)]
pub struct SearchRequest {
    pub(crate) query: String,
    pub(crate) engines: Vec<String>,
}

#[derive(SimpleObject)]
pub struct SearchResult {
    pub(crate) title: String,
    pub(crate) link: String,
    pub(crate) description: String,
}

#[derive(SimpleObject)]
pub struct EngineResult {
    pub(crate) engine: String,
    pub(crate) results: Vec<SearchResult>,
    pub(crate) count: i32,
}

#[derive(SimpleObject)]
pub struct SearchResponse {
    pub(crate) query: String,
    pub(crate) timestamp: String,
    pub(crate) results: Vec<EngineResult>,
}