use std::sync::Arc;

use sea_orm::{ActiveValue, DatabaseConnection, DbErr, Value};

use crate::connection::connect;

pub mod member;
pub mod message;
pub mod proxy;
pub mod system;

pub enum DatabaseUpdate<T> {
    Keep,
    Set(T),
}

impl<T> DatabaseUpdate<T> {
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> DatabaseUpdate<U> {
        match self {
            Self::Keep => return DatabaseUpdate::Keep,
            Self::Set(value) => return DatabaseUpdate::Set(f(value)),
        }
    }
}

impl<T> From<DatabaseUpdate<T>> for ActiveValue<T>
where
    Value: From<T>,
{
    fn from(value: DatabaseUpdate<T>) -> Self {
        match value {
            DatabaseUpdate::Keep => return ActiveValue::NotSet,
            DatabaseUpdate::Set(value) => return ActiveValue::Set(value),
        }
    }
}

pub struct DatabaseHandler {
    conn: DatabaseConnection,
}

impl DatabaseHandler {
    pub async fn new(url: &str) -> Result<Arc<Self>, DbErr> {
        return Ok(Arc::new(Self {
            conn: connect(url).await?,
        }));
    }
}
