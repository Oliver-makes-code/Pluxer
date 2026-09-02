use pluxer_backend::PluxerApi;
use pluxer_database::{
    entities::{DatabaseId, member, proxy, system, user},
    model::{member::MemberModel, system::SystemModel},
    sea_orm::{
        ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, PaginatorTrait,
        QueryFilter, QuerySelect, entity::prelude::async_trait::async_trait,
        sqlx::types::chrono::Utc,
    },
};
use ulid::Ulid;
use xxhash_rust::xxh3::xxh3_64;

use crate::bot::PluxerContext;

#[cfg(feature = "fluxer")]
mod fluxer;

pub enum DatabaseUpdate<T> {
    Keep,
    Set(T),
}

impl<T> DatabaseUpdate<T> {
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> DatabaseUpdate<U> {
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
        display_name: Option<&str>,
        description: Option<&str>,
        pronouns: Option<&str>,
        tag: Option<&str>,
        avatar_url: Option<&str>,
        color: Option<u32>,
    ) -> Result<Ulid, DbErr> {
        let system_id = Ulid::generate();

        let system = system::ActiveModel {
            id: ActiveValue::Set(system_id.into()),
            name: ActiveValue::Set(name.to_string()),
            display_name: ActiveValue::Set(display_name.map(ToString::to_string)),
            avatar_url: ActiveValue::Set(avatar_url.map(ToString::to_string)),
            tag: ActiveValue::Set(tag.map(ToString::to_string)),
            description: ActiveValue::Set(description.map(ToString::to_string)),
            pronouns: ActiveValue::Set(pronouns.map(ToString::to_string)),
            color: ActiveValue::Set(color.map(|it| it as i32)),
            timezone: ActiveValue::NotSet,

            created_at: ActiveValue::Set(Utc::now()),
            updated_at: ActiveValue::Set(Utc::now()),
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

    async fn update_system_by_id(
        context: &PluxerContext<Self>,
        system_id: Ulid,
        name: DatabaseUpdate<&str>,
        tag: DatabaseUpdate<Option<&str>>,
        display_name: DatabaseUpdate<Option<&str>>,
        avatar_url: DatabaseUpdate<Option<&str>>,
        description: DatabaseUpdate<Option<&str>>,
        pronouns: DatabaseUpdate<Option<&str>>,
        color: DatabaseUpdate<Option<u32>>,
    ) -> Result<(), DbErr> {
        let system = system::ActiveModel {
            id: ActiveValue::Set(DatabaseId::from(system_id)),

            name: name.map(ToString::to_string).into(),
            avatar_url: avatar_url.map(|it| it.map(ToString::to_string)).into(),
            tag: tag.map(|it| it.map(ToString::to_string)).into(),
            description: description.map(|it| it.map(ToString::to_string)).into(),
            pronouns: pronouns.map(|it| it.map(ToString::to_string)).into(),
            display_name: display_name.map(|it| it.map(ToString::to_string)).into(),
            color: color.map(|it| it.map(|it| it as i32)).into(),

            updated_at: ActiveValue::Set(Utc::now()),

            ..Default::default()
        };

        system.update(&context.database_connection).await?;

        return Ok(());
    }

    async fn fetch_member_count(
        context: &PluxerContext<Self>,
        system_id: Ulid,
    ) -> Result<usize, DbErr> {
        return Ok(member::Entity::find()
            .filter(member::Column::SystemId.eq(DatabaseId::from(system_id)))
            .count(&context.database_connection)
            .await? as usize);
    }

    async fn create_member(
        context: &PluxerContext<Self>,
        system_id: Ulid,
        name: &str,
        display_name: Option<&str>,
        description: Option<&str>,
        pronouns: Option<&str>,
        avatar_url: Option<&str>,
        color: Option<u32>,
    ) -> Result<(Ulid, u32), DbErr> {
        let member_id = Ulid::generate();
        let member_id_hash = xxh3_64(&member_id.to_bytes()) as u32;

        let member = member::ActiveModel {
            id: ActiveValue::Set(DatabaseId::from(member_id)),

            id_hash: ActiveValue::Set(member_id_hash as i32),
            system_id: ActiveValue::Set(DatabaseId::from(system_id)),

            name: ActiveValue::Set(name.to_ascii_lowercase()),
            display_name: ActiveValue::Set(display_name.map(ToString::to_string)),

            description: ActiveValue::Set(description.map(ToString::to_string)),
            pronouns: ActiveValue::Set(pronouns.map(ToString::to_string)),
            avatar_url: ActiveValue::Set(avatar_url.map(ToString::to_string)),
            color: ActiveValue::Set(color.map(|it| it as i32)),

            created_at: ActiveValue::Set(Utc::now()),
            updated_at: ActiveValue::Set(Utc::now()),
        };

        member.insert(&context.database_connection).await?;

        return Ok((member_id, member_id_hash));
    }

    async fn fetch_member_id_by_name(
        context: &PluxerContext<Self>,
        system_id: Ulid,
        name: &str,
    ) -> Result<Option<Ulid>, DbErr> {
        let member: Option<DatabaseId> = member::Entity::find()
            .filter(member::Column::SystemId.eq(DatabaseId::from(system_id)))
            .filter(member::Column::Name.eq(name.to_ascii_lowercase()))
            .select_only()
            .column(member::Column::Id)
            .into_tuple()
            .one(&context.database_connection)
            .await?;

        return Ok(member.map(Into::into));
    }

    async fn fetch_member_by_name(
        context: &PluxerContext<Self>,
        system_id: Ulid,
        name: &str,
    ) -> Result<Option<MemberModel>, DbErr> {
        let member = member::Entity::find()
            .filter(member::Column::SystemId.eq(DatabaseId::from(system_id)))
            .filter(member::Column::Name.eq(name.to_ascii_lowercase()))
            .one(&context.database_connection)
            .await?;

        return Ok(member.map(Into::into));
    }

    async fn fetch_member_id_by_hash(
        context: &PluxerContext<Self>,
        system_id: Ulid,
        id_hash: u32,
    ) -> Result<Option<Ulid>, DbErr> {
        let member: Option<DatabaseId> = member::Entity::find()
            .filter(member::Column::SystemId.eq(DatabaseId::from(system_id)))
            .filter(member::Column::IdHash.eq(id_hash as i32))
            .select_only()
            .column(member::Column::Id)
            .into_tuple()
            .one(&context.database_connection)
            .await?;

        return Ok(member.map(Into::into));
    }

    async fn fetch_member_by_hash(
        context: &PluxerContext<Self>,
        system_id: Ulid,
        id_hash: u32,
    ) -> Result<Option<MemberModel>, DbErr> {
        let member = member::Entity::find()
            .filter(member::Column::SystemId.eq(DatabaseId::from(system_id)))
            .filter(member::Column::IdHash.eq(id_hash as i32))
            .one(&context.database_connection)
            .await?;

        return Ok(member.map(Into::into));
    }

    async fn member_exists(
        context: &PluxerContext<Self>,
        system_id: Ulid,
        member_id: Ulid,
    ) -> Result<bool, DbErr> {
        let count = member::Entity::find_by_id(DatabaseId::from(member_id))
            .filter(member::Column::SystemId.eq(DatabaseId::from(system_id)))
            .count(&context.database_connection)
            .await?;

        return Ok(count != 0);
    }

    async fn fetch_member_by_id(
        context: &PluxerContext<Self>,
        system_id: Ulid,
        member_id: Ulid,
    ) -> Result<Option<MemberModel>, DbErr> {
        let member = member::Entity::find_by_id(DatabaseId::from(member_id))
            .filter(member::Column::SystemId.eq(DatabaseId::from(system_id)))
            .one(&context.database_connection)
            .await?;

        return Ok(member.map(Into::into));
    }

    async fn update_member_by_id(
        context: &PluxerContext<Self>,
        member_id: Ulid,
        name: DatabaseUpdate<&str>,
        display_name: DatabaseUpdate<Option<&str>>,
        pronouns: DatabaseUpdate<Option<&str>>,
        avatar_url: DatabaseUpdate<Option<&str>>,
        description: DatabaseUpdate<Option<&str>>,
        color: DatabaseUpdate<Option<u32>>,
    ) -> Result<(), DbErr> {
        let member = member::ActiveModel {
            id: ActiveValue::Set(DatabaseId::from(member_id)),

            id_hash: ActiveValue::NotSet,
            system_id: ActiveValue::NotSet,

            name: name.map(str::to_ascii_lowercase).into(),
            display_name: display_name.map(|it| it.map(ToString::to_string)).into(),

            description: description.map(|it| it.map(ToString::to_string)).into(),
            pronouns: pronouns.map(|it| it.map(ToString::to_string)).into(),
            avatar_url: avatar_url.map(|it| it.map(ToString::to_string)).into(),
            color: color.map(|it| it.map(|it| it as i32)).into(),

            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::Set(Utc::now()),
        };

        member.update(&context.database_connection).await?;

        return Ok(());
    }

    async fn delete_member(
        context: &PluxerContext<Self>,
        system_id: Ulid,
        member_id: Ulid,
    ) -> Result<(), DbErr> {
        member::Entity::delete_by_id(DatabaseId::from(member_id))
            .filter(member::Column::SystemId.eq(DatabaseId::from(system_id)))
            .exec(&context.database_connection)
            .await?;

        return Ok(());
    }

    async fn create_member_proxy(
        context: &PluxerContext<Self>,
        system_id: Ulid,
        member_id: Ulid,
        proxy: String,
    ) -> Result<(), DbErr> {
        let proxy = proxy::ActiveModel {
            id: ActiveValue::Set(DatabaseId::from(Ulid::generate())),
            member_id: ActiveValue::Set(DatabaseId::from(member_id)),
            system_id: ActiveValue::Set(DatabaseId::from(system_id)),
            proxy: ActiveValue::Set(proxy),
        };

        proxy.insert(&context.database_connection).await?;

        return Ok(());
    }

    async fn fetch_member_proxies(
        context: &PluxerContext<Self>,
        system_id: Ulid,
        member_id: Ulid,
    ) -> Result<Vec<String>, DbErr> {
        let proxies = proxy::Entity::find()
            .filter(proxy::Column::SystemId.eq(DatabaseId::from(system_id)))
            .filter(proxy::Column::MemberId.eq(DatabaseId::from(member_id)))
            .select_only()
            .column(proxy::Column::Proxy)
            .into_tuple()
            .all(&context.database_connection)
            .await?;

        return Ok(proxies);
    }

    async fn fetch_system_proxies(
        context: &PluxerContext<Self>,
        system_id: Ulid,
    ) -> Result<Vec<(String, Ulid)>, DbErr> {
        let proxies: Vec<(String, DatabaseId)> = proxy::Entity::find()
            .filter(proxy::Column::SystemId.eq(DatabaseId::from(system_id)))
            .select_only()
            .column(proxy::Column::Proxy)
            .column(proxy::Column::MemberId)
            .into_tuple()
            .all(&context.database_connection)
            .await?;

        return Ok(proxies.into_iter().map(|it| (it.0, it.1.into())).collect());
    }

    async fn has_member_proxy(
        context: &PluxerContext<Self>,
        system_id: Ulid,
        member_id: Ulid,
        proxy: &str,
    ) -> Result<bool, DbErr> {
        let count = proxy::Entity::find()
            .filter(proxy::Column::SystemId.eq(DatabaseId::from(system_id)))
            .filter(proxy::Column::MemberId.eq(DatabaseId::from(member_id)))
            .filter(proxy::Column::Proxy.eq(proxy))
            .count(&context.database_connection)
            .await?;

        return Ok(count != 0);
    }

    async fn has_system_proxy(
        context: &PluxerContext<Self>,
        system_id: Ulid,
        proxy: &str,
    ) -> Result<bool, DbErr> {
        let count = proxy::Entity::find()
            .filter(proxy::Column::SystemId.eq(DatabaseId::from(system_id)))
            .filter(proxy::Column::Proxy.eq(proxy))
            .count(&context.database_connection)
            .await?;

        return Ok(count != 0);
    }

    async fn delete_member_proxy(
        context: &PluxerContext<Self>,
        system_id: Ulid,
        member_id: Ulid,
        proxy: &str,
    ) -> Result<(), DbErr> {
        proxy::Entity::delete_many()
            .filter(proxy::Column::SystemId.eq(DatabaseId::from(system_id)))
            .filter(proxy::Column::MemberId.eq(DatabaseId::from(member_id)))
            .filter(proxy::Column::Proxy.eq(proxy))
            .exec(&context.database_connection)
            .await?;

        return Ok(());
    }
}
