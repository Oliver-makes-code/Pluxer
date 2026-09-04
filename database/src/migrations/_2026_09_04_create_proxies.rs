use sea_orm_migration::{async_trait::async_trait, prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Proxies::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Proxies::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Proxies::MemberId).uuid().not_null())
                    .col(ColumnDef::new(Proxies::SystemId).uuid().not_null())
                    .col(ColumnDef::new(Proxies::Proxy).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-proxies-member")
                            .from(Proxies::Table, Proxies::MemberId)
                            .to(Members::Table, Members::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-proxies-system")
                            .from(Proxies::Table, Proxies::SystemId)
                            .to(Systems::Table, Systems::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("uq-proxies-system")
                            .col(Proxies::SystemId)
                            .col(Proxies::Proxy)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        return Ok(());
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Proxies::Table).to_owned())
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
enum Proxies {
    Table,
    Id,
    MemberId,
    SystemId,
    Proxy,
}
