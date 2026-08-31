use pluxer_backend::{PluxerApi, message::BackendMessage};

use crate::bot::{
    command_parser::{CommandContext, CommandRoot, parse_command},
    commands::create_command_tree,
};

mod command_parser;
mod commands;

pub struct PluxerContext<A: PluxerApi> {
    pub bot: A::Bot,
    pub command_tree: CommandRoot<PluxerContext<A>>,
}

impl<A: PluxerApi> CommandContext for PluxerContext<A> {
    type CommandData = A::Message;
}

impl<A: PluxerApi> PluxerContext<A> {
    const PREFIX: &str = "pl!";

    pub fn new(bot: A::Bot) -> Self {
        return Self {
            bot,
            command_tree: create_command_tree(),
        };
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

        parse_command(command, &self.command_tree, self, message).await?;

        return Ok(());
    }
}
