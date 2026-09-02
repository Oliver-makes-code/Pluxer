use async_trait::async_trait;

use crate::{PluxerApi, embed::Embed};

#[async_trait]
pub trait BackendBot: Send + Sync {
    type Api: PluxerApi;

    async fn get_self_id(
        &self,
    ) -> Result<<Self::Api as PluxerApi>::Id, <Self::Api as PluxerApi>::Error>;

    async fn fetch_webhooks(
        &self,
        channel_id: &<Self::Api as PluxerApi>::Id,
    ) -> Result<Vec<<Self::Api as PluxerApi>::Webhook>, <Self::Api as PluxerApi>::Error>;

    async fn create_webhook(
        &self,
        channel_id: &<Self::Api as PluxerApi>::Id,
        name: &str,
    ) -> Result<<Self::Api as PluxerApi>::Webhook, <Self::Api as PluxerApi>::Error>;

    async fn send_message_webhook(
        &self,
        webhook: &<Self::Api as PluxerApi>::Webhook,
        content: Option<String>,
        embed: Option<Embed>,
        referenced_message: Option<&<Self::Api as PluxerApi>::Message>,
    ) -> Result<Option<<Self::Api as PluxerApi>::Message>, <Self::Api as PluxerApi>::Error>;

    async fn send_message(
        &self,
        channel_id: &<Self::Api as PluxerApi>::Id,
        content: Option<String>,
        embed: Option<Embed>,
        referenced_message: Option<&<Self::Api as PluxerApi>::Message>,
    ) -> Result<<Self::Api as PluxerApi>::Message, <Self::Api as PluxerApi>::Error>;

    async fn get_channel(
        &self,
        channel_id: &<Self::Api as PluxerApi>::Id,
    ) -> Result<<Self::Api as PluxerApi>::Channel, <Self::Api as PluxerApi>::Error>;
}
