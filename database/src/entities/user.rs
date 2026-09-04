use sea_orm::*;

use crate::entities::{DatabaseId, platform_id::PlatformId, system};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: PlatformId,

    pub system_id: DatabaseId,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "system::Entity",
        from = "Column::SystemId",
        to = "system::Column::Id"
    )]
    System,
}

impl Related<system::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::System.def();
    }
}

impl ActiveModelBehavior for ActiveModel {}
