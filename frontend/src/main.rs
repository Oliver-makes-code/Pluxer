#![allow(incomplete_features)]
#![feature(checked_type_aliases, duration_constructors)]

use std::{fs, sync::Arc, time::Duration};

use pluxer_database::handler::DatabaseHandler;
use serde::{Deserialize, Serialize};
use tokio::time::interval;

use crate::service::PluxerService;

mod bot;
mod service;

#[derive(Serialize, Deserialize)]
struct PluxerConfig {
    #[serde(default)]
    pub services: Box<[PluxerService]>,
    pub database_url: Arc<str>,
}

const CONFIG_PATH: &str = "./pluxer_config.json";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_string = fs::read_to_string(CONFIG_PATH)?;

    let config = serde_json::from_str::<PluxerConfig>(&config_string)?;

    let database = DatabaseHandler::new(&config.database_url).await?;

    let db = database.clone();

    tokio::spawn(async move {
        let mut interval = interval(Duration::from_hours(1));

        loop {
            interval.tick().await;

            if let Err(err) = db.clean_up_messages().await {
                pawkit_logger::log!(error, "Failed to delete expired messages: {err}");
            }
        }
    });

    let mut services = tokio::task::JoinSet::new();

    for service in config.services {
        services.spawn(service.start(database.clone()));
    }

    while let Some(result) = services.join_next().await {
        result??;
    }

    return Ok(());
}
