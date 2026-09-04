use sea_orm_migration::{async_trait::async_trait, prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Members::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Members::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Members::IdHash).integer().not_null())
                    .col(ColumnDef::new(Members::SystemId).uuid().not_null())
                    .col(ColumnDef::new(Members::Name).string().not_null())
                    .col(ColumnDef::new(Members::DisplayName).string())
                    .col(ColumnDef::new(Members::Description).string())
                    .col(ColumnDef::new(Members::Pronouns).string())
                    .col(ColumnDef::new(Members::AvatarUrl).string())
                    .col(ColumnDef::new(Members::Color).integer())
                    .col(
                        ColumnDef::new(Members::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Members::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-members-system")
                            .from(Members::Table, Members::SystemId)
                            .to(Systems::Table, Systems::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("uq-members-system-id-hash")
                            .col(Members::SystemId)
                            .col(Members::IdHash)
                            .unique(),
                    )
                    .index(
                        Index::create()
                            .name("uq-members-system-name")
                            .col(Members::SystemId)
                            .col(Members::Name)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        return Ok(());
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Members::Table).to_owned())
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
enum Members {
    Table,
    Id,
    IdHash,
    SystemId,
    Name,
    DisplayName,
    Pronouns,
    Description,
    AvatarUrl,
    CreatedAt,
    UpdatedAt,
    Color,
}
