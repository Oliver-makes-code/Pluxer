use crate::bot::command::{
    CommandParser,
    cursor::SourceSpan,
    node::{CommandArgument, parse_string},
};

pub struct StringArgument {
    pub argument_name: &'static str,
}

pub struct StringListArgument {
    pub argument_name: &'static str,
}

impl CommandArgument for StringArgument {
    fn parse<'a>(&self, parser: &mut CommandParser<'a>) -> Option<SourceSpan<'a>> {
        let string = parse_string(parser)?;

        parser.arguments.insert(
            self.argument_name,
            string.span.into_spanned(Box::new([string])),
        );

        return Some(string.span);
    }
}

impl CommandArgument for StringListArgument {
    fn parse<'a>(&self, context: &mut CommandParser<'a>) -> Option<SourceSpan<'a>> {
        let mut strings = vec![];

        while let Some(string) = parse_string(context) {
            strings.push(string);
        }

        let span = strings.first()?.span + strings.last()?.span;

        context
            .arguments
            .insert(self.argument_name, span.into_spanned(strings.into()));

        return Some(span);
    }
}
