use std::time::Duration;

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, QueryFilter,
    sea_query::prelude::Utc,
};
use ulid::Ulid;

use crate::{entities::message, handler::DatabaseHandler, platform_id::PlatformId};

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
}
