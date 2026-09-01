use crate::{
    bot::{
        PluxerContext,
        command_parser::{CommandRoot, builder::CommandBuilder},
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
