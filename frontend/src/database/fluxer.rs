use std::ops::Deref;

use pluxer_backend::{PluxerApi, fluxer::FluxerApi, id::BackendId};
use pluxer_database::{
    entities::{DatabaseSnowflake, user},
    sea_orm::{
        ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
        QueryFilter, entity::prelude::async_trait::async_trait,
    },
};
use ulid::Ulid;

use crate::{bot::PluxerContext, database::DatabaseExtension};

#[async_trait]
impl DatabaseExtension for FluxerApi {
    async fn get_system_id(
        context: &PluxerContext<Self>,
        user_id: &<Self as PluxerApi>::Id,
    ) -> Result<Option<Ulid>, DbErr> {
        let Some(snowflake) = user_id.as_snowflake() else {
            return Ok(None);
        };

        let user_id = DatabaseSnowflake(snowflake as i64);

        let user = user::fluxer::Entity::find_by_id((user_id, context.instance_url.to_string()))
            .one(&context.database_connection)
            .await?;

        return Ok(user.map(|it| it.system_id.into()));
    }

    async fn set_system_id(
        context: &PluxerContext<Self>,
        user_id: &<Self as PluxerApi>::Id,
        system_id: Option<Ulid>,
    ) -> Result<(), DbErr> {
        let Some(snowflake) = user_id.as_snowflake() else {
            return Ok(());
        };

        let user_id = DatabaseSnowflake(snowflake as i64);

        let Some(system_id) = system_id else {
            user::fluxer::Entity::delete_by_id((user_id, context.instance_url.to_string()))
                .exec(&context.database_connection)
                .await?;

            return Ok(());
        };

        let model = user::fluxer::ActiveModel {
            id: ActiveValue::Set(user_id),
            instance_url: ActiveValue::Set(context.instance_url.to_string()),
            system_id: ActiveValue::Set(system_id.into()),
        };

        model.insert(&context.database_connection).await?;

        return Ok(());
    }
}
