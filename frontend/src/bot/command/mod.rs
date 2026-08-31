use std::collections::HashMap;

use crate::bot::command::{cursor::{SourceCursor, Spanned}, node::{CommandArgument, CommandNode}};

pub mod builder;
mod cursor;
pub mod node;

pub type CommandArguments<'a> = HashMap<&'static str, Spanned<'a, Box<[Spanned<'a, &'a str>]>>>;
pub type CommandExecutor<C> = fn(&CommandArguments<'_>, &mut C);
pub type CommandRoot<C> = Box<[CommandNode<C>]>;

pub struct CommandParser<'a> {
    pub cursor: SourceCursor<'a>,
    pub arguments: CommandArguments<'a>,
}

pub fn parse_command<'a, C>(source: &'a str, root: CommandRoot<C>, context: &mut C) {
    let mut parser = CommandParser {
        cursor: SourceCursor::new(source, None),
        arguments: Default::default(),
    };

    let mut arguments_to_parse: &[CommandNode<C>] = &root;
    let mut latest_executor: Option<CommandExecutor<C>> = None;

    let mut i = 0;

    loop {
        if i == arguments_to_parse.len() {
            if let Some(executor) = latest_executor {
                executor(&parser.arguments, context);
            }

            return;
        }

        let arg = &arguments_to_parse[i];

        if let Some(span) = arg.argument.parse(&mut parser) {
            if let Some(executor) = arg.executes {
                latest_executor = Some(executor);
            }

            i = 0;
            arguments_to_parse = &arg.children;
            continue;
        }

        i += 1;
    }
}
