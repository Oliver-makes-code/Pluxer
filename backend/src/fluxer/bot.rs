use async_trait::async_trait;
use fluxer_builders::MessagePayloadData;
use fluxer_core::{Channel, Error, Message, Webhook};
use fluxer_rest::{Rest, RestError};
use fluxer_types::{ApiChannel, ApiMessage, ApiMessageReference, ApiUser, ApiWebhook, Routes, Snowflake};
use serde::{Deserialize, Serialize};

use crate::{bot::BackendBot, embed::Embed, fluxer::FluxerApi};

#[derive(Serialize, Deserialize)]
struct AllowedMentions {
    pub replied_user: bool,
}

#[derive(Serialize, Deserialize)]
struct MessagePayloadDataExtension {
    #[serde(flatten)]
    pub message: MessagePayloadData,
    pub allowed_mentions: Option<AllowedMentions>,
}

// why does not have in library?????
#[derive(Serialize, Deserialize)]
struct WebhookPayloadData {
    #[serde(flatten)]
    pub message: MessagePayloadDataExtension,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct WebhookCreatePayloadData {
    pub name: String,
    pub avatar_url: Option<String>,
}

fn message_payload(
    content: Option<String>,
    embed: Option<Embed>,
    referenced_message: Option<&Message>,
) -> MessagePayloadDataExtension {
    return MessagePayloadDataExtension {
        message: MessagePayloadData {
            content: content,
            embeds: embed.map(Into::into).map(|it| vec![it]),
            message_reference: referenced_message.map(|it| ApiMessageReference {
                channel_id: it.channel_id.clone(),
                message_id: it.id.clone(),
                guild_id: it.guild_id.clone(),
                kind: None,
            }),
            ..Default::default()
        },
        allowed_mentions: Some(AllowedMentions {
            replied_user: false,
        }),
    };
}

#[async_trait]
impl BackendBot for Rest {
    type Api = FluxerApi;

    async fn get_self_id(&self) -> Result<Snowflake, Error> {
        let user = self.get::<ApiUser>(Routes::current_user()).await?;

        return Ok(user.id);
    }

    async fn fetch_webhooks(&self, channel_id: &Snowflake) -> Result<Vec<Webhook>, Error> {
        let webhooks = self.get::<Vec<ApiWebhook>>(&Routes::channel_webhooks(channel_id)).await?;

        return Ok(webhooks.iter().map(Webhook::from_api).collect());
    }

    async fn create_webhook(&self, channel_id: &Snowflake, name: &str) -> Result<Webhook, Error> {
        let webhook = self.post::<ApiWebhook>(&Routes::channel_webhooks(channel_id), Some(&WebhookCreatePayloadData {
            name: name.to_string(),
            avatar_url: None
        })).await?;

        return Ok(Webhook::from_api(&webhook));
    } 

    async fn send_message_webhook(
        &self,
        webhook: &Webhook,
        content: Option<String>,
        embed: Option<Embed>,
        referenced_message: Option<&Message>,
    ) -> Result<Option<Message>, Error> {
        let payload = WebhookPayloadData {
            message: message_payload(content, embed, referenced_message),
            username: None,
            avatar_url: None,
        };

        let value = webhook
            .send(
                self,
                &serde_json::to_value(&payload).map_err(RestError::Json)?,
                true,
            )
            .await?;

        return Ok(value.as_ref().map(Message::from_api));
    }

    async fn send_message(
        &self,
        channel_id: &Snowflake,
        content: Option<String>,
        embed: Option<Embed>,
        referenced_message: Option<&Message>,
    ) -> Result<Message, Error> {
        let payload = message_payload(content, embed, referenced_message);

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
