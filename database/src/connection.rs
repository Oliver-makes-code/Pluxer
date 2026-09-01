use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use std::time::Duration;

use crate::migrations::{self, _2026_08_31_initial};

pub async fn connect(url: &str) -> Result<DatabaseConnection, sea_orm::DbErr> {
    let mut options = ConnectOptions::new(url);

    options
        .max_connections(20)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8));

    let database = Database::connect(options).await?;

    migrations::Migrator::up(&database, None).await?;

    return Ok(database);
}
