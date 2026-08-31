use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

pub async fn connect(url: &str) -> Result<DatabaseConnection, sea_orm::DbErr> {
    let mut options = ConnectOptions::new(url);

    options
        .max_connections(20)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8));

    return Database::connect(options).await;
}
