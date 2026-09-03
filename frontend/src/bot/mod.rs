use std::{fs, path::Path, sync::Arc, time::Duration};

use moka::{future::Cache, policy::EvictionPolicy};
use pluxer_backend::{
    bot::BackendBot, embed::Embed, message::BackendMessage, user::BackendUser,
    webhook::BackendWebhook,
};
use pluxer_database::sea_orm::DatabaseConnection;
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

    async fn handle_command(
        &self,
        message: &A::Message,
        command_substring: &str,
    ) -> anyhow::Result<()> {
        let Err(err) = parse_command(command_substring, &self.command_tree, self, message).await
        else {
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

    pub async fn on_message(&self, message: &A::Message) -> anyhow::Result<()> {
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

        // TODO: Proxy messages here

        return Ok(());
    }
}
