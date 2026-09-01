use std::io;

use pluxer_backend::{
    bot::BackendBot,
    embed::{Embed, EmbedField},
    message::BackendMessage,
    user::BackendUser,
};
use pluxer_database::sea_orm::entity::prelude::async_trait::async_trait;
use thiserror::Error;

use crate::{
    bot::{
        PluxerContext,
        command_parser::{CommandArguments, CommandExecutor, builder::CommandBuilder},
        commands::system::create::CreateSystemCommand,
    },
    database::DatabaseExtension,
};

mod create;

pub struct SystemCommand;

impl SystemCommand {
    pub fn append<A: DatabaseExtension>(command: &mut CommandBuilder<PluxerContext<A>>) {
        command.literal(&["system", "sys", "s"], |command| {
            command.executes(SystemCommand);

            CreateSystemCommand::append(command);
        });
    }
}

#[async_trait]
impl<A: DatabaseExtension> CommandExecutor<PluxerContext<A>> for SystemCommand {
    async fn execute<'a>(
        &self,
        _args: &'a CommandArguments<'a>,
        context: &'a PluxerContext<A>,
        message: &A::Message,
    ) -> anyhow::Result<()> {
        let Some(system) = A::fetch_system_by_user(context, message.author().id()).await? else {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some("You do not have a system. Create one with `pl!system new <name>`".into()),
                    None,
                )
                .await?;

            return Ok(());
        };

        let mut fields = vec![];

        if let Some(tag) = system.tag {
            fields.push(EmbedField {
                name: "Tag".into(),
                value: tag,
                inline: true,
            });
        }

        let embed = Embed {
            title: Some(system.name),
            description: system.description,
            fields,
            footer: Some(format!("System ID: {}", system.id)),
            ..Default::default()
        };

        context
            .bot
            .send_message(message.channel_id(), None, Some(embed))
            .await?;

        return Ok(());
    }
}
