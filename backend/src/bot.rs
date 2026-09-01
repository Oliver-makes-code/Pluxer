use async_trait::async_trait;

use crate::{PluxerApi, embed::Embed};

#[async_trait]
pub trait BackendBot: Send + Sync {
    type Api: PluxerApi;

    async fn send_message(
        &self,
        channel_id: &<Self::Api as PluxerApi>::Id,
        content: Option<String>,
        embed: Option<Embed>,
    ) -> Result<<Self::Api as PluxerApi>::Message, <Self::Api as PluxerApi>::Error>;

    async fn get_channel(
        &self,
        channel_id: &<Self::Api as PluxerApi>::Id,
    ) -> Result<<Self::Api as PluxerApi>::Channel, <Self::Api as PluxerApi>::Error>;
}
