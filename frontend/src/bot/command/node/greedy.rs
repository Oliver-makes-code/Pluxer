use crate::bot::command::{CommandParser, cursor::SourceSpan, node::CommandArgument};

pub struct GreedyStringArgument {
    pub argument_name: &'static str,
}

impl CommandArgument for GreedyStringArgument {
    fn parse<'a>(&self, parser: &mut CommandParser<'a>) -> Option<SourceSpan<'a>> {
        parser.cursor.consume_until_eof();

        let span = parser.cursor.commit();

        let trimmed = span.slice().trim();

        parser.arguments.insert(
            self.argument_name,
            span.into_spanned(Box::new([span.into_spanned(trimmed)])),
        );

        return Some(span);
    }
}
