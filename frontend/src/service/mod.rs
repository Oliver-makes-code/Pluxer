use std::sync::Arc;

use pluxer_database::sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

#[cfg(feature = "fluxer")]
mod fluxer;

#[derive(Serialize, Deserialize)]
#[serde(tag = "service")]
pub enum PluxerService {
    #[cfg(feature = "fluxer")]
    Fluxer {
        api_endpoint: Arc<str>,
        token: Arc<str>,
        instance_name: Arc<str>,
    },
    #[serde(other)]
    Disabled,
}

impl PluxerService {
    pub async fn start(self, database: DatabaseConnection) -> anyhow::Result<()> {
        match self {
            Self::Disabled => {}

            #[cfg(feature = "fluxer")]
            Self::Fluxer {
                api_endpoint,
                token,
                instance_name,
            } => {
                fluxer::run(api_endpoint, &token, instance_name, database).await?;
            }
        }

        return Ok(());
    }
}
