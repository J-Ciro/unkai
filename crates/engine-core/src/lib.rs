// engine-core skeleton
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use dashmap::DashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub widgets: Vec<WidgetConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WidgetConfig {
    pub id: String,
    pub widget_type: String,
    #[serde(default)]
    pub update_interval_ms: Option<u64>,
}

pub type SharedState = Arc<DashMap<String, WidgetRuntime>>;

#[derive(Debug)]
pub struct WidgetRuntime {
    pub config: WidgetConfig,
}

pub fn init() -> Result<SharedState> {
    let state: SharedState = Arc::new(DashMap::new());
    Ok(state)
}
