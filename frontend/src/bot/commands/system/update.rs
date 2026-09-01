use pluxer_backend::{bot::BackendBot, message::BackendMessage, user::BackendUser};
use pluxer_database::sea_orm::entity::prelude::async_trait::async_trait;

use crate::{
    bot::{
        PluxerContext,
        command_parser::{
            CommandArguments, CommandExecutor, builder::CommandBuilder, get_argument_single,
            node::unix::UnixParameter,
        },
        commands::system::{SystemCommand, create::CreateSystemCommand},
    },
    database::{DatabaseExtension, DatabaseUpdate},
};

pub struct UpdateSystemCommand {
    subcommand: Option<&'static str>,
    clear: bool,
}

impl UpdateSystemCommand {
    const UNIX_PARAMETERS_SET: &[UnixParameter] = &[
        UnixParameter::value(SystemCommand::NAME, SystemCommand::NAME_VARIANTS),
        UnixParameter::value(SystemCommand::TAG, SystemCommand::TAG_VARIANTS),
        UnixParameter::value(
            SystemCommand::AVATAR_URL,
            SystemCommand::AVATAR_URL_VARIANTS,
        ),
        UnixParameter::value(
            SystemCommand::DESCRIPTION,
            SystemCommand::DESCRIPTION_VARIANTS,
        ),
    ];
    const UNIX_PARAMETERS_CLEAR: &[UnixParameter] = &[
        UnixParameter::flag(SystemCommand::TAG, SystemCommand::TAG_VARIANTS),
        UnixParameter::flag(
            SystemCommand::AVATAR_URL,
            SystemCommand::AVATAR_URL_VARIANTS,
        ),
        UnixParameter::flag(
            SystemCommand::DESCRIPTION,
            SystemCommand::DESCRIPTION_VARIANTS,
        ),
    ];

    fn usage(&self) -> String {
        let Some(subcommand) = self.subcommand else {
            if self.clear {
                return "pl!system clear [--tag] [--avatar_url] [--description] [--name]".into();
            }

            return "pl!system update [--tag=\"<tag>\"] [--avatar_url=\"<avatar_url>\"] [--description=\"<description>\"] [--name=\"<name>\"]".into();
        };

        return format!(
            "pl!system {0}{1} <{0}>",
            subcommand,
            if self.clear { " clear" } else { "" }
        );
    }

    pub fn append<A: DatabaseExtension>(command: &mut CommandBuilder<PluxerContext<A>>) {
        command.literal(&["update", "set", "u"], |command| {
            command.executes(UpdateSystemCommand {
                subcommand: None,
                clear: false,
            });

            command.unix(&Self::UNIX_PARAMETERS_SET, |_| {});
        });

        command.literal(&["clear", "c"], |command| {
            command.executes(UpdateSystemCommand {
                subcommand: None,
                clear: true,
            });

            command.unix(&Self::UNIX_PARAMETERS_CLEAR, |_| {});
        });

        const SUBCOMMANDS: &[(&str, &[&str], bool)] = &[
            (SystemCommand::NAME, SystemCommand::NAME_VARIANTS, false),
            (SystemCommand::TAG, SystemCommand::TAG_VARIANTS, true),
            (
                SystemCommand::AVATAR_URL,
                SystemCommand::AVATAR_URL_VARIANTS,
                true,
            ),
            (
                SystemCommand::DESCRIPTION,
                SystemCommand::DESCRIPTION_VARIANTS,
                true,
            ),
        ];

        for (name, variants, allow_clear) in SUBCOMMANDS {
            command.literal(variants, |command| {
                if *allow_clear {
                    command.literal(&["clear", "c"], |command| {
                        command.executes(UpdateSystemCommand {
                            subcommand: Some(name),
                            clear: true,
                        });
                    });
                }

                command.executes(UpdateSystemCommand {
                    subcommand: Some(name),
                    clear: false,
                });

                command.greedy_string(name, |_| {});
            });
        }
    }
}

#[async_trait]
impl<A: DatabaseExtension> CommandExecutor<PluxerContext<A>> for UpdateSystemCommand {
    async fn execute<'a>(
        &self,
        args: &'a CommandArguments<'a>,
        context: &'a PluxerContext<A>,
        message: &A::Message,
    ) -> anyhow::Result<()> {
        let Some(system_id) = A::fetch_system_id(context, message.author().id()).await? else {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some(format!(
                        "You do not have a system. Create one with `{}`",
                        CreateSystemCommand::USAGE
                    )),
                    None,
                    Some(message),
                )
                .await?;

            return Ok(());
        };

        if args.is_empty() {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some(format!("Usage: `{}`", self.usage())),
                    None,
                    Some(message),
                )
                .await?;

            return Ok(());
        }

        fn extract_arg<'a>(
            args: &'a CommandArguments<'a>,
            arg_name: &'static str,
            current_arg: Option<&str>,
            clear: bool,
        ) -> DatabaseUpdate<Option<&'a str>> {
            if clear && current_arg.is_some_and(|it| it == arg_name) {
                return DatabaseUpdate::Set(None);
            }

            let Some(value) = get_argument_single(args, arg_name) else {
                return DatabaseUpdate::Keep;
            };

            if clear {
                return DatabaseUpdate::Set(None);
            }

            return DatabaseUpdate::Set(Some(value));
        }

        let description = extract_arg(
            args,
            SystemCommand::DESCRIPTION,
            self.subcommand,
            self.clear,
        );
        let tag = extract_arg(args, SystemCommand::TAG, self.subcommand, self.clear);
        let avatar_url = extract_arg(args, SystemCommand::AVATAR_URL, self.subcommand, self.clear);

        let name = get_argument_single(args, SystemCommand::NAME)
            .map_or(DatabaseUpdate::Keep, DatabaseUpdate::Set);

        A::update_system_by_id(context, system_id, name, tag, avatar_url, description).await?;
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some("System info updated! View it with `pl!system`".into()),
                    None,
                    Some(message),
                )
                .await?;

        return Ok(());
    }
}
