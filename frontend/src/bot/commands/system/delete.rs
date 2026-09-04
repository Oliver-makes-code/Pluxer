use async_trait::async_trait;
use pluxer_backend::{
    PluxerApi,
    bot::BackendBot,
    message::{BackendMessage, ReferencedMessageKind},
    user::BackendUser,
};

use crate::bot::{
    PluxerContext,
    command_parser::{
        CommandArguments, CommandExecutor, builder::CommandBuilder, get_argument_single,
    },
    commands::{DELETE_VARIANTS, YES, YES_UNIX},
};

pub struct DeleteSystemCommand;

impl DeleteSystemCommand {
    pub fn append<A: PluxerApi>(command: &mut CommandBuilder<PluxerContext<A>>) {
        command.literal(DELETE_VARIANTS, |command| {
            command.executes(DeleteSystemCommand);

            command.unix(YES_UNIX, |_| {});
        });
    }
}

#[async_trait]
impl<A: PluxerApi> CommandExecutor<PluxerContext<A>> for DeleteSystemCommand {
    async fn execute<'a>(
        &self,
        args: &'a CommandArguments<'a>,
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
                    Some("You already do not have a system.".into()),
                    None,
                    Some((ReferencedMessageKind::Reply, message)),
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
                    Some("Are you sure you want to delete your system? Rerun the command as `pl!system delete -y` to confirm.".into()),
                    None,
                    Some((ReferencedMessageKind::Reply, message)),
                    &[],
                )
                .await?;

            return Ok(());
        };

        if context
            .database
            .detach_or_delete_system(context.get_platform_id(message.author().id()), system_id)
            .await?
        {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some("System deleted.".into()),
                    None,
                    Some((ReferencedMessageKind::Reply, message)),
                    &[],
                )
                .await?;
        } else {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some("System detached from this accout.".into()),
                    None,
                    Some((ReferencedMessageKind::Reply, message)),
                    &[],
                )
                .await?;
        }

        return Ok(());
    }
}
