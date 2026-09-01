use crate::bot::command_parser::{
    CommandContext, CommandExecutor,
    node::{
        CommandArgumentKind, CommandNode,
        greedy::GreedyStringArgument,
        literal::LiteralArgument,
        string::{StringArgument, StringListArgument},
        unix::{UnixArgument, UnixParameter},
    },
};

pub struct CommandBuilder<C: CommandContext> {
    argument: Option<CommandArgumentKind>,
    children: Vec<CommandBuilder<C>>,
    executes: Option<Box<dyn CommandExecutor<C>>>,
}

impl<C: CommandContext> CommandBuilder<C> {
    fn build_internal<F: FnOnce(&mut Self)>(argument: Option<CommandArgumentKind>, f: F) -> Self {
        let mut builder = Self {
            argument,
            children: vec![],
            executes: None,
        };

        f(&mut builder);

        return builder;
    }

    fn children_into_nodes(children: Vec<Self>) -> Box<[CommandNode<C>]> {
        return children
            .into_iter()
            .map(Self::into_node)
            .map(Option::unwrap)
            .collect();
    }

    fn into_node(self) -> Option<CommandNode<C>> {
        return Some(CommandNode {
            argument: self.argument?,
            children: Self::children_into_nodes(self.children),
            executes: self.executes,
        });
    }

    pub fn build<F: FnOnce(&mut Self)>(f: F) -> Box<[CommandNode<C>]> {
        let builder = Self::build_internal(None, f);

        return Self::children_into_nodes(builder.children);
    }

    pub fn executes(&mut self, executes: impl CommandExecutor<C> + 'static) {
        self.executes = Some(Box::new(executes));
    }

    pub fn literal<F: FnOnce(&mut Self)>(
        &mut self,
        literals: &'static [&'static str],
        f: F,
    ) -> &mut Self {
        self.children.push(Self::build_internal(
            Some(CommandArgumentKind::Literal(LiteralArgument { literals })),
            f,
        ));

        return self;
    }

    pub fn string<F: FnOnce(&mut Self)>(&mut self, argument_name: &'static str, f: F) -> &mut Self {
        self.children.push(Self::build_internal(
            Some(CommandArgumentKind::String(StringArgument {
                argument_name,
            })),
            f,
        ));

        return self;
    }

    #[allow(unused)]
    pub fn string_list<F: FnOnce(&mut Self)>(
        &mut self,
        argument_name: &'static str,
        f: F,
    ) -> &mut Self {
        self.children.push(Self::build_internal(
            Some(CommandArgumentKind::StringList(StringListArgument {
                argument_name,
            })),
            f,
        ));

        return self;
    }

    pub fn greedy_string<F: FnOnce(&mut Self)>(
        &mut self,
        argument_name: &'static str,
        f: F,
    ) -> &mut Self {
        self.children.push(Self::build_internal(
            Some(CommandArgumentKind::GreedyString(GreedyStringArgument {
                argument_name,
            })),
            f,
        ));

        return self;
    }

    pub fn unix<F: FnOnce(&mut Self)>(
        &mut self,
        arguments: &'static [UnixParameter],
        f: F,
    ) -> &mut Self {
        self.children.push(Self::build_internal(
            Some(CommandArgumentKind::Unix(UnixArgument { arguments })),
            f,
        ));

        return self;
    }
}
