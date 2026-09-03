use pluxer_backend::{bot::BackendBot, message::BackendMessage, user::BackendUser};
use pluxer_database::sea_orm::entity::prelude::async_trait::async_trait;

use crate::{
    bot::{
        PluxerContext,
        command_parser::{
            CommandArguments, CommandExecutor, builder::CommandBuilder, get_argument_single,
        },
        commands::{
            CLEAR_VARIANTS, CREATE_VARIANTS, PROXY, member::MemberCommand,
            system::create::CreateSystemCommand,
        },
    },
    database::DatabaseExtension,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ProxyCommandMode {
    Create,
    Clear,
}

pub struct MemberProxyCommand(Option<ProxyCommandMode>);

impl MemberProxyCommand {
    pub const USAGE: &str = "pl!member <name> proxy <create|clear> [prefix]text[suffix]";

    pub fn append<A: DatabaseExtension>(command: &mut CommandBuilder<PluxerContext<A>>) {
        command.literal(&["proxy", "p"], |command| {
            command.executes(MemberProxyCommand(None));

            command.literal(CREATE_VARIANTS, |command| {
                command.executes(MemberProxyCommand(Some(ProxyCommandMode::Create)));

                command.greedy_string(PROXY, |_| {});
            });

            command.literal(CLEAR_VARIANTS, |command| {
                command.executes(MemberProxyCommand(Some(ProxyCommandMode::Clear)));

                command.greedy_string(PROXY, |_| {});
            });
        });
    }
}

#[async_trait]
impl<A: DatabaseExtension> CommandExecutor<PluxerContext<A>> for MemberProxyCommand {
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
                    &[],
                )
                .await?;

            return Ok(());
        };

        let Some(name) = get_argument_single(args, MemberCommand::MEMBER_NAME) else {
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

        let Some(member_id) = MemberCommand::fetch_member_id(context, system_id, name).await?
        else {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some(format!(
                        "Unable to find member '{}'. Are you sure you typed in the name correctly?",
                        name
                    )),
                    None,
                    Some(message),
                    &[],
                )
                .await?;

            return Ok(());
        };

        let Some(mode) = self.0 else {
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

        let Some(proxy) = get_argument_single(args, PROXY) else {
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

        if !proxy.contains("text") {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some(format!(
                        "Proxy tag must contain 'text'\nUsage: `{}`",
                        Self::USAGE
                    )),
                    None,
                    Some(message),
                    &[],
                )
                .await?;

            return Ok(());
        }

        if proxy == "text" {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some(format!(
                        "Proxy tag must not be just 'text'\nUsage: `{}`",
                        Self::USAGE
                    )),
                    None,
                    Some(message),
                    &[],
                )
                .await?;

            return Ok(());
        }

        if let ProxyCommandMode::Create = mode {
            let has_proxy = A::has_system_proxy(context, system_id, proxy).await?;

            if has_proxy {
                context
                    .bot
                    .send_message(
                        message.channel_id(),
                        Some(format!(
                            "Proxy tag `{}` already exists in the system.",
                            proxy
                        )),
                        None,
                        Some(message),
                        &[],
                    )
                    .await?;

                return Ok(());
            }

            A::create_member_proxy(context, system_id, member_id, proxy.into()).await?;

            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some(format!("Proxy tag `{}` created.", proxy)),
                    None,
                    Some(message),
                    &[],
                )
                .await?;

            return Ok(());
        }

        let has_proxy = A::has_member_proxy(context, system_id, member_id, proxy).await?;

        if !has_proxy {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some(format!(
                        "Proxy tag `{}` already does not exist in the system.",
                        proxy
                    )),
                    None,
                    Some(message),
                    &[],
                )
                .await?;

            return Ok(());
        }

        A::delete_member_proxy(context, system_id, member_id, proxy).await?;

        context
            .bot
            .send_message(
                message.channel_id(),
                Some(format!("Proxy tag `{}` removed.", proxy)),
                None,
                Some(message),
                &[],
            )
            .await?;

        return Ok(());
    }
}
