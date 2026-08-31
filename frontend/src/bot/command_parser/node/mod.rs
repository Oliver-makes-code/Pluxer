use crate::bot::command_parser::{
    CommandContext, CommandExecutor, CommandParser,
    cursor::{SourceSpan, Spanned},
    node::{
        greedy::GreedyStringArgument,
        literal::LiteralArgument,
        string::{StringArgument, StringListArgument},
        unix::UnixArgument,
    },
};

pub mod greedy;
pub mod literal;
pub mod string;
pub mod unix;

fn parse_quoted<'a>(parser: &mut CommandParser<'a>, quote: char) -> Option<Spanned<'a, &'a str>> {
    if !parser.cursor.consume_char(quote) {
        return None;
    }

    while !parser.cursor.consume_char(quote) {
        if parser.cursor.is_eof() {
            parser.cursor.rollback();
            return None;
        }

        if parser.cursor.consume_char('\\') {
            parser.cursor.advance();
        } else {
            parser.cursor.advance();
        }
    }

    let span = parser.cursor.commit();
    let slice = span.slice();

    return Some(span.into_spanned(&slice[1..slice.len() - 1]));
}

fn parse_string<'a>(parser: &mut CommandParser<'a>) -> Option<Spanned<'a, &'a str>> {
    if parser.cursor.is_eof() {
        return None;
    }

    if let Some(quote) = parse_quoted(parser, '"') {
        return Some(quote);
    }

    if let Some(quote) = parse_quoted(parser, '\'') {
        return Some(quote);
    }

    if !parser.cursor.is_fn(char::is_whitespace) {
        parser.cursor.while_fn(|c| !c.is_whitespace());

        let span = parser.cursor.commit();

        let slice = span.slice();

        return Some(span.into_spanned(slice));
    }

    return None;
}

pub trait CommandArgument {
    fn parse<'a>(&self, parser: &mut CommandParser<'a>) -> Option<SourceSpan<'a>>;
}

pub enum CommandArgumentKind {
    Literal(LiteralArgument),
    GreedyString(GreedyStringArgument),
    String(StringArgument),
    StringList(StringListArgument),
    Unix(UnixArgument),
}

impl CommandArgument for CommandArgumentKind {
    fn parse<'a>(&self, parser: &mut CommandParser<'a>) -> Option<SourceSpan<'a>> {
        match self {
            Self::Literal(literal) => return literal.parse(parser),
            Self::GreedyString(greedy) => return greedy.parse(parser),
            Self::String(string) => return string.parse(parser),
            Self::StringList(list) => return list.parse(parser),
            Self::Unix(unix) => return unix.parse(parser),
        }
    }
}

pub struct CommandNode<C: CommandContext> {
    pub argument: CommandArgumentKind,
    pub children: Box<[CommandNode<C>]>,
    pub executes: Option<Box<dyn CommandExecutor<C>>>,
}
