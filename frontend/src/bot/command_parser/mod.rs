use std::{collections::HashMap, ops::Deref};

use async_trait::async_trait;

use crate::bot::command_parser::{
    cursor::{SourceCursor, Spanned},
    node::{CommandArgument, CommandNode},
};

pub mod builder;
mod cursor;
pub mod node;

pub type CommandArguments<'a> = HashMap<&'static str, Spanned<'a, Box<[Spanned<'a, &'a str>]>>>;
pub type CommandRoot<C: CommandContext> = Box<[CommandNode<C>]>;

pub trait CommandContext: Send + Sync {
    type CommandData: Send + Sync;
}

#[async_trait]
pub trait CommandExecutor<C: CommandContext>: Send + Sync {
    async fn execute<'a>(
        &self,
        args: &'a CommandArguments<'a>,
        context: &'a C,
        data: &C::CommandData,
    ) -> anyhow::Result<()>;
}

pub struct CommandParser<'a> {
    pub cursor: SourceCursor<'a>,
    pub arguments: CommandArguments<'a>,
}

pub fn get_argument_single<'a>(args: &CommandArguments<'a>, name: &'static str) -> Option<&'a str> {
    let arg = args.get(name)?;

    return arg.value.first().map(|it| it.value);
}

pub async fn parse_command<'a, C: CommandContext>(
    source: &'a str,
    root: &CommandRoot<C>,
    context: &C,
    data: &C::CommandData,
) -> anyhow::Result<()> {
    let mut parser = CommandParser {
        cursor: SourceCursor::new(source, None),
        arguments: Default::default(),
    };

    let mut arguments_to_parse: &[CommandNode<C>] = &root;
    let mut latest_executor: Option<&dyn CommandExecutor<C>> = None;

    let mut i = 0;

    loop {
        if i == arguments_to_parse.len() {
            if let Some(executor) = latest_executor {
                executor.execute(&parser.arguments, context, data).await?;
            }

            return Ok(());
        }

        let arg = &arguments_to_parse[i];

        parser.cursor.consume_fn(char::is_whitespace);
        let _ = parser.cursor.commit();

        if arg.argument.parse(&mut parser).is_some() {
            if let Some(executor) = &arg.executes {
                latest_executor = Some(executor.deref());
            }

            i = 0;
            arguments_to_parse = &arg.children;
            continue;
        }

        i += 1;
    }
}
