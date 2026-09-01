use pluxer_backend::{
    bot::BackendBot,
    embed::{Embed, EmbedField},
    message::BackendMessage,
    user::BackendUser,
};
use pluxer_database::{
    model::member::MemberModel, sea_orm::entity::prelude::async_trait::async_trait,
};
use ulid::Ulid;

use crate::{
    bot::{
        PluxerContext,
        command_parser::{
            CommandArguments, CommandExecutor, builder::CommandBuilder, get_argument_single,
        },
        commands::{
            NAME, base64_to_u32,
            member::{
                create::CreateMemberCommand, delete::DeleteMemberCommand,
                update::UpdateMemberCommand,
            },
            system::create::CreateSystemCommand,
            u32_to_base64,
        },
    },
    database::DatabaseExtension,
};

pub mod create;
pub mod delete;
pub mod update;

pub struct MemberCommand;

impl MemberCommand {
    pub const USAGE: &str = "pl!member <name>";

    pub fn append<A: DatabaseExtension>(command: &mut CommandBuilder<PluxerContext<A>>) {
        command.literal(&["member", "m"], |command| {
            command.executes(MemberCommand);

            CreateMemberCommand::append(command);
            DeleteMemberCommand::append(command);
            UpdateMemberCommand::append(command);

            command.greedy_string("name", |_| {});
        });
    }

    pub fn member_to_embed(member: MemberModel) -> Embed {
        let mut description = vec![];

        member.description.map(|it| description.push(it));

        member
            .pronouns
            .map(|it| description.push(format!("\n**Pronouns:** {}", it)));

        description.push(format!("\n-# Unique Member Name: `{}`", member.name));
        description.push(format!("-# Member ID: `{}`", member.id));
        description.push(format!(
            "-# Member Shorthand ID: `{}`",
            u32_to_base64(member.id_hash)
        ));
        description.push(format!("-# System ID: `{}`", member.system_id));

        return Embed {
            title: Some(member.display_name.unwrap_or(member.name)),
            description: Some(description.join("\n")),
            color: member.color.unwrap_or(0),
            thumbnail_url: member.avatar_url,
            ..Default::default()
        };
    }

    async fn fetch_member<'a, A: DatabaseExtension>(
        context: &'a PluxerContext<A>,
        system_id: Ulid,
        name: &str,
    ) -> anyhow::Result<Option<MemberModel>> {
        if let Some(id_hash) = base64_to_u32(name) {
            if let Some(member) = A::fetch_member_by_hash(context, system_id, id_hash).await? {
                return Ok(Some(member));
            }
        }

        if let Some(member_id) = Ulid::try_from(name).ok() {
            if let Some(member) = A::fetch_member_by_id(context, system_id, member_id).await? {
                return Ok(Some(member));
            }
        }

        return Ok(A::fetch_member_by_name(context, system_id, name).await?);
    }
}

#[async_trait]
impl<A: DatabaseExtension> CommandExecutor<PluxerContext<A>> for MemberCommand {
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

        let Some(name) = get_argument_single(args, NAME) else {
            context
                .bot
                .send_message(
                    message.channel_id(),
                    Some(format!("Usage: `{}`", Self::USAGE)),
                    None,
                    Some(message),
                )
                .await?;

            return Ok(());
        };

        let Some(member) = Self::fetch_member(context, system_id, name).await? else {
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
                )
                .await?;

            return Ok(());
        };

        context
            .bot
            .send_message(
                message.channel_id(),
                None,
                Some(Self::member_to_embed(member)),
                Some(message),
            )
            .await?;

        return Ok(());
    }
}
