use sea_orm::{entity::prelude::DateTimeUtc, *};

use crate::entities::DatabaseId;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "systems")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: DatabaseId,

    pub name: String,
    pub display_name: Option<String>,

    pub tag: Option<String>,

    pub description: Option<String>,
    pub pronouns: Option<String>,
    pub avatar_url: Option<String>,
    pub timezone: Option<String>,
    pub color: Option<i32>,

    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user::fluxer::Entity")]
    FluxerUsers,
    #[sea_orm(has_many = "super::message::fluxer::Entity")]
    FluxerMessages,
    #[sea_orm(has_many = "super::member::Entity")]
    Members,
    #[sea_orm(has_many = "super::proxy::Entity")]
    Proxies,
}

impl Related<super::message::fluxer::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::FluxerMessages.def();
    }
}

impl Related<super::user::fluxer::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::FluxerUsers.def();
    }
}

impl Related<super::member::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::Members.def();
    }
}

impl Related<super::proxy::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::Proxies.def();
    }
}

impl ActiveModelBehavior for ActiveModel {}
