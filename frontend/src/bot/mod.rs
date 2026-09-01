use std::{fs, path::Path, sync::Arc};

use pluxer_backend::{bot::BackendBot, embed::Embed, message::BackendMessage};
use pluxer_database::{connect, sea_orm::DatabaseConnection};
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
    pub instance_name: Arc<str>,
    pub instance_url: Arc<str>,
}

impl<A: DatabaseExtension> CommandContext for PluxerContext<A> {
    type CommandData = A::Message;
}

impl<A: DatabaseExtension> PluxerContext<A> {
    const PREFIX: &str = "pl!";

    pub async fn new(
        bot: A::Bot,
        database_url: &str,
        instance_name: Arc<str>,
        instance_url: Arc<str>,
    ) -> anyhow::Result<Self> {
        return Ok(Self {
            bot,
            database_connection: connect(database_url).await?,
            command_tree: create_command_tree(),
            instance_name,
            instance_url,
        });
    }

    pub async fn on_message(&self, message: &A::Message) -> anyhow::Result<()> {
        if message.created_by_bot() {
            return Ok(());
        }

        let content = message.content();

        if !content.starts_with(Self::PREFIX) {
            return Ok(());
        }

        let command = &content[Self::PREFIX.len()..];

        if let Err(err) = parse_command(command, &self.command_tree, self, message).await {
            let error_id = Ulid::generate();

            let mut error_report = String::new();

            error_report += &format!("{}", err);

            let result = self.bot.send_message(message.channel_id(), None, Some(Embed {
                title: Some("Unexpected error!".into()),
                description: Some("Please report to our [GitHub Repository](https://github.com/Oliver-makes-code/Pluxer/issues), and include the error ID listed below.".into()),
                footer: Some(format!("Error ID: {}", error_id)),

                ..Default::default()
            })).await;

            if let Err(err) = result {
                error_report += &format!("\n\n{}", err);
            };

            pawkit_logger::log!(error, "{}: {}", error_id, error_report);

            let path = Path::new("logs/error");

            fs::create_dir_all(path)?;

            let file_path = path.join(format!("{}.txt", error_id));

            fs::write(file_path, error_report)?;
        }

        return Ok(());
    }
}
