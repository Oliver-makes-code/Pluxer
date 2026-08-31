use crate::bot::command::{
    CommandParser,
    cursor::SourceSpan,
    node::{CommandArgument, parse_string},
};

pub enum UnixParameterKind {
    Flag,
    Value,
}

pub struct UnixParameter {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub kind: UnixParameterKind,
}

pub struct UnixArgument {
    pub arguments: &'static [UnixParameter],
}

impl UnixParameter {
    fn parse<'a>(&self, parser: &mut CommandParser<'a>) -> Option<SourceSpan<'a>> {
        for alias in self.aliases {
            if !parser.cursor.consume_str(alias) {
                continue;
            }

            match self.kind {
                UnixParameterKind::Flag => {
                    if !parser.cursor.is_eof_or_fn(char::is_whitespace) {
                        parser.cursor.rollback();
                        continue;
                    }

                    let argument_span = parser.cursor.commit();

                    parser.arguments.insert(
                        self.name,
                        argument_span
                            .into_spanned(Box::new([argument_span.into_spanned(self.name)])),
                    );

                    return Some(argument_span);
                }

                UnixParameterKind::Value => {
                    if parser.cursor.consume_char('=') {
                        // -name=value
                    } else if parser.cursor.is_fn(char::is_whitespace) {
                        parser.cursor.while_fn(char::is_whitespace);
                    } else {
                        parser.cursor.rollback();
                        continue;
                    }

                    let value = match parse_string(parser) {
                        Some(value) => value,
                        None => {
                            parser.cursor.rollback();
                            return None;
                        }
                    };

                    let argument_span = parser.cursor.commit();

                    parser
                        .arguments
                        .insert(self.name, argument_span.into_spanned(Box::new([value])));

                    return Some(argument_span);
                }
            }
        }

        return None;
    }
}

impl CommandArgument for UnixArgument {
    fn parse<'a>(&self, context: &mut CommandParser<'a>) -> Option<SourceSpan<'a>> {
        let mut span: Option<SourceSpan<'a>> = None;

        loop {
            context.cursor.while_fn(char::is_whitespace);

            if !context.cursor.consume_char('-') {
                break;
            }

            // We want to support both `-` and `--`.
            context.cursor.consume_char('-');

            let mut matched = false;

            for argument in self.arguments {
                let Some(argument_span) = argument.parse(context) else {
                    continue;
                };

                span = Some(span.unwrap_or(argument_span) + argument_span);
                matched = true;
            }

            if !matched {
                context.cursor.rollback();
                return None;
            }
        }

        return span;
    }
}
