use sea_orm::*;

use crate::entities::{DatabaseId, DatabaseSnowflake, system};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "fluxer_users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: DatabaseSnowflake,

    pub system_id: DatabaseId,

    pub instance_url: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "system::Entity",
        from = "Column::SystemId"
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
