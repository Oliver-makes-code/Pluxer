use std::borrow::Cow;

use async_trait::async_trait;
use pluxer_backend::{
    PluxerApi,
    bot::BackendBot,
    id::BackendId,
    message::{BackendMessage, ReferencedMessageKind},
    user::BackendUser,
};

use crate::bot::{
    PluxerContext,
    command_parser::{
        CommandArguments, CommandExecutor, builder::CommandBuilder, get_argument_single,
        node::unix::UnixParameter,
    },
    commands::system::create::CreateSystemCommand,
};

pub enum MessageCommand {
    Delete,
    Edit,
}

impl MessageCommand {
    const EDIT_UNIX: &[UnixParameter] = &[
        UnixParameter::flag("append", &["append", "a"]),
        UnixParameter::flag("no_space", &["no_space", "nospace", "no-space", "ns"]),
    ];

    pub fn append<A: PluxerApi>(command: &mut CommandBuilder<PluxerContext<A>>) {
        command.literal(&["edit", "e"], |command| {
            command.unix(Self::EDIT_UNIX, |command| {
                command.executes(Self::Edit);
                command.greedy_string("content", |_| {});
            });
        });

        command.literal(&["delete", "d"], |command| {
            command.executes(Self::Delete);
        });
    }
}

#[async_trait]
impl<A: PluxerApi> CommandExecutor<PluxerContext<A>> for MessageCommand {
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
                    Some(&format!(
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

        let db_message = if let Some(referenced) = message.referenced_message() {
            context
                .database
                .fetch_message_by_id(context.get_platform_id(referenced.id()))
                .await?
        } else {
            context
                .database
                .fetch_latest_message_in_channel(
                    context.get_platform_id(message.channel_id()),
                    system_id,
                )
                .await?
        };

        let Some(db_message) = db_message else {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some("Unable to find message. Are you sure I proxied any messages?".into()),
                    None,
                    Some((ReferencedMessageKind::Reply, message)),
                    &[],
                )
                .await?;

            return Ok(());
        };

        let Some(channel_id) = A::Id::from_platform_id(&db_message.channel_id) else {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some("Unable to find message. Are you sure I proxied any messages?".into()),
                    None,
                    Some((ReferencedMessageKind::Reply, message)),
                    &[],
                )
                .await?;

            return Ok(());
        };

        let Some(message_id) = A::Id::from_platform_id(&db_message.message_id) else {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some("Unable to find message. Are you sure I proxied any messages?".into()),
                    None,
                    Some((ReferencedMessageKind::Reply, message)),
                    &[],
                )
                .await?;

            return Ok(());
        };

        if let MessageCommand::Delete = self {
            context
                .database
                .delete_message(db_message.message_id)
                .await?;

            context.bot.delete_message(&channel_id, &message_id).await?;

            context
                .bot
                .delete_message(message.channel_id(), message.id())
                .await?;

            return Ok(());
        }

        let Some(new_content) = get_argument_single(args, "content") else {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some("Edit command must have new content".into()),
                    None,
                    Some((ReferencedMessageKind::Reply, message)),
                    &[],
                )
                .await?;
            return Ok(());
        };

        let content = if get_argument_single(args, "append").is_some() {
            let message_to_edit = context.bot.fetch_message(&channel_id, &message_id).await?;

            Cow::Owned(format!(
                "{}{}{}",
                message_to_edit.content(),
                get_argument_single(args, "no_space")
                    .map(|_| "")
                    .unwrap_or(" "),
                new_content
            ))
        } else {
            Cow::Borrowed(new_content)
        };

        let webhook = context.fetch_webhook(&channel_id).await?;

        context
            .bot
            .edit_message_webhook(&webhook, &message_id, &content)
            .await?;
        context
            .bot
            .delete_message(message.channel_id(), message.id())
            .await?;

        return Ok(());
    }
}
