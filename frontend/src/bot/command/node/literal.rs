use crate::bot::command::{CommandParser, cursor::SourceSpan, node::CommandArgument};

pub struct LiteralArgument {
    pub literals: &'static [&'static str],
}

impl CommandArgument for LiteralArgument {
    fn parse<'a>(&self, parser: &mut CommandParser<'a>) -> Option<SourceSpan<'a>> {
        if parser.cursor.is_eof() {
            return None;
        }

        for literal in self.literals {
            if !parser.cursor.consume_str(*literal) {
                continue;
            }

            if !parser.cursor.is_eof_or_fn(char::is_whitespace) {
                parser.cursor.rollback();
                continue;
            }

            return Some(parser.cursor.commit());
        }

        return None;
    }
}
