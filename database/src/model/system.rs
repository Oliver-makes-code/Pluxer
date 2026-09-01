use std::alloc::System;

use sea_orm::prelude::DateTimeUtc;
use ulid::Ulid;

use crate::entities::system;

pub struct SystemModel {
    pub id: Ulid,

    pub name: String,

    pub tag: Option<String>,

    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub timezone: Option<String>,

    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

impl From<system::Model> for SystemModel {
    fn from(value: system::Model) -> Self {
        return Self {
            id: value.id.into(),
            name: value.name,
            tag: value.tag,
            description: value.description,
            avatar_url: value.avatar_url,
            timezone: value.timezone,
            created_at: value.created_at,
            updated_at: value.updated_at,
        };
    }
}
