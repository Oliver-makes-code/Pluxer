use std::time::Duration;

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder,
    sea_query::prelude::Utc,
};
use ulid::Ulid;

use crate::{
    entities::{DatabaseId, message},
    handler::DatabaseHandler,
    model::message::MessageModel,
    platform_id::PlatformId,
};

impl DatabaseHandler {
    const MESSAGE_ALIVE_TIME: Duration = Duration::from_days(7);

    pub async fn clean_up_messages(&self) -> Result<(), DbErr> {
        message::Entity::delete_many()
            .filter(message::Column::ExpiresAt.lte(Utc::now()))
            .exec(&self.conn)
            .await?;

        return Ok(());
    }

    pub async fn create_message(
        &self,
        message_id: PlatformId,
        channel_id: PlatformId,
        user_id: PlatformId,
        system_id: Ulid,
        member_id: Ulid,
    ) -> Result<(), DbErr> {
        let created = Utc::now();

        let model = message::ActiveModel {
            message_id: ActiveValue::Set(message_id),
            user_id: ActiveValue::Set(user_id),
            channel_id: ActiveValue::Set(channel_id),

            system_id: ActiveValue::Set(system_id.into()),
            member_id: ActiveValue::Set(member_id.into()),

            created_at: ActiveValue::Set(created),
            expires_at: ActiveValue::Set(created + Self::MESSAGE_ALIVE_TIME),
        };

        model.insert(&self.conn).await?;

        return Ok(());
    }

    pub async fn fetch_latest_message_in_channel(
        &self,
        channel_id: PlatformId,
        system_id: Ulid,
    ) -> Result<Option<MessageModel>, DbErr> {
        let message = message::Entity::find()
            .filter(message::Column::SystemId.eq(DatabaseId::from(system_id)))
            .filter(message::Column::ChannelId.eq(channel_id))
            .order_by_desc(message::Column::CreatedAt)
            .one(&self.conn)
            .await?;

        return Ok(message.map(Into::into));
    }

    pub async fn fetch_message_by_id(
        &self,
        message_id: PlatformId,
    ) -> Result<Option<MessageModel>, DbErr> {
        let message = message::Entity::find_by_id(message_id)
            .one(&self.conn)
            .await?;

        return Ok(message.map(Into::into));
    }

    pub async fn bump_message_expiry(&self, message_id: PlatformId) -> Result<(), DbErr> {
        let expires = Utc::now() + Self::MESSAGE_ALIVE_TIME;

        let message = message::ActiveModel {
            message_id: ActiveValue::Set(message_id),
            expires_at: ActiveValue::Set(expires),
            ..Default::default()
        };

        message.update(&self.conn).await?;

        return Ok(());
    }

    pub async fn delete_message(&self, message_id: PlatformId) -> Result<(), DbErr> {
        message::Entity::delete_by_id(message_id)
            .exec(&self.conn)
            .await?;

        return Ok(());
    }
}
