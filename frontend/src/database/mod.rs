use pluxer_backend::PluxerApi;
use pluxer_database::{
    entities::{DatabaseId, system, user},
    model::system::SystemModel,
    sea_orm::{
        ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, PaginatorTrait,
        QueryFilter, entity::prelude::async_trait::async_trait, sqlx::types::chrono::DateTime,
    },
};
use ulid::Ulid;

use crate::bot::PluxerContext;

#[cfg(feature = "fluxer")]
mod fluxer;

pub enum DatabaseUpdate<T> {
    Keep,
    Set(T),
}

impl<T> DatabaseUpdate<T> {
    fn map<U>(self, f: impl FnOnce(T) -> U) -> DatabaseUpdate<U> {
        match self {
            Self::Keep => return DatabaseUpdate::Keep,
            Self::Set(value) => return DatabaseUpdate::Set(f(value)),
        }
    }
}

impl<T> From<DatabaseUpdate<T>> for ActiveValue<T>
where
    pluxer_database::sea_orm::Value: From<T>,
{
    fn from(value: DatabaseUpdate<T>) -> Self {
        match value {
            DatabaseUpdate::Keep => return ActiveValue::NotSet,
            DatabaseUpdate::Set(value) => return ActiveValue::Set(value),
        }
    }
}

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
        description: Option<&str>,
        tag: Option<&str>,
        avatar_url: Option<&str>,
    ) -> Result<Ulid, DbErr> {
        let system_id = Ulid::generate();

        let system = system::ActiveModel {
            id: ActiveValue::Set(system_id.into()),
            name: ActiveValue::Set(name.to_string()),
            avatar_url: ActiveValue::Set(avatar_url.map(ToString::to_string)),
            tag: ActiveValue::Set(tag.map(ToString::to_string)),
            description: ActiveValue::Set(description.map(ToString::to_string)),

            created_at: ActiveValue::Set(DateTime::default()),
            updated_at: ActiveValue::Set(DateTime::default()),

            ..Default::default()
        };

        system.insert(&context.database_connection).await?;

        Self::set_system_id(context, user_id, Some(system_id)).await?;

        return Ok(system_id);
    }

    /// Returns true if the system was deleted.
    async fn detach_or_delete_system(
        context: &PluxerContext<Self>,
        user_id: &<Self as PluxerApi>::Id,
        system_id: Ulid,
    ) -> Result<bool, DbErr> {
        Self::set_system_id(context, user_id, None).await?;

        let fluxer_count = user::fluxer::Entity::find()
            .filter(user::fluxer::Column::SystemId.eq(DatabaseId::from(system_id)))
            .count(&context.database_connection)
            .await?;

        if fluxer_count != 0 {
            return Ok(false);
        }

        system::Entity::delete_by_id(DatabaseId::from(system_id))
            .exec(&context.database_connection)
            .await?;

        return Ok(true);
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

    async fn update_system_by_id(
        context: &PluxerContext<Self>,
        system_id: Ulid,
        name: DatabaseUpdate<&str>,
        tag: DatabaseUpdate<Option<&str>>,
        avatar_url: DatabaseUpdate<Option<&str>>,
        description: DatabaseUpdate<Option<&str>>,
    ) -> Result<(), DbErr> {
        let system = system::ActiveModel {
            id: ActiveValue::Set(DatabaseId::from(system_id)),

            name: name.map(ToString::to_string).into(),
            avatar_url: avatar_url.map(|it| it.map(ToString::to_string)).into(),
            tag: tag.map(|it| it.map(ToString::to_string)).into(),
            description: description.map(|it| it.map(ToString::to_string)).into(),

            updated_at: ActiveValue::Set(DateTime::default()),

            ..Default::default()
        };

        system.update(&context.database_connection).await?;

        return Ok(());
    }

    async fn update_system_by_user(
        context: &PluxerContext<Self>,
        user_id: &<Self as PluxerApi>::Id,
        name: DatabaseUpdate<&str>,
        tag: DatabaseUpdate<Option<&str>>,
        avatar_url: DatabaseUpdate<Option<&str>>,
        description: DatabaseUpdate<Option<&str>>,
    ) -> Result<(), DbErr> {
        let Some(system_id) = Self::fetch_system_id(context, user_id).await? else {
            return Ok(());
        };

        Self::update_system_by_id(context, system_id, name, tag, avatar_url, description).await?;

        return Ok(());
    }
}
