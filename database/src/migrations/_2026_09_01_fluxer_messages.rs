use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FluxerMessages::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(FluxerMessages::Id).big_integer().not_null())
                    .col(
                        ColumnDef::new(FluxerMessages::InstanceUrl)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(FluxerMessages::AuthorId).big_integer().not_null())
                    .col(ColumnDef::new(FluxerMessages::SystemId).uuid().not_null())
                    .col(ColumnDef::new(FluxerMessages::MemberId).uuid().not_null())
                    .col(
                        ColumnDef::new(FluxerMessages::ExpiresAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(FluxerMessages::Id)
                            .col(FluxerMessages::InstanceUrl),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-fluxer-messages-system")
                            .from(FluxerMessages::Table, FluxerMessages::SystemId)
                            .to(Systems::Table, Systems::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-fluxer-messages-member")
                            .from(FluxerMessages::Table, FluxerMessages::MemberId)
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
            .drop_table(Table::drop().table(FluxerMessages::Table).to_owned())
            .await?;

        return Ok(());
    }
}

#[derive(DeriveIden)]
enum FluxerMessages {
    Table,
    Id,
    InstanceUrl,
    AuthorId,
    SystemId,
    MemberId,
    ExpiresAt,
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
