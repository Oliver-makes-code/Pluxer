use pluxer_backend::PluxerApi;
use pluxer_database::sea_orm::entity::prelude::async_trait::async_trait;

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
        args: &'a CommandArguments<'a>,
        context: &'a PluxerContext<A>,
        message: &A::Message,
    ) -> anyhow::Result<()> {
        return Ok(());
    }
}
