use std::{borrow::Cow, fs, path::Path, sync::Arc, time::Duration};

use moka::{future::Cache, policy::EvictionPolicy};
use pluxer_backend::{
    bot::{BackendBot, FileUpload},
    embed::Embed,
    message::BackendMessage,
    user::BackendUser,
    webhook::BackendWebhook,
};
use pluxer_database::{model::member::MemberModel, sea_orm::DatabaseConnection};
use thiserror::Error;
use tokio::sync::OnceCell;
use ulid::Ulid;

use crate::{
    bot::{
        command_parser::{CommandContext, CommandRoot, parse_command},
        commands::create_command_tree,
    },
    database::DatabaseExtension,
};

mod command_parser;
mod commands;

pub struct PluxerContext<A: DatabaseExtension> {
    pub bot: A::Bot,
    pub database_connection: DatabaseConnection,
    pub command_tree: CommandRoot<PluxerContext<A>>,
    pub webhook_cache: Cache<A::Id, A::Webhook>,
    pub _instance_name: Arc<str>,
    pub instance_url: Arc<str>,
    pub user_id: OnceCell<A::Id>,
}

impl<A: DatabaseExtension> CommandContext for PluxerContext<A> {
    type CommandData = A::Message;
}

#[derive(Debug, Error)]
#[error("The proxies should be deleted if the member doesn't exist")]
struct NoMemberToProxyError;

impl<A: DatabaseExtension> PluxerContext<A> {
    const PREFIXES: &[&str] = &["pl!", "pl?", "pl;", "pl/"];

    pub async fn new(
        bot: A::Bot,
        database: DatabaseConnection,
        instance_name: Arc<str>,
        instance_url: Arc<str>,
    ) -> anyhow::Result<Self> {
        return Ok(Self {
            bot,
            database_connection: database,
            command_tree: create_command_tree(),
            webhook_cache: Cache::builder()
                .max_capacity(50_000)
                .eviction_policy(EvictionPolicy::lru())
                .time_to_idle(Duration::from_hours(1))
                .build(),
            _instance_name: instance_name,
            instance_url,
            user_id: OnceCell::new(),
        });
    }

    async fn with_error_handler(
        &self,
        message: &A::Message,
        fut: impl Future<Output = anyhow::Result<()>>,
    ) -> anyhow::Result<()> {
        let Err(err) = fut.await else {
            return Ok(());
        };

        let error_id = Ulid::generate();

        let mut error_report = String::new();

        error_report += &format!("{}", err);

        let result = self.bot.send_message(message.channel_id(), None, Some(Embed {
            title: Some("Unexpected error!".into()),
            description: Some("Please report to our [GitHub Repository](https://github.com/Oliver-makes-code/Pluxer/issues), and include the error ID listed below.".into()),
            footer: Some(format!("Error ID: {}", error_id)),

            ..Default::default()
        }), Some(message), &[]).await;

        if let Err(err) = result {
            error_report += &format!("\n\n{}", err);
        };

        pawkit_logger::log!(error, "{}: {}", error_id, error_report);

        let path = Path::new("logs/error");

        fs::create_dir_all(path)?;

        let file_path = path.join(format!("{}.txt", error_id));

        fs::write(file_path, error_report)?;

        return Ok(());
    }

    async fn handle_command(
        &self,
        message: &A::Message,
        command_substring: &str,
    ) -> anyhow::Result<()> {
        return parse_command(command_substring, &self.command_tree, self, message).await;
    }

    async fn get_id(&self) -> Result<&A::Id, A::Error> {
        return self
            .user_id
            .get_or_try_init(async || self.bot.get_self_id().await)
            .await;
    }

    async fn fetch_webhook(&self, channel_id: &A::Id) -> anyhow::Result<A::Webhook> {
        if let Some(webhook) = self.webhook_cache.get(channel_id).await {
            return Ok(webhook);
        }

        for webhook in self.bot.fetch_webhooks(channel_id).await? {
            if webhook.owner().id() == self.get_id().await? {
                self.webhook_cache
                    .insert(channel_id.clone(), webhook.clone())
                    .await;

                return Ok(webhook);
            }
        }

        let webhook = self
            .bot
            .create_webhook(channel_id, "Pluxer Webhook")
            .await?;

        self.webhook_cache
            .insert(channel_id.clone(), webhook.clone())
            .await;

        return Ok(webhook);
    }

    async fn fetch(url: &str) -> Result<Vec<u8>, reqwest::Error> {
        let response = reqwest::get(url).await?;

        return Ok(response.bytes().await?.to_vec());
    }

    async fn resend_message(
        &self,
        message: &A::Message,
        member: &MemberModel,
        trimmed_message_content: &str,
    ) -> anyhow::Result<()> {
        let Some(system) = A::fetch_system_by_id(self, member.system_id).await? else {
            return Ok(());
        };

        let webhook = self.fetch_webhook(message.channel_id()).await?;

        let mut files = vec![];

        for attachment in message.attachments() {
            files.push(FileUpload {
                file_name: attachment.file_name,
                data: Self::fetch(&attachment.file_url).await?,
            });
        }

        let username = member.display_name.as_deref().unwrap_or(&member.name);

        let username = if let Some(tag) = system.tag.as_deref() {
            Cow::Owned(format!("{} {}", username, tag))
        } else {
            Cow::Borrowed(username)
        };

        let new_message = self
            .bot
            .send_message_webhook(
                &webhook,
                Some(trimmed_message_content.to_string()),
                None,
                message.referenced_message(),
                &files,
                &username,
                member
                    .avatar_url
                    .as_deref()
                    .or(system.avatar_url.as_deref()),
            )
            .await?;

        A::create_message(
            self,
            new_message.id(),
            message.author().id(),
            system.id,
            member.id,
        )
        .await?;

        self.bot
            .delete_message(message.channel_id(), message.id())
            .await?;

        return Ok(());
    }

    async fn on_message_raw(&self, message: &A::Message) -> anyhow::Result<()> {
        if message.created_by_bot() {
            return Ok(());
        }

        let content = message.content();

        for prefix in Self::PREFIXES {
            if !content.starts_with(prefix) {
                continue;
            }

            let command = &content[prefix.len()..];

            self.handle_command(message, command).await?;

            return Ok(());
        }

        let user_id = message.author().id();

        let Some(system_id) = A::fetch_system_id(self, user_id).await? else {
            return Ok(());
        };

        let mut proxied_member = None;

        let content = message.content();

        let mut proxies = A::fetch_system_proxies(self, system_id).await?;

        proxies.sort_by_key(|a| a.0.len());

        let mut trimmed_message_content = content;

        for (proxy, member_id) in A::fetch_system_proxies(self, system_id).await? {
            let Some((prefix, suffix)) = proxy.split_once("text") else {
                continue;
            };

            let total_len = prefix.len() + suffix.len();

            if content.len() <= total_len {
                continue;
            }

            if content.starts_with(prefix) && content.ends_with(suffix) {
                let start = prefix.len();
                let end = content.len() - suffix.len();

                trimmed_message_content = &content[start..end];

                proxied_member = Some(member_id);
                break;
            }
        }

        let Some(member_id) = proxied_member else {
            return Ok(());
        };

        let member = A::fetch_member_by_id(self, system_id, member_id)
            .await?
            .ok_or(NoMemberToProxyError)?;

        self.resend_message(message, &member, trimmed_message_content)
            .await?;

        return Ok(());
    }

    pub async fn on_message(&self, message: &A::Message) -> anyhow::Result<()> {
        return self
            .with_error_handler(message, self.on_message_raw(message))
            .await;
    }
}
