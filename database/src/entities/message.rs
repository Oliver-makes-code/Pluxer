use sea_orm::{prelude::DateTimeUtc, *};

use crate::entities::{DatabaseId, member, platform_id::PlatformId, system};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "messages")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub message_id: PlatformId,
    pub user_id: PlatformId,
    pub channel_id: PlatformId,

    pub system_id: DatabaseId,
    pub member_id: DatabaseId,

    pub created_at: DateTimeUtc,
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
