use sea_orm::*;

use crate::entities::DatabaseId;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "proxies")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: DatabaseId,

    pub member_id: DatabaseId,
    pub system_id: DatabaseId,

    pub proxy: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::member::Entity",
        from = "Column::MemberId",
        to = "super::member::Column::Id"
    )]
    Member,
    #[sea_orm(
        belongs_to = "super::system::Entity",
        from = "Column::SystemId",
        to = "super::system::Column::Id"
    )]
    System,
}

impl Related<super::member::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::Member.def();
    }
}

impl Related<super::system::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::System.def();
    }
}

impl ActiveModelBehavior for ActiveModel {}
