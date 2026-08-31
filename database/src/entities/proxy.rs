use sea_orm::*;

use crate::entities::DatabaseId;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "proxies")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: DatabaseId,

    pub member_id: DatabaseId,

    pub prefix: Option<String>,
    pub suffix: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::member::Entity",
        from = "Column::MemberId",
        to = "super::member::Column::Id"
    )]
    Member,
}

impl Related<super::member::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::Member.def();
    }
}

impl ActiveModelBehavior for ActiveModel {}
