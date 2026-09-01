use pluxer_backend::{bot::BackendBot, message::BackendMessage, user::BackendUser};
use pluxer_database::sea_orm::entity::prelude::async_trait::async_trait;

use crate::{
    bot::{
        PluxerContext,
        command_parser::{
            CommandArguments, CommandExecutor, builder::CommandBuilder, get_argument_single,
            node::unix::UnixParameter,
        },
        commands::{
            AVATAR_URL, AVATAR_URL_VARIANTS, CLEAR_VARIANTS, COLOR, COLOR_VARIANTS, DESCRIPTION,
            DESCRIPTION_VARIANTS, DISPLAY_NAME, DISPLAY_NAME_VARIANTS, NAME, NAME_VARIANTS,
            PRONOUNS, PRONOUNS_VARIANTS, UPDATE_VARIANTS, extract_arg, member::MemberCommand,
            parse_color_rgb, system::create::CreateSystemCommand,
        },
    },
    database::{DatabaseExtension, DatabaseUpdate},
};

pub struct UpdateMemberCommand {
    subcommand: Option<&'static str>,
    clear: bool,
}

impl UpdateMemberCommand {
    const MEMBER_NAME: &str = "member_name";

    const UNIX_PARAMETERS_SET: &[UnixParameter] = &[
        UnixParameter::value(NAME, NAME_VARIANTS),
        UnixParameter::value(AVATAR_URL, AVATAR_URL_VARIANTS),
        UnixParameter::value(DESCRIPTION, DESCRIPTION_VARIANTS),
        UnixParameter::value(PRONOUNS, PRONOUNS_VARIANTS),
        UnixParameter::value(DISPLAY_NAME, DISPLAY_NAME_VARIANTS),
        UnixParameter::value(COLOR, COLOR_VARIANTS),
    ];
    const UNIX_PARAMETERS_CLEAR: &[UnixParameter] = &[
        UnixParameter::flag(AVATAR_URL, AVATAR_URL_VARIANTS),
        UnixParameter::flag(DESCRIPTION, DESCRIPTION_VARIANTS),
        UnixParameter::flag(PRONOUNS, PRONOUNS_VARIANTS),
        UnixParameter::flag(DISPLAY_NAME, DISPLAY_NAME_VARIANTS),
        UnixParameter::flag(COLOR, COLOR_VARIANTS),
    ];

    fn usage(&self) -> String {
        let Some(subcommand) = self.subcommand else {
            if self.clear {
                return "pl!member clear [--tag=\"<tag>\"] [--avatar_url=\"<avatar_url>\"] [--description=\"<description>\"] [--pronouns=\"<pronouns>\"] [--display_name=\"<display_name>\"] [--name=\"<name>\"] <name>".into();
            }

            return "pl!member update [--tag=\"<tag>\"] [--avatar_url=\"<avatar_url>\"] [--description=\"<description>\"] [--pronouns=\"<pronouns>\"] [--display_name=\"<display_name>\"] [--name=\"<name>\"] <name>".into();
        };

        return format!(
            "pl!maember {0} <name>{1} <{0}>",
            subcommand,
            if self.clear { " clear" } else { "" }
        );
    }

    pub fn append<A: DatabaseExtension>(command: &mut CommandBuilder<PluxerContext<A>>) {
        command.literal(UPDATE_VARIANTS, |command| {
            command.executes(UpdateMemberCommand {
                subcommand: None,
                clear: false,
            });

            command.unix(&Self::UNIX_PARAMETERS_SET, |command| {
                command.greedy_string(Self::MEMBER_NAME, |_| {});
            });
        });

        command.literal(CLEAR_VARIANTS, |command| {
            command.executes(UpdateMemberCommand {
                subcommand: None,
                clear: true,
            });

            command.unix(&Self::UNIX_PARAMETERS_CLEAR, |_| {});
        });

        const SUBCOMMANDS: &[(&str, &[&str], bool)] = &[
            (NAME, NAME_VARIANTS, false),
            (AVATAR_URL, AVATAR_URL_VARIANTS, true),
            (DESCRIPTION, DESCRIPTION_VARIANTS, true),
            (DISPLAY_NAME, DISPLAY_NAME_VARIANTS, true),
            (PRONOUNS, PRONOUNS_VARIANTS, true),
            (COLOR, COLOR_VARIANTS, true),
        ];

        for (name, variants, allow_clear) in SUBCOMMANDS {
            command.literal(variants, |command| {
                command.string(Self::MEMBER_NAME, |command| {
                    if *allow_clear {
                        command.literal(CLEAR_VARIANTS, |command| {
                            command.executes(UpdateMemberCommand {
                                subcommand: Some(name),
                                clear: true,
                            });
                        });
                    }

                    command.greedy_string(name, |_| {});
                });

                command.executes(UpdateMemberCommand {
                    subcommand: Some(name),
                    clear: false,
                });
            });
        }
    }
}

#[async_trait]
impl<A: DatabaseExtension> CommandExecutor<PluxerContext<A>> for UpdateMemberCommand {
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

        let Some(member_name) = get_argument_single(args, Self::MEMBER_NAME) else {
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
        };

        let Some(member) = MemberCommand::fetch_member(context, system_id, member_name).await?
        else {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some(format!(
                        "Unable to find member '{}'. Are you sure you typed in the name correctly?",
                        member_name
                    )),
                    None,
                    Some(message),
                )
                .await?;

            return Ok(());
        };

        if !self.clear && args.len() == 1 {
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

        let description = extract_arg(args, DESCRIPTION, self.subcommand, self.clear);
        let avatar_url = extract_arg(args, AVATAR_URL, self.subcommand, self.clear);
        let pronouns = extract_arg(args, PRONOUNS, self.subcommand, self.clear);
        let display_name = extract_arg(args, DISPLAY_NAME, self.subcommand, self.clear);
        let color = extract_arg(args, COLOR, self.subcommand, self.clear)
            .map(|it| it.and_then(parse_color_rgb));

        let name =
            get_argument_single(args, NAME).map_or(DatabaseUpdate::Keep, DatabaseUpdate::Set);

        A::update_member_by_id(
            context,
            member.id,
            name,
            display_name,
            pronouns,
            avatar_url,
            description,
            color,
        )
        .await?;

        context
            .bot
            .send_message(
                message.channel_id(),
                Some("System info updated! View them with `pl!member`".into()),
                None,
                Some(message),
            )
            .await?;

        return Ok(());
    }
}
