use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter,
    QuerySelect,
};
use ulid::Ulid;

use crate::{
    entities::{DatabaseId, proxy},
    handler::DatabaseHandler,
};

impl DatabaseHandler {
    pub async fn create_member_proxy(
        &self,
        system_id: Ulid,
        member_id: Ulid,
        proxy: &str,
    ) -> Result<(), DbErr> {
        let proxy = proxy::ActiveModel {
            id: ActiveValue::Set(DatabaseId::from(Ulid::generate())),
            member_id: ActiveValue::Set(DatabaseId::from(member_id)),
            system_id: ActiveValue::Set(DatabaseId::from(system_id)),
            proxy: ActiveValue::Set(proxy.to_string()),
        };

        proxy.insert(&self.conn).await?;

        return Ok(());
    }

    pub async fn fetch_member_proxies(
        &self,
        system_id: Ulid,
        member_id: Ulid,
    ) -> Result<Vec<String>, DbErr> {
        let proxies = proxy::Entity::find()
            .filter(proxy::Column::SystemId.eq(DatabaseId::from(system_id)))
            .filter(proxy::Column::MemberId.eq(DatabaseId::from(member_id)))
            .select_only()
            .column(proxy::Column::Proxy)
            .into_tuple()
            .all(&self.conn)
            .await?;

        return Ok(proxies);
    }

    pub async fn fetch_system_proxies(
        &self,
        system_id: Ulid,
    ) -> Result<Vec<(String, Ulid)>, DbErr> {
        let proxies: Vec<(String, DatabaseId)> = proxy::Entity::find()
            .filter(proxy::Column::SystemId.eq(DatabaseId::from(system_id)))
            .select_only()
            .column(proxy::Column::Proxy)
            .column(proxy::Column::MemberId)
            .into_tuple()
            .all(&self.conn)
            .await?;

        return Ok(proxies.into_iter().map(|it| (it.0, it.1.into())).collect());
    }

    pub async fn has_member_proxy(
        &self,
        system_id: Ulid,
        member_id: Ulid,
        proxy: &str,
    ) -> Result<bool, DbErr> {
        let count = proxy::Entity::find()
            .filter(proxy::Column::SystemId.eq(DatabaseId::from(system_id)))
            .filter(proxy::Column::MemberId.eq(DatabaseId::from(member_id)))
            .filter(proxy::Column::Proxy.eq(proxy))
            .count(&self.conn)
            .await?;

        return Ok(count != 0);
    }

    pub async fn has_system_proxy(&self, system_id: Ulid, proxy: &str) -> Result<bool, DbErr> {
        let count = proxy::Entity::find()
            .filter(proxy::Column::SystemId.eq(DatabaseId::from(system_id)))
            .filter(proxy::Column::Proxy.eq(proxy))
            .count(&self.conn)
            .await?;

        return Ok(count != 0);
    }

    pub async fn delete_member_proxy(
        &self,
        system_id: Ulid,
        member_id: Ulid,
        proxy: &str,
    ) -> Result<(), DbErr> {
        proxy::Entity::delete_many()
            .filter(proxy::Column::SystemId.eq(DatabaseId::from(system_id)))
            .filter(proxy::Column::MemberId.eq(DatabaseId::from(member_id)))
            .filter(proxy::Column::Proxy.eq(proxy))
            .exec(&self.conn)
            .await?;

        return Ok(());
    }
}
