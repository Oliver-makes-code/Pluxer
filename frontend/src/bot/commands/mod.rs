use pluxer_backend::{PluxerApi, bot::BackendBot, message::BackendMessage};
use pluxer_database::sea_orm::entity::prelude::async_trait::async_trait;

use crate::bot::{
    PluxerContext,
    command_parser::{CommandArguments, CommandExecutor, CommandRoot, builder::CommandBuilder},
};

pub struct TestCommand;

#[async_trait]
impl<A: PluxerApi> CommandExecutor<PluxerContext<A>> for TestCommand {
    async fn execute<'a>(
        &self,
        args: &'a CommandArguments<'a>,
        context: &'a PluxerContext<A>,
        message: &A::Message,
    ) -> anyhow::Result<()> {
        context
            .bot
            .send_message(message.channel_id(), &format!("{:#?}", args))
            .await?;

        return Ok(());
    }
}

pub fn create_command_tree<'a, A: PluxerApi>() -> CommandRoot<PluxerContext<A>> {
    return CommandBuilder::<PluxerContext<A>>::build(|command| {
        command.literal(&["test"], |test| {
            test.string("test", |test| {
                test.executes(TestCommand);
            });

            test.executes(TestCommand);
        });
    });
}
