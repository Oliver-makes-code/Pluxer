use pluxer_backend::{PluxerApi, bot::BackendBot, message::BackendMessage};
use pluxer_database::sea_orm::entity::prelude::async_trait::async_trait;

use crate::{
    bot::{
        PluxerContext,
        command_parser::{CommandArguments, CommandExecutor, CommandRoot, builder::CommandBuilder},
        commands::system::SystemCommand,
    },
    database::DatabaseExtension,
};

mod system;

pub fn create_command_tree<'a, A: DatabaseExtension>() -> CommandRoot<PluxerContext<A>> {
    return CommandBuilder::<PluxerContext<A>>::build(|command| {
        SystemCommand::append(command);
    });
}
