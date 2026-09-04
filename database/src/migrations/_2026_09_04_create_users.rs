use sea_orm_migration::{async_trait::async_trait, prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::UserId)
                            .string()
                            .primary_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Users::SystemId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-users-system")
                            .from(Users::Table, Users::SystemId)
                            .to(Systems::Table, Systems::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        return Ok(());
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        return Ok(());
    }
}

#[derive(DeriveIden)]
enum Systems {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    UserId,
    SystemId,
}
