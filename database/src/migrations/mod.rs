use sea_orm_migration::prelude::*;

mod _2026_09_04_create_members;
mod _2026_09_04_create_messages;
mod _2026_09_04_create_proxies;
mod _2026_09_04_create_systems;
mod _2026_09_04_create_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        return vec![
            Box::new(_2026_09_04_create_systems::Migration),
            Box::new(_2026_09_04_create_members::Migration),
            Box::new(_2026_09_04_create_proxies::Migration),
            Box::new(_2026_09_04_create_messages::Migration),
            Box::new(_2026_09_04_create_users::Migration),
        ];
    }
}
