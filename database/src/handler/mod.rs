use std::{sync::Arc, time::Duration};

use sea_orm::{ActiveValue, ConnectOptions, Database, DatabaseConnection, DbErr, Value};
use sea_orm_migration::MigratorTrait;

use crate::migrations;

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
        let mut options = ConnectOptions::new(url);

        options
            .max_connections(20)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8));

        let database = Database::connect(options).await?;

        migrations::Migrator::up(&database, None).await?;

        return Ok(Arc::new(Self { conn: database }));
    }
}
