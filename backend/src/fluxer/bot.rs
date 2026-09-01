use async_trait::async_trait;
use fluxer_builders::MessagePayloadData;
use fluxer_core::{Channel, Error, Message};
use fluxer_rest::Rest;
use fluxer_types::{ApiChannel, ApiMessage, Routes, Snowflake};

use crate::{bot::BackendBot, embed::Embed, fluxer::FluxerApi};

#[async_trait]
impl BackendBot for Rest {
    type Api = FluxerApi;

    async fn send_message(
        &self,
        channel_id: &Snowflake,
        content: Option<String>,
        embed: Option<Embed>,
    ) -> Result<Message, Error> {
        let payload = MessagePayloadData {
            content: content,
            embeds: embed.map(Into::into).map(|it| vec![it]),
            ..Default::default()
        };

        let message: ApiMessage = self
            .post(
                &fluxer_types::Routes::channel_messages(&channel_id),
                Some(&payload),
            )
            .await?;

        return Ok(Message::from_api(&message));
    }

    async fn get_channel(&self, channel_id: &Snowflake) -> Result<Channel, Error> {
        let channel: ApiChannel = self.get(&Routes::channel(channel_id)).await?;

        return Ok(Channel::from_api(&channel));
    }
}
