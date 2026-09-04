use std::ops::Deref;

use async_trait::async_trait;
use pluxer_backend::{PluxerApi, bot::BackendBot, message::BackendMessage, user::BackendUser};
use pluxer_database::handler::DatabaseUpdate;

use crate::bot::{
    PluxerContext,
    command_parser::{
        CommandArguments, CommandExecutor, builder::CommandBuilder, get_argument_single,
        node::unix::UnixParameter,
    },
    commands::{
        AVATAR_URL, AVATAR_URL_VARIANTS, CLEAR_VARIANTS, COLOR, COLOR_VARIANTS, DESCRIPTION,
        DESCRIPTION_VARIANTS, DISPLAY_NAME, DISPLAY_NAME_VARIANTS, NAME, NAME_VARIANTS, PRONOUNS,
        PRONOUNS_VARIANTS, TAG, TAG_VARIANTS, UPDATE_VARIANTS, extract_arg, parse_color_rgb,
        system::create::CreateSystemCommand,
    },
};

pub struct UpdateSystemCommand {
    subcommand: Option<&'static str>,
    clear: bool,
}

impl UpdateSystemCommand {
    const UNIX_PARAMETERS_SET: &[UnixParameter] = &[
        UnixParameter::value(NAME, NAME_VARIANTS),
        UnixParameter::value(TAG, TAG_VARIANTS),
        UnixParameter::value(AVATAR_URL, AVATAR_URL_VARIANTS),
        UnixParameter::value(DESCRIPTION, DESCRIPTION_VARIANTS),
        UnixParameter::value(PRONOUNS, PRONOUNS_VARIANTS),
        UnixParameter::value(DISPLAY_NAME, DISPLAY_NAME_VARIANTS),
        UnixParameter::value(COLOR, COLOR_VARIANTS),
    ];
    const UNIX_PARAMETERS_CLEAR: &[UnixParameter] = &[
        UnixParameter::flag(TAG, TAG_VARIANTS),
        UnixParameter::flag(AVATAR_URL, AVATAR_URL_VARIANTS),
        UnixParameter::flag(DESCRIPTION, DESCRIPTION_VARIANTS),
        UnixParameter::flag(PRONOUNS, PRONOUNS_VARIANTS),
        UnixParameter::flag(DISPLAY_NAME, DISPLAY_NAME_VARIANTS),
        UnixParameter::flag(COLOR, COLOR_VARIANTS),
    ];

    fn usage(&self) -> String {
        let Some(subcommand) = self.subcommand else {
            return "pl!system update [--tag=\"<tag>\"] [--avatar_url=\"<avatar_url>\"] [--description=\"<description>\"] [--pronouns=\"<pronouns>\"] [--display_name=\"<display_name>\"] [--name=\"<name>\"]".into();
        };

        return format!("pl!system {0} <{0}>", subcommand);
    }

    pub fn append<A: PluxerApi>(command: &mut CommandBuilder<PluxerContext<A>>) {
        command.literal(UPDATE_VARIANTS, |command| {
            command.executes(UpdateSystemCommand {
                subcommand: None,
                clear: false,
            });

            command.unix(&Self::UNIX_PARAMETERS_SET, |_| {});
        });

        command.literal(CLEAR_VARIANTS, |command| {
            command.executes(UpdateSystemCommand {
                subcommand: None,
                clear: true,
            });

            command.unix(&Self::UNIX_PARAMETERS_CLEAR, |_| {});
        });

        const SUBCOMMANDS: &[(&str, &[&str], bool)] = &[
            (NAME, NAME_VARIANTS, false),
            (TAG, TAG_VARIANTS, true),
            (AVATAR_URL, AVATAR_URL_VARIANTS, true),
            (DESCRIPTION, DESCRIPTION_VARIANTS, true),
            (DISPLAY_NAME, DISPLAY_NAME_VARIANTS, true),
            (PRONOUNS, PRONOUNS_VARIANTS, true),
            (COLOR, COLOR_VARIANTS, true),
        ];

        for (name, variants, allow_clear) in SUBCOMMANDS {
            command.literal(variants, |command| {
                if *allow_clear {
                    command.literal(CLEAR_VARIANTS, |command| {
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
impl<A: PluxerApi> CommandExecutor<PluxerContext<A>> for UpdateSystemCommand {
    async fn execute<'a>(
        &self,
        args: &'a CommandArguments<'a>,
        context: &'a PluxerContext<A>,
        message: &A::Message,
    ) -> anyhow::Result<()> {
        let Some(system_id) = context
            .database
            .fetch_system_id(context.get_platform_id(message.author().id()))
            .await?
        else {
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
                    &[],
                )
                .await?;

            return Ok(());
        };

        let has_avatar_image = message.attachments().next().is_some();

        if !(self.subcommand == Some(AVATAR_URL) && has_avatar_image)
            && !self.clear
            && args.is_empty()
        {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some(format!("Usage: `{}`", self.usage())),
                    None,
                    Some(message),
                    &[],
                )
                .await?;

            return Ok(());
        }

        let description = extract_arg(args, DESCRIPTION, self.subcommand, self.clear);
        let tag = extract_arg(args, TAG, self.subcommand, self.clear);
        let mut avatar_url = extract_arg(args, AVATAR_URL, self.subcommand, self.clear);

        let attachment = message.attachments().next();
        let attachment = attachment.as_ref().map(|it| it.file_url.deref());

        if let Some(attachment) = attachment
            && self.subcommand.is_some_and(|it| it == AVATAR_URL)
        {
            avatar_url = match avatar_url {
                DatabaseUpdate::Keep => DatabaseUpdate::Set(Some(attachment)),
                DatabaseUpdate::Set(old) => DatabaseUpdate::Set(old.or(Some(attachment))),
            };
        }

        let pronouns = extract_arg(args, PRONOUNS, self.subcommand, self.clear);
        let display_name = extract_arg(args, DISPLAY_NAME, self.subcommand, self.clear);
        let color = extract_arg(args, COLOR, self.subcommand, self.clear)
            .map(|it| it.and_then(parse_color_rgb));

        let name =
            get_argument_single(args, NAME).map_or(DatabaseUpdate::Keep, DatabaseUpdate::Set);

        context
            .database
            .update_system_by_id(
                system_id,
                name,
                tag,
                display_name,
                avatar_url,
                description,
                pronouns,
                color,
            )
            .await?;

        context
            .bot
            .send_message(
                message.channel_id(),
                Some("System info updated! View it with `pl!system`".into()),
                None,
                Some(message),
                &[],
            )
            .await?;

        return Ok(());
    }
}
