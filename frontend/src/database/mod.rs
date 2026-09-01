use pluxer_backend::PluxerApi;
use pluxer_database::{
    entities::{DatabaseId, system},
    model::system::SystemModel,
    sea_orm::{
        ActiveModelTrait, ActiveValue, DbErr, EntityTrait,
        entity::prelude::async_trait::async_trait, sqlx::types::chrono::DateTime,
    },
};
use ulid::Ulid;

use crate::bot::PluxerContext;

#[cfg(feature = "fluxer")]
mod fluxer;

#[async_trait]
pub trait DatabaseExtension: PluxerApi + Sized {
    async fn fetch_system_id(
        context: &PluxerContext<Self>,
        user_id: &<Self as PluxerApi>::Id,
    ) -> Result<Option<Ulid>, DbErr>;

    async fn set_system_id(
        context: &PluxerContext<Self>,
        user_id: &<Self as PluxerApi>::Id,
        system_id: Option<Ulid>,
    ) -> Result<(), DbErr>;

    async fn create_system(
        context: &PluxerContext<Self>,
        user_id: &<Self as PluxerApi>::Id,
        name: &str,
        avatar_url: Option<&str>,
        tag: Option<&str>,
    ) -> Result<Ulid, DbErr> {
        let system_id = Ulid::generate();

        let system = system::ActiveModel {
            id: ActiveValue::Set(system_id.into()),
            name: ActiveValue::Set(name.to_string()),
            avatar_url: ActiveValue::Set(avatar_url.map(ToString::to_string)),
            tag: ActiveValue::Set(tag.map(ToString::to_string)),

            created_at: ActiveValue::Set(DateTime::default()),
            updated_at: ActiveValue::Set(DateTime::default()),

            ..Default::default()
        };

        system.insert(&context.database_connection).await?;

        Self::set_system_id(context, user_id, Some(system_id)).await?;

        return Ok(system_id);
    }

    async fn fetch_system_by_id(
        context: &PluxerContext<Self>,
        system_id: Ulid,
    ) -> Result<Option<SystemModel>, DbErr> {
        let system = system::Entity::find_by_id(DatabaseId::from(system_id))
            .one(&context.database_connection)
            .await?;

        return Ok(system.map(Into::into));
    }

    async fn fetch_system_by_user(
        context: &PluxerContext<Self>,
        user_id: &<Self as PluxerApi>::Id,
    ) -> Result<Option<SystemModel>, DbErr> {
        let Some(system_id) = Self::fetch_system_id(context, user_id).await? else {
            return Ok(None);
        };

        return Self::fetch_system_by_id(context, system_id).await;
    }
}
