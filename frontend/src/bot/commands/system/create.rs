use async_trait::async_trait;
use pluxer_backend::{PluxerApi, bot::BackendBot, message::BackendMessage, user::BackendUser};

use crate::bot::{
    PluxerContext,
    command_parser::{
        CommandArguments, CommandExecutor, builder::CommandBuilder, get_argument_single,
        node::unix::UnixParameter,
    },
    commands::{
        AVATAR_URL, AVATAR_URL_VARIANTS, COLOR, COLOR_VARIANTS, CREATE_VARIANTS, DESCRIPTION,
        DESCRIPTION_VARIANTS, DISPLAY_NAME, DISPLAY_NAME_VARIANTS, NAME, PRONOUNS,
        PRONOUNS_VARIANTS, TAG, TAG_VARIANTS, parse_color_rgb,
    },
};

pub struct CreateSystemCommand;

impl CreateSystemCommand {
    const UNIX_PARAMETERS: &[UnixParameter] = &[
        UnixParameter::value(TAG, TAG_VARIANTS),
        UnixParameter::value(AVATAR_URL, AVATAR_URL_VARIANTS),
        UnixParameter::value(DESCRIPTION, DESCRIPTION_VARIANTS),
        UnixParameter::value(DISPLAY_NAME, DISPLAY_NAME_VARIANTS),
        UnixParameter::value(PRONOUNS, PRONOUNS_VARIANTS),
        UnixParameter::value(COLOR, COLOR_VARIANTS),
    ];

    pub const USAGE: &str = "pl!system create [--tag=\"<tag>\"] [--avatar_url=\"<avatar_url>\"] [--description=\"<description>\"] [--pronouns=\"<pronouns>\"] [--display_name=\"<display_name>\"] [--color=\"<color>\"] <name>";

    pub fn append<A: PluxerApi>(command: &mut CommandBuilder<PluxerContext<A>>) {
        command.literal(CREATE_VARIANTS, |command| {
            command.executes(CreateSystemCommand);

            command.unix(&Self::UNIX_PARAMETERS, |command| {
                command.greedy_string(NAME, |_| {});
            });
        });
    }
}

#[async_trait]
impl<A: PluxerApi> CommandExecutor<PluxerContext<A>> for CreateSystemCommand {
    async fn execute<'a>(
        &self,
        args: &'a CommandArguments<'a>,
        context: &'a PluxerContext<A>,
        message: &A::Message,
    ) -> anyhow::Result<()> {
        let system_id = context
            .database
            .fetch_system_id(context.get_platform_id(message.author().id()))
            .await?;

        if system_id.is_some() {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some("You have already created a system. View it with `pl!system`".into()),
                    None,
                    Some(message),
                    &[],
                )
                .await?;

            return Ok(());
        }

        let Some(name) = get_argument_single(args, NAME) else {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some(format!("Usage: `{}`", Self::USAGE)),
                    None,
                    Some(message),
                    &[],
                )
                .await?;

            return Ok(());
        };

        let display_name = get_argument_single(args, DISPLAY_NAME);

        let pronouns = get_argument_single(args, PRONOUNS);

        let description = get_argument_single(args, DESCRIPTION);

        let tag = get_argument_single(args, TAG);

        let avatar_url = get_argument_single(args, AVATAR_URL);

        let color = get_argument_single(args, COLOR).and_then(parse_color_rgb);

        let system_id = context
            .database
            .create_system(
                context.get_platform_id(message.author().id()),
                name,
                display_name,
                description,
                tag,
                pronouns,
                avatar_url,
                color,
            )
            .await?;

        context
            .bot
            .send_message(
                message.channel_id(),
                Some(format!(
                    "System created! View it with `pl!system`\n-# System ID: {}",
                    system_id
                )),
                None,
                Some(message),
                &[],
            )
            .await?;

        return Ok(());
    }
}
