mod duckduckgo;
mod wikimedia;

use crate::models::search::EngineResult;
use async_trait::async_trait;
use once_cell::sync::Lazy;
use std::collections::HashMap;

#[async_trait]
pub trait SearchEngine: Send + Sync {
    async fn search(&self, query: &str) -> Result<EngineResult, anyhow::Error>;
}

static ENGINES: Lazy<HashMap<&'static str, &'static dyn SearchEngine>> = Lazy::new(|| {
    let mut engines = HashMap::new();
    engines.insert(
        "wikimedia",
        &wikimedia::WikimediaEngine as &dyn SearchEngine,
    );
    engines.insert("duckduckgo", &duckduckgo::DuckDuckGoEngine);
    engines
});

pub struct EngineManager;

impl EngineManager {
    pub async fn search(engine: &str, query: &str) -> Result<EngineResult, anyhow::Error> {
        ENGINES
            .get(engine)
            .ok_or_else(|| anyhow::anyhow!("Engine '{}' not found", engine))?
            .search(query)
            .await
    }
}
