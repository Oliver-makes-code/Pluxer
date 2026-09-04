use sea_orm_migration::{async_trait::async_trait, prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Messages::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Messages::MessageId)
                            .string()
                            .primary_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Messages::UserId).string().not_null())
                    .col(ColumnDef::new(Messages::ChannelId).string().not_null())
                    .col(ColumnDef::new(Messages::SystemId).uuid().not_null())
                    .col(ColumnDef::new(Messages::MemberId).uuid().not_null())
                    .col(
                        ColumnDef::new(Messages::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Messages::ExpiresAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-messages-system")
                            .from(Messages::Table, Messages::SystemId)
                            .to(Systems::Table, Systems::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-messages-member")
                            .from(Messages::Table, Messages::MemberId)
                            .to(Members::Table, Members::Id)
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
            .drop_table(Table::drop().table(Messages::Table).to_owned())
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
}

#[derive(DeriveIden)]
enum Messages {
    Table,

    MessageId,
    UserId,
    ChannelId,

    SystemId,
    MemberId,

    CreatedAt,
    ExpiresAt,
}
