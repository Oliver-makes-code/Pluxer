use async_trait::async_trait;
use fluxer_core::{
    Channel,
    Error::{self, WebhookTokenRequired},
    Message, Webhook,
};
use fluxer_rest::Rest;
use fluxer_types::{
    ApiChannel, ApiEmbed, ApiMessage, ApiMessageReference, ApiUser, ApiWebhook, Routes, Snowflake,
};
use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};

use crate::{
    bot::{BackendBot, FileUpload},
    embed::Embed,
    fluxer::FluxerApi,
    message::ReferencedMessageKind,
};

#[derive(Serialize, Deserialize)]
struct AllowedMentions {
    pub replied_user: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessagePayloadData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<ApiEmbed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentPayload>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<ApiMessageReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentPayload {
    pub id: u32,
    pub filename: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u32>,
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

fn message_payload<T: Serialize>(
    content: Option<&str>,
    embed: Option<Embed>,
    referenced_message: Option<(ReferencedMessageKind, &Message)>,
    file_uploads: &[FileUpload],
    map_payload: impl FnOnce(MessagePayloadDataExtension) -> T,
) -> Form {
    let mut attachments = vec![];

    for (id, upload) in file_uploads.iter().enumerate() {
        attachments.push(AttachmentPayload {
            id: id as u32,
            filename: upload.file_name.clone(),
            description: None,
            flags: Some(
                0 | (upload.is_spoiler.then_some(8).unwrap_or(0))
                    | (upload.is_nsfw.then_some(32).unwrap_or(0)),
            ),
        });
    }

    let mut flags = 0;

    if let Some(referenced) = &referenced_message {
        if referenced.0 == ReferencedMessageKind::Forward {
            flags |= 1;
        }
    }

    let message_payload = map_payload(MessagePayloadDataExtension {
        message: MessagePayloadData {
            content: content.map(Into::into),
            embeds: embed.map(Into::into).map(|it| vec![it]),
            message_reference: referenced_message.map(|it| ApiMessageReference {
                channel_id: it.1.channel_id.clone(),
                message_id: it.1.id.clone(),
                guild_id: it.1.guild_id.clone(),
                kind: Some(it.0 as u8),
            }),
            attachments: Some(attachments),
            flags: Some(flags),
            ..Default::default()
        },
        allowed_mentions: Some(AllowedMentions {
            replied_user: false,
        }),
    });

    let json_str = serde_json::to_string(&message_payload).unwrap();

    let mut form = Form::new();

    form = form.part("payload_json", Part::text(json_str));

    for (i, file) in file_uploads.iter().enumerate() {
        let part = Part::bytes(file.data.clone())
            .file_name(file.file_name.to_string())
            .mime_str("application/octet-stream")
            .expect("valid MIME type");

        form = form.part(format!("files[{i}]"), part);
    }

    return form;
}

#[async_trait]
impl BackendBot for Rest {
    type Api = FluxerApi;

    async fn get_self_id(&self) -> Result<Snowflake, Error> {
        let user = self.get::<ApiUser>(Routes::current_user()).await?;

        return Ok(user.id);
    }

    async fn fetch_webhooks(&self, channel_id: &Snowflake) -> Result<Vec<Webhook>, Error> {
        let webhooks = self
            .get::<Vec<ApiWebhook>>(&Routes::channel_webhooks(channel_id))
            .await?;

        return Ok(webhooks.iter().map(Webhook::from_api).collect());
    }

    async fn create_webhook(&self, channel_id: &Snowflake, name: &str) -> Result<Webhook, Error> {
        let webhook = self
            .post::<ApiWebhook>(
                &Routes::channel_webhooks(channel_id),
                Some(&WebhookCreatePayloadData {
                    name: name.to_string(),
                    avatar_url: None,
                }),
            )
            .await?;

        return Ok(Webhook::from_api(&webhook));
    }

    async fn send_message_webhook(
        &self,
        webhook: &Webhook,
        content: Option<&str>,
        embed: Option<Embed>,
        referenced_message: Option<(ReferencedMessageKind, &Message)>,
        file_uploads: &[FileUpload],
        username: &str,
        avatar_url: Option<&str>,
    ) -> Result<Message, Error> {
        let payload = message_payload(
            content,
            embed,
            referenced_message,
            file_uploads,
            |message| WebhookPayloadData {
                message,
                username: Some(username.into()),
                avatar_url: avatar_url.map(ToString::to_string),
            },
        );

        let token = webhook.token.as_deref().ok_or(WebhookTokenRequired)?;

        let route = Routes::webhook_execute(&webhook.id, token) + "?wait=true";

        let value: ApiMessage = self.post_multipart(&route, payload).await?;

        return Ok(Message::from_api(&value));
    }

    async fn send_message(
        &self,
        channel_id: &Snowflake,
        content: Option<&str>,
        embed: Option<Embed>,
        referenced_message: Option<(ReferencedMessageKind, &Message)>,
        file_uploads: &[FileUpload],
    ) -> Result<Message, Error> {
        let payload = message_payload(
            content,
            embed,
            referenced_message,
            file_uploads,
            std::convert::identity,
        );

        let message: ApiMessage = self
            .post_multipart(
                &fluxer_types::Routes::channel_messages(&channel_id),
                payload,
            )
            .await?;

        return Ok(Message::from_api(&message));
    }

    async fn get_channel(&self, channel_id: &Snowflake) -> Result<Channel, Error> {
        let channel: ApiChannel = self.get(&Routes::channel(channel_id)).await?;

        return Ok(Channel::from_api(&channel));
    }

    async fn delete_message(
        &self,
        channel_id: &Snowflake,
        message_id: &Snowflake,
    ) -> Result<(), Error> {
        self.delete_route(&Routes::channel_message(channel_id, message_id))
            .await?;

        return Ok(());
    }

    async fn fetch_message(
        &self,
        channel_id: &Snowflake,
        message_id: &Snowflake,
    ) -> Result<Message, Error> {
        let message = self
            .get::<ApiMessage>(&Routes::channel_message(channel_id, message_id))
            .await?;

        return Ok(Message::from_api(&message));
    }

    async fn edit_message_webhook(
        &self,
        webhook: &Webhook,
        message_id: &Snowflake,
        content: &str,
    ) -> Result<(), Error> {
        let data = MessagePayloadData {
            content: Some(content.to_string()),
            ..Default::default()
        };

        self.patch::<ApiMessage>(
            &format!(
                "/webhooks/{}/{}/messages/{}",
                webhook.id,
                webhook.token.as_ref().unwrap(),
                message_id
            ),
            Some(&data),
        )
        .await?;

        return Ok(());
    }
}
