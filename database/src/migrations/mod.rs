pub mod _2026_08_31_initial;
pub mod _2026_09_01_fluxer_messages;

use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        return vec![
            Box::new(_2026_08_31_initial::Migration),
            Box::new(_2026_09_01_fluxer_messages::Migration),
        ];
    }
}
