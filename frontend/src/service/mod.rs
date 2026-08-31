#[cfg(feature = "fluxer")]
use std::sync::Arc;

#[cfg(feature = "fluxer")]
use pluxer_backend::fluxer::fluxer_core::Error;
use serde::{Deserialize, Serialize};
use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum PluxerServiceError {
    #[cfg(feature = "fluxer")]
    #[error(transparent)]
    Fluxer(#[from] Error),
}

impl PluxerService {
    pub async fn start(self) -> Result<(), PluxerServiceError> {
        match self {
            Self::Disabled => {}

            #[cfg(feature = "fluxer")]
            Self::Fluxer {
                api_endpoint,
                token,
                instance_name,
            } => {
                fluxer::run(&api_endpoint, &token, instance_name).await?;
            }
        }

        return Ok(());
    }
}
