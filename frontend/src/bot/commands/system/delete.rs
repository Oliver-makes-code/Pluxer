use pluxer_backend::{bot::BackendBot, message::BackendMessage, user::BackendUser};
use pluxer_database::sea_orm::entity::prelude::async_trait::async_trait;

use crate::{
    bot::{
        PluxerContext,
        command_parser::{
            CommandArguments, CommandExecutor, builder::CommandBuilder, get_argument_single,
            node::unix::UnixParameter,
        },
    },
    database::DatabaseExtension,
};

pub struct DeleteSystemCommand;

impl DeleteSystemCommand {
    const YES: &str = "yes";

    const UNIX_PARAMETERS: &[UnixParameter] = &[UnixParameter::flag(Self::YES, &["yes", "y"])];

    pub fn append<A: DatabaseExtension>(command: &mut CommandBuilder<PluxerContext<A>>) {
        command.literal(&["delete", "remove", "del", "rem"], |command| {
            command.executes(DeleteSystemCommand);

            command.unix(Self::UNIX_PARAMETERS, |_| {});
        });
    }
}

#[async_trait]
impl<A: DatabaseExtension> CommandExecutor<PluxerContext<A>> for DeleteSystemCommand {
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
                    Some("You already do not have a system.".into()),
                    None,
                    Some(message),
                )
                .await?;

            return Ok(());
        };

        let Some(_) = get_argument_single(args, Self::YES) else {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some("Are you sure you want to delete your system? Rerun the command as `pl!system delete -y` to confirm.".into()),
                    None,
                    Some(message),
                )
                .await?;

            return Ok(());
        };

        if A::detach_or_delete_system(context, message.author().id(), system_id).await? {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some("System deleted.".into()),
                    None,
                    Some(message),
                )
                .await?;
        } else {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some("System detached from this accout.".into()),
                    None,
                    Some(message),
                )
                .await?;
        }

        return Ok(());
    }
}
