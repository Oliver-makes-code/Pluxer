use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter,
    QuerySelect, sea_query::prelude::Utc,
};
use ulid::Ulid;
use xxhash_rust::xxh3::xxh3_64;

use crate::{
    entities::{DatabaseId, member},
    handler::{DatabaseHandler, DatabaseUpdate},
    model::member::MemberModel,
};

impl DatabaseHandler {
    pub async fn fetch_member_count(&self, system_id: Ulid) -> Result<usize, DbErr> {
        return Ok(member::Entity::find()
            .filter(member::Column::SystemId.eq(DatabaseId::from(system_id)))
            .count(&self.conn)
            .await? as usize);
    }

    pub async fn create_member(
        &self,
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

        member.insert(&self.conn).await?;

        return Ok((member_id, member_id_hash));
    }

    pub async fn fetch_member_id_by_name(
        &self,
        system_id: Ulid,
        name: &str,
    ) -> Result<Option<Ulid>, DbErr> {
        let member: Option<DatabaseId> = member::Entity::find()
            .filter(member::Column::SystemId.eq(DatabaseId::from(system_id)))
            .filter(member::Column::Name.eq(name.to_ascii_lowercase()))
            .select_only()
            .column(member::Column::Id)
            .into_tuple()
            .one(&self.conn)
            .await?;

        return Ok(member.map(Into::into));
    }

    pub async fn fetch_member_by_name(
        &self,
        system_id: Ulid,
        name: &str,
    ) -> Result<Option<MemberModel>, DbErr> {
        let member = member::Entity::find()
            .filter(member::Column::SystemId.eq(DatabaseId::from(system_id)))
            .filter(member::Column::Name.eq(name.to_ascii_lowercase()))
            .one(&self.conn)
            .await?;

        return Ok(member.map(Into::into));
    }

    pub async fn fetch_member_id_by_hash(
        &self,
        system_id: Ulid,
        id_hash: u32,
    ) -> Result<Option<Ulid>, DbErr> {
        let member: Option<DatabaseId> = member::Entity::find()
            .filter(member::Column::SystemId.eq(DatabaseId::from(system_id)))
            .filter(member::Column::IdHash.eq(id_hash as i32))
            .select_only()
            .column(member::Column::Id)
            .into_tuple()
            .one(&self.conn)
            .await?;

        return Ok(member.map(Into::into));
    }

    pub async fn fetch_member_by_hash(
        &self,
        system_id: Ulid,
        id_hash: u32,
    ) -> Result<Option<MemberModel>, DbErr> {
        let member = member::Entity::find()
            .filter(member::Column::SystemId.eq(DatabaseId::from(system_id)))
            .filter(member::Column::IdHash.eq(id_hash as i32))
            .one(&self.conn)
            .await?;

        return Ok(member.map(Into::into));
    }

    pub async fn member_exists(&self, system_id: Ulid, member_id: Ulid) -> Result<bool, DbErr> {
        let count = member::Entity::find_by_id(DatabaseId::from(member_id))
            .filter(member::Column::SystemId.eq(DatabaseId::from(system_id)))
            .count(&self.conn)
            .await?;

        return Ok(count != 0);
    }

    pub async fn fetch_member_by_id(
        &self,
        system_id: Ulid,
        member_id: Ulid,
    ) -> Result<Option<MemberModel>, DbErr> {
        let member = member::Entity::find_by_id(DatabaseId::from(member_id))
            .filter(member::Column::SystemId.eq(DatabaseId::from(system_id)))
            .one(&self.conn)
            .await?;

        return Ok(member.map(Into::into));
    }

    pub async fn update_member_by_id(
        &self,
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

        member.update(&self.conn).await?;

        return Ok(());
    }

    pub async fn delete_member(&self, system_id: Ulid, member_id: Ulid) -> Result<(), DbErr> {
        member::Entity::delete_by_id(DatabaseId::from(member_id))
            .filter(member::Column::SystemId.eq(DatabaseId::from(system_id)))
            .exec(&self.conn)
            .await?;

        return Ok(());
    }
}
