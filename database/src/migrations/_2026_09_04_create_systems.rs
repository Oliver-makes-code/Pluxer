use sea_orm_migration::{async_trait::async_trait, prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Systems::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Systems::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Systems::Name).string().not_null())
                    .col(ColumnDef::new(Systems::DisplayName).string())
                    .col(ColumnDef::new(Systems::Tag).string())
                    .col(ColumnDef::new(Systems::Description).string())
                    .col(ColumnDef::new(Systems::Pronouns).string())
                    .col(ColumnDef::new(Systems::AvatarUrl).string())
                    .col(ColumnDef::new(Systems::Timezone).string())
                    .col(ColumnDef::new(Systems::Color).integer())
                    .col(
                        ColumnDef::new(Systems::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Systems::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        return Ok(());
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Systems::Table).to_owned())
            .await?;

        return Ok(());
    }
}

#[derive(DeriveIden)]
enum Systems {
    Table,
    Id,
    Name,
    DisplayName,
    Pronouns,
    Tag,
    Description,
    AvatarUrl,
    Timezone,
    CreatedAt,
    UpdatedAt,
    Color,
}
