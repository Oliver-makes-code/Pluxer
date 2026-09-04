use async_trait::async_trait;
use fluxer_builders::{AttachmentPayload, MessagePayloadData};
use fluxer_core::{
    Channel,
    Error::{self, WebhookTokenRequired},
    Message, Webhook,
};
use fluxer_rest::Rest;
use fluxer_types::{
    ApiChannel, ApiMessage, ApiMessageReference, ApiUser, ApiWebhook, Routes, Snowflake,
};
use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};

use crate::{
    bot::{BackendBot, FileUpload},
    embed::Embed,
    fluxer::FluxerApi,
};

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

fn message_payload<T: Serialize>(
    content: Option<String>,
    embed: Option<Embed>,
    referenced_message: Option<&Message>,
    file_uploads: &[FileUpload],
    map_payload: impl FnOnce(MessagePayloadDataExtension) -> T,
) -> Form {
    let mut attachments = vec![];

    for (id, upload) in file_uploads.iter().enumerate() {
        attachments.push(AttachmentPayload {
            id: id as u32,
            filename: upload.file_name.clone(),
            description: None,
        });
    }

    let message_payload = map_payload(MessagePayloadDataExtension {
        message: MessagePayloadData {
            content: content,
            embeds: embed.map(Into::into).map(|it| vec![it]),
            message_reference: referenced_message.map(|it| ApiMessageReference {
                channel_id: it.channel_id.clone(),
                message_id: it.id.clone(),
                guild_id: it.guild_id.clone(),
                kind: None,
            }),
            attachments: Some(attachments),
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
        content: Option<String>,
        embed: Option<Embed>,
        referenced_message: Option<&Message>,
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
        content: Option<String>,
        embed: Option<Embed>,
        referenced_message: Option<&Message>,
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
}
