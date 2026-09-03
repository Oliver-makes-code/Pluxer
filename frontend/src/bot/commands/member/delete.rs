use pluxer_backend::{bot::BackendBot, message::BackendMessage, user::BackendUser};
use pluxer_database::sea_orm::entity::prelude::async_trait::async_trait;

use crate::{
    bot::{
        PluxerContext,
        command_parser::{
            CommandArguments, CommandExecutor, builder::CommandBuilder, get_argument_single,
        },
        commands::{
            DELETE_VARIANTS, YES, YES_UNIX, member::MemberCommand,
            system::create::CreateSystemCommand,
        },
    },
    database::DatabaseExtension,
};

pub struct DeleteMemberCommand;

impl DeleteMemberCommand {
    pub const USAGE: &str = "pl!member <name> delete [-y]";

    pub fn append<A: DatabaseExtension>(command: &mut CommandBuilder<PluxerContext<A>>) {
        command.literal(DELETE_VARIANTS, |command| {
            command.executes(DeleteMemberCommand);

            command.unix(YES_UNIX, |_| {});
        });
    }
}

#[async_trait]
impl<A: DatabaseExtension> CommandExecutor<PluxerContext<A>> for DeleteMemberCommand {
    async fn execute<'a>(
        &self,
        args: &'a CommandArguments<'a>,
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
                    &[],
                )
                .await?;

            return Ok(());
        };

        let Some(name) = get_argument_single(args, MemberCommand::MEMBER_NAME) else {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some(format!("Usage: `{}`", Self::USAGE)),
                    None,
                    Some(message),
                    &[],
                )
                .await?;

            return Ok(());
        };

        let Some(member) = MemberCommand::fetch_member(context, system_id, name).await? else {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some(format!(
                        "Unable to find member '{}'. Are you sure you typed in the name correctly?",
                        name
                    )),
                    None,
                    Some(message),
                    &[],
                )
                .await?;

            return Ok(());
        };

        let Some(_) = get_argument_single(args, YES) else {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some(format!("Are you sure you want to delete this member? Rerun the command as `pl!system delete -y {}` to confirm.", name)),
                    None,
                    Some(message),
                    &[],
                )
                .await?;

            return Ok(());
        };

        A::delete_member(context, system_id, member.id).await?;

        context
            .bot
            .send_message(
                message.channel_id(),
                Some("Member deleted.".into()),
                None,
                Some(message),
                &[],
            )
            .await?;

        return Ok(());
    }
}
