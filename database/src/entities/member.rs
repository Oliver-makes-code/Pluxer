use sea_orm::{entity::prelude::DateTimeUtc, *};

use crate::entities::DatabaseId;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "members")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: DatabaseId,
    pub id_hash: i32,

    pub system_id: DatabaseId,

    pub name: String,
    pub display_name: Option<String>,

    pub description: Option<String>,
    pub pronouns: Option<String>,
    pub avatar_url: Option<String>,
    pub color: Option<i32>,

    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::system::Entity",
        from = "Column::SystemId",
        to = "super::system::Column::Id"
    )]
    System,
    #[sea_orm(has_many = "super::message::Entity")]
    Messages,
    #[sea_orm(has_many = "super::proxy::Entity")]
    Proxies,
}

impl Related<super::message::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::Messages.def();
    }
}

impl Related<super::system::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::System.def();
    }
}

impl Related<super::proxy::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::Proxies.def();
    }
}

impl ActiveModelBehavior for ActiveModel {}
