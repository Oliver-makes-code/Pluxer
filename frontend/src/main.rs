#![allow(incomplete_features)]
#![feature(checked_type_aliases)]

use std::{fs, sync::Arc, time::Duration};

use pluxer_database::{
    connect,
    entities::message,
    sea_orm::{ColumnTrait, EntityTrait, QueryFilter, sqlx::types::chrono::Utc},
};
use serde::{Deserialize, Serialize};
use tokio::time::interval;

use crate::service::PluxerService;

mod bot;
mod database;
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

    let database = connect(&config.database_url).await.unwrap();

    let db = database.clone();

    tokio::spawn(async move {
        let mut interval = interval(Duration::from_hours(1));

        loop {
            interval.tick().await;

            if let Err(err) = message::fluxer::Entity::delete_many()
                .filter(message::fluxer::Column::ExpiresAt.lte(Utc::now()))
                .exec(&db)
                .await
            {
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
