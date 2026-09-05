use async_trait::async_trait;
use pluxer_backend::{
    PluxerApi,
    bot::BackendBot,
    embed::Embed,
    message::{BackendMessage, ReferencedMessageKind},
    user::BackendUser,
};
use pluxer_database::model::system::SystemModel;

use crate::bot::{
    PluxerContext,
    command_parser::{CommandArguments, CommandExecutor, builder::CommandBuilder},
    commands::{
        member::MemberCommand,
        system::{
            create::CreateSystemCommand, delete::DeleteSystemCommand, update::UpdateSystemCommand,
        },
    },
};

pub mod create;
pub mod delete;
pub mod update;

pub struct SystemCommand;

impl SystemCommand {
    pub fn append<A: PluxerApi>(command: &mut CommandBuilder<PluxerContext<A>>) {
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

        {
            let mut info_row = vec![];

            system
                .pronouns
                .map(|it| description.push(format!("**Pronouns:** {}", it)));

            system
                .tag
                .map(|it| description.push(format!("**Tag:** {}", it)));

            info_row.push(format!("**Members:**: {}", member_count));

            if !info_row.is_empty() {
                description.push(format!("\n{}", info_row.join("\n")));
            }
        }

        {
            description.push(format!("\n-# System ID: `{}`", system.id));
        }

        return Embed {
            title: Some(system.display_name.unwrap_or(system.name)),
            description: Some(description.join("\n").trim().into()),
            color: system.color.unwrap_or(0),
            thumbnail_url: system.avatar_url,
            ..Default::default()
        };
    }
}

#[async_trait]
impl<A: PluxerApi> CommandExecutor<PluxerContext<A>> for SystemCommand {
    async fn execute<'a>(
        &self,
        _args: &'a CommandArguments<'a>,
        context: &'a PluxerContext<A>,
        message: &A::Message,
    ) -> anyhow::Result<()> {
        let Some(system_id) = context
            .database
            .fetch_system_id(context.get_platform_id(message.author().id()))
            .await?
        else {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some(&format!(
                        "You do not have a system. Create one with `{}`",
                        CreateSystemCommand::USAGE
                    )),
                    None,
                    Some((ReferencedMessageKind::Reply, message)),
                    &[],
                )
                .await?;

            return Ok(());
        };

        let Some(system) = context.database.fetch_system_by_id(system_id).await? else {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some(&format!(
                        "You do not have a system. Create one with `{}`",
                        CreateSystemCommand::USAGE
                    )),
                    None,
                    Some((ReferencedMessageKind::Reply, message)),
                    &[],
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
                    context.database.fetch_member_count(system_id).await?,
                )),
                Some((ReferencedMessageKind::Reply, message)),
                &[],
            )
            .await?;

        return Ok(());
    }
}
