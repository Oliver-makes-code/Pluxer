use sea_orm::{prelude::DateTimeUtc, *};

use crate::entities::{DatabaseId, DatabaseSnowflake, member, system};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "fluxer_messages")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: DatabaseSnowflake,

    #[sea_orm(primary_key, auto_increment = false)]
    pub instance_url: String,

    pub author_id: DatabaseSnowflake,

    pub system_id: DatabaseId,
    pub member_id: DatabaseId,

    pub expires_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "system::Entity",
        from = "Column::SystemId",
        to = "system::Column::Id"
    )]
    System,
    #[sea_orm(
        belongs_to = "member::Entity",
        from = "Column::MemberId",
        to = "member::Column::Id"
    )]
    Member,
}

impl Related<system::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::System.def();
    }
}

impl Related<member::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::Member.def();
    }
}

impl ActiveModelBehavior for ActiveModel {}
