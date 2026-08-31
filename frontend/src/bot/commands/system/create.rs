use pluxer_backend::{PluxerApi, bot::BackendBot, message::BackendMessage};
use pluxer_database::sea_orm::entity::prelude::async_trait::async_trait;

use crate::bot::{PluxerContext, command_parser::{CommandArguments, CommandExecutor, builder::CommandBuilder, node::unix::UnixParameter}};

pub struct CreateSystemCommand;

impl CreateSystemCommand {
    const NAME: &str = "name";
    const TAG: &str = "tag";
    const AVATAR_URL: &str = "avatar_url";

    const UNIX_PARAMETERS: &[UnixParameter] = &[
        UnixParameter::value(Self::TAG, &["tag", "t"]),
        UnixParameter::value(Self::AVATAR_URL, &["avatar_url", "avatar", "a"]),
    ];
    
    pub fn append<A: PluxerApi>(command: &mut CommandBuilder<PluxerContext<A>>) {
        command.literal(&["new", "n", "create", "c", "make"], |command| {
            command.executes(CreateSystemCommand);

            command.unix(&Self::UNIX_PARAMETERS, |command| {
                command.greedy_string(Self::NAME, |_| {});
            });
        });
    }
}

#[async_trait]
impl<A: PluxerApi> CommandExecutor<PluxerContext<A>> for CreateSystemCommand {
    async fn execute<'a>(
        &self,
        args: &'a CommandArguments<'a>,
        context: &'a PluxerContext<A>,
        message: &A::Message,
    ) -> anyhow::Result<()> {
        context.bot.send_message(message.channel_id(), &format!("{:#?}", args)).await?;

        return Ok(());
    }
}
