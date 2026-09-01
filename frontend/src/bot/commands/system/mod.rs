use pluxer_backend::{bot::BackendBot, embed::Embed, message::BackendMessage, user::BackendUser};
use pluxer_database::sea_orm::entity::prelude::async_trait::async_trait;

use crate::{
    bot::{
        PluxerContext,
        command_parser::{CommandArguments, CommandExecutor, builder::CommandBuilder},
        commands::system::{create::CreateSystemCommand, delete::DeleteSystemCommand},
    },
    database::DatabaseExtension,
};

mod create;
mod delete;

pub struct SystemCommand;

impl SystemCommand {
    pub fn append<A: DatabaseExtension>(command: &mut CommandBuilder<PluxerContext<A>>) {
        command.literal(&["system", "sys", "s"], |command| {
            command.executes(SystemCommand);

            CreateSystemCommand::append(command);
            DeleteSystemCommand::append(command);
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
                    Some(format!("You do not have a system. Create one with `{}`", CreateSystemCommand::USAGE)),
                    None,
                    Some(message),
                )
                .await?;

            return Ok(());
        };

        let mut description = vec![];

        system.description.map(|it| description.push(it));

        system
            .tag
            .map(|it| description.push(format!("**Tag:** {}", it)));

        let embed = Embed {
            title: Some(system.name),
            description: Some(description.join("\n")),
            footer: Some(format!("System ID: {}", system.id)),
            ..Default::default()
        };

        context
            .bot
            .send_message(message.channel_id(), None, Some(embed), Some(message))
            .await?;

        return Ok(());
    }
}
