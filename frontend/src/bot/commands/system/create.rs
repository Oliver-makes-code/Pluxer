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

pub struct CreateSystemCommand;

impl CreateSystemCommand {
    const NAME: &str = "name";
    const TAG: &str = "tag";
    const AVATAR_URL: &str = "avatar_url";

    const UNIX_PARAMETERS: &[UnixParameter] = &[
        UnixParameter::value(Self::TAG, &["tag", "t"]),
        UnixParameter::value(Self::AVATAR_URL, &["avatar_url", "avatar", "a"]),
    ];

    pub const USAGE: &str = "pl!system create [--tag=<tag>] [--avatar_url=<avatar_url>] <name>";

    pub fn append<A: DatabaseExtension>(command: &mut CommandBuilder<PluxerContext<A>>) {
        command.literal(&["new", "n", "create", "c", "make"], |command| {
            command.executes(CreateSystemCommand);

            command.unix(&Self::UNIX_PARAMETERS, |command| {
                command.greedy_string(Self::NAME, |_| {});
            });
        });
    }
}

#[async_trait]
impl<A: DatabaseExtension> CommandExecutor<PluxerContext<A>> for CreateSystemCommand {
    async fn execute<'a>(
        &self,
        args: &'a CommandArguments<'a>,
        context: &'a PluxerContext<A>,
        message: &A::Message,
    ) -> anyhow::Result<()> {
        let system_id = A::fetch_system_id(context, message.author().id()).await?;

        if system_id.is_some() {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some("You have already created a system. View it with `pl!system`".into()),
                    None,
                    Some(message),
                )
                .await?;

            return Ok(());
        }

        let Some(name) = get_argument_single(args, Self::NAME) else {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some(format!("Usage: `{}`", Self::USAGE)),
                    None,
                    Some(message),
                )
                .await?;

            return Ok(());
        };

        let tag = get_argument_single(args, Self::TAG);

        let avatar_url = get_argument_single(args, Self::AVATAR_URL);

        let system_id =
            A::create_system(context, message.author().id(), name, avatar_url, tag).await?;

        context
            .bot
            .send_message(
                message.channel_id(),
                Some(format!(
                    "System created! View it with `pl!system`\n-# System ID: {}",
                    system_id
                )),
                None,
                Some(message),
            )
            .await?;

        return Ok(());
    }
}
