use sea_orm::prelude::DateTimeUtc;
use ulid::Ulid;

use crate::{entities::message, platform_id::PlatformId};

pub struct MessageModel {
    pub message_id: PlatformId,
    pub user_id: PlatformId,
    pub channel_id: PlatformId,

    pub system_id: Ulid,
    pub member_id: Ulid,

    pub created_at: DateTimeUtc,
    pub expires_at: DateTimeUtc,
}

impl From<message::Model> for MessageModel {
    fn from(value: message::Model) -> Self {
        return Self {
            message_id: value.message_id,
            user_id: value.user_id,
            channel_id: value.channel_id,
            system_id: value.system_id.into(),
            member_id: value.member_id.into(),
            created_at: value.created_at,
            expires_at: value.expires_at,
        };
    }
}
