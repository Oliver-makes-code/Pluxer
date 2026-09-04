use async_trait::async_trait;
use pluxer_backend::{
    PluxerApi,
    bot::BackendBot,
    message::{BackendMessage, ReferencedMessageKind},
    user::BackendUser,
};

use crate::bot::{
    PluxerContext,
    command_parser::{
        CommandArguments, CommandExecutor, builder::CommandBuilder, get_argument_single,
        node::unix::UnixParameter,
    },
    commands::{
        AVATAR_URL, AVATAR_URL_VARIANTS, COLOR, COLOR_VARIANTS, CREATE_VARIANTS, DESCRIPTION,
        DESCRIPTION_VARIANTS, DISPLAY_NAME, DISPLAY_NAME_VARIANTS, NAME, PRONOUNS,
        PRONOUNS_VARIANTS, member::MemberCommand, parse_color_rgb,
        system::create::CreateSystemCommand, u32_to_base64,
    },
};

pub struct CreateMemberCommand;

impl CreateMemberCommand {
    const UNIX_PARAMETERS: &[UnixParameter] = &[
        UnixParameter::value(DISPLAY_NAME, DISPLAY_NAME_VARIANTS),
        UnixParameter::value(AVATAR_URL, AVATAR_URL_VARIANTS),
        UnixParameter::value(DESCRIPTION, DESCRIPTION_VARIANTS),
        UnixParameter::value(PRONOUNS, PRONOUNS_VARIANTS),
        UnixParameter::value(COLOR, COLOR_VARIANTS),
    ];

    pub const USAGE: &str = "pl!member create [--avatar_url=\"<avatar_url>\"] [--description=\"<description>\"] [--pronouns=\"<pronouns>\"] [--display_name=\"<display_name>\"] [--color=\"<color>\"] <name>";

    pub fn append<A: PluxerApi>(command: &mut CommandBuilder<PluxerContext<A>>) {
        command.literal(CREATE_VARIANTS, |command| {
            command.executes(CreateMemberCommand);

            command.unix(Self::UNIX_PARAMETERS, |command| {
                command.greedy_string(NAME, |_| {});
            });
        });
    }
}

#[async_trait]
impl<A: PluxerApi> CommandExecutor<PluxerContext<A>> for CreateMemberCommand {
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
                    Some((ReferencedMessageKind::Reply, message)),
                    &[],
                )
                .await?;

            return Ok(());
        };

        let Some(name) = get_argument_single(args, NAME) else {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some(format!("Usage: `{}`", Self::USAGE)),
                    None,
                    Some((ReferencedMessageKind::Reply, message)),
                    &[],
                )
                .await?;

            return Ok(());
        };

        if context
            .database
            .fetch_member_by_name(system_id, name)
            .await?
            .is_some()
        {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some(format!("You already have a member named {}.", name)),
                    None,
                    Some((ReferencedMessageKind::Reply, message)),
                    &[],
                )
                .await?;

            return Ok(());
        }

        let description = get_argument_single(args, DESCRIPTION);

        let pronouns = get_argument_single(args, PRONOUNS);

        let display_name = get_argument_single(args, DISPLAY_NAME);

        let avatar_url = get_argument_single(args, AVATAR_URL);

        let color = get_argument_single(args, COLOR).and_then(parse_color_rgb);

        let (member_id, member_id_hash) = context
            .database
            .create_member(
                system_id,
                name,
                display_name,
                description,
                pronouns,
                avatar_url,
                color,
            )
            .await?;

        context
            .bot
            .send_message(
                message.channel_id(),
                Some(format!("Member created! View them with `{}`\n-# Member ID: {}\n-# Member Shorthand ID: {}", MemberCommand::USAGE, member_id, u32_to_base64(member_id_hash))),
                None,
                Some((ReferencedMessageKind::Reply, message)),
                    &[],
            )
            .await?;

        return Ok(());
    }
}
