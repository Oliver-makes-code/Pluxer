use sea_orm::prelude::DateTimeUtc;
use ulid::Ulid;

use crate::entities::system;

pub struct SystemModel {
    pub id: Ulid,

    pub name: String,
    pub display_name: Option<String>,

    pub tag: Option<String>,

    pub description: Option<String>,
    pub pronouns: Option<String>,
    pub avatar_url: Option<String>,
    pub timezone: Option<String>,
    pub color: Option<u32>,

    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

impl From<system::Model> for SystemModel {
    fn from(value: system::Model) -> Self {
        return Self {
            id: value.id.into(),
            name: value.name,
            display_name: value.display_name,
            tag: value.tag,
            description: value.description,
            pronouns: value.pronouns,
            avatar_url: value.avatar_url,
            timezone: value.timezone,
            color: value.color.map(|it| it as u32),
            created_at: value.created_at,
            updated_at: value.updated_at,
        };
    }
}
