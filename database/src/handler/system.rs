use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter,
    sea_query::prelude::Utc,
};
use ulid::Ulid;

use crate::{
    entities::{DatabaseId, platform_id::PlatformId, system, user},
    handler::{DatabaseHandler, DatabaseUpdate},
    model::system::SystemModel,
};

impl DatabaseHandler {
    pub async fn fetch_system_id(&self, user_id: PlatformId) -> Result<Option<Ulid>, DbErr> {
        let user = user::Entity::find_by_id(user_id).one(&self.conn).await?;

        return Ok(user.map(|it| it.system_id.into()));
    }

    async fn set_system_id(
        &self,
        user_id: PlatformId,
        system_id: Option<Ulid>,
    ) -> Result<(), DbErr> {
        let Some(system_id) = system_id else {
            user::Entity::delete_by_id(user_id).exec(&self.conn).await?;

            return Ok(());
        };

        let model = user::ActiveModel {
            id: ActiveValue::Set(user_id),
            system_id: ActiveValue::Set(system_id.into()),
        };

        model.insert(&self.conn).await?;

        return Ok(());
    }

    pub async fn create_system(
        &self,
        user_id: PlatformId,
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

        system.insert(&self.conn).await?;

        self.set_system_id(user_id, Some(system_id)).await?;

        return Ok(system_id);
    }

    /// Returns true if the system was deleted.
    pub async fn detach_or_delete_system(
        &self,
        user_id: PlatformId,
        system_id: Ulid,
    ) -> Result<bool, DbErr> {
        self.set_system_id(user_id, None).await?;

        let fluxer_count = user::Entity::find()
            .filter(user::Column::SystemId.eq(DatabaseId::from(system_id)))
            .count(&self.conn)
            .await?;

        if fluxer_count != 0 {
            return Ok(false);
        }

        system::Entity::delete_by_id(DatabaseId::from(system_id))
            .exec(&self.conn)
            .await?;

        return Ok(true);
    }

    pub async fn fetch_system_by_id(&self, system_id: Ulid) -> Result<Option<SystemModel>, DbErr> {
        let system = system::Entity::find_by_id(DatabaseId::from(system_id))
            .one(&self.conn)
            .await?;

        return Ok(system.map(Into::into));
    }

    pub async fn update_system_by_id(
        &self,
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

        system.update(&self.conn).await?;

        return Ok(());
    }
}
