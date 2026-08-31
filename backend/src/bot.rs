use async_trait::async_trait;

use crate::PluxerApi;

#[async_trait]
pub trait BackendBot {
    type Api: PluxerApi;

    async fn send_message(
        &self,
        channel_id: &<Self::Api as PluxerApi>::Id,
        content: &str,
    ) -> Result<<Self::Api as PluxerApi>::Message, <Self::Api as PluxerApi>::Error>;
}
