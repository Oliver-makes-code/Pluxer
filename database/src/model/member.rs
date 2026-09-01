use sea_orm::prelude::DateTimeUtc;
use ulid::Ulid;

use crate::entities::member;

pub struct MemberModel {
    pub id: Ulid,
    pub id_hash: u32,

    pub system_id: Ulid,

    pub name: String,
    pub display_name: Option<String>,

    pub description: Option<String>,
    pub pronouns: Option<String>,
    pub avatar_url: Option<String>,
    pub color: Option<u32>,

    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

impl From<member::Model> for MemberModel {
    fn from(value: member::Model) -> Self {
        return Self {
            id: value.id.into(),
            id_hash: value.id_hash as u32,

            system_id: value.system_id.into(),

            name: value.name,
            display_name: value.display_name,

            description: value.description,
            pronouns: value.pronouns,
            avatar_url: value.avatar_url,
            color: value.color.map(|it| it as u32),

            created_at: value.created_at,
            updated_at: value.updated_at,
        };
    }
}
