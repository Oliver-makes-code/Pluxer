use pluxer_backend::{bot::BackendBot, embed::Embed, message::BackendMessage, user::BackendUser};
use pluxer_database::{
    model::system::SystemModel, sea_orm::entity::prelude::async_trait::async_trait,
};

use crate::{
    bot::{
        PluxerContext,
        command_parser::{CommandArguments, CommandExecutor, builder::CommandBuilder},
        commands::{
            member::MemberCommand,
            system::{
                create::CreateSystemCommand, delete::DeleteSystemCommand,
                update::UpdateSystemCommand,
            },
        },
    },
    database::DatabaseExtension,
};

pub mod create;
pub mod delete;
pub mod update;

pub struct SystemCommand;

impl SystemCommand {
    pub fn append<A: DatabaseExtension>(command: &mut CommandBuilder<PluxerContext<A>>) {
        command.literal(&["system", "sys", "s"], |command| {
            command.executes(SystemCommand);

            MemberCommand::append(command);

            CreateSystemCommand::append(command);
            DeleteSystemCommand::append(command);
            UpdateSystemCommand::append(command);
        });
    }

    pub fn system_to_embed(system: SystemModel, member_count: usize) -> Embed {
        let mut description = vec![];

        system.description.map(|it| description.push(it));

        system
            .pronouns
            .map(|it| description.push(format!("\n**Pronouns:** {}", it)));

        system
            .tag
            .map(|it| description.push(format!("\n**Tag:** {}", it)));

        description.push(format!("\n**Members:**: {}", member_count));

        description.push(format!("\n-# System ID: `{}`", system.id));

        return Embed {
            title: Some(system.display_name.unwrap_or(system.name)),
            description: Some(description.join("\n")),
            color: system.color.unwrap_or(0),
            thumbnail_url: system.avatar_url,
            ..Default::default()
        };
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
        let Some(system_id) = A::fetch_system_id(context, message.author().id()).await? else {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some(format!(
                        "You do not have a system. Create one with `{}`",
                        CreateSystemCommand::USAGE
                    )),
                    None,
                    Some(message),
                )
                .await?;

            return Ok(());
        };

        let Some(system) = A::fetch_system_by_id(context, system_id).await? else {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some(format!(
                        "You do not have a system. Create one with `{}`",
                        CreateSystemCommand::USAGE
                    )),
                    None,
                    Some(message),
                )
                .await?;

            return Ok(());
        };

        context
            .bot
            .send_message(
                message.channel_id(),
                None,
                Some(Self::system_to_embed(
                    system,
                    A::fetch_member_count(context, system_id).await?,
                )),
                Some(message),
            )
            .await?;

        return Ok(());
    }
}
