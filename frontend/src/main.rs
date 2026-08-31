use std::fs;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};

use crate::service::PluxerService;

mod bot;
mod service;

#[derive(Serialize, Deserialize)]
struct PluxerConfig {
    #[serde(default)]
    pub services: Box<[PluxerService]>,
}

const CONFIG_PATH: &str = "./pluxer_config.json";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_string = fs::read_to_string(CONFIG_PATH)?;

    let config = serde_json::from_str::<PluxerConfig>(&config_string)?;

    let mut services = tokio::task::JoinSet::new();

    for service in config.services {
        services.spawn(service.start());
    }

    while let Some(result) = services.join_next().await {
        result??;
    }

    return Ok(());
}
