use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
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
                    .col(ColumnDef::new(Systems::Pronouns).string())
                    .col(ColumnDef::new(Systems::Tag).string())
                    .col(ColumnDef::new(Systems::Description).string())
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
                    .col(ColumnDef::new(Members::AvatarUrl).string())
                    .col(ColumnDef::new(Members::Color).integer())
                    .col(ColumnDef::new(Members::Pronouns).string())
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

        manager
            .create_table(
                Table::create()
                    .table(FluxerUsers::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(FluxerUsers::Id).integer().not_null())
                    .col(ColumnDef::new(FluxerUsers::InstanceUrl).string().not_null())
                    .col(ColumnDef::new(FluxerUsers::SystemId).uuid().not_null())
                    .primary_key(
                        Index::create()
                            .col(FluxerUsers::Id)
                            .col(FluxerUsers::InstanceUrl),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-fluxer-users-system")
                            .from(FluxerUsers::Table, FluxerUsers::SystemId)
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
            .drop_table(Table::drop().table(FluxerUsers::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Proxies::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Members::Table).to_owned())
            .await?;

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

#[derive(DeriveIden)]
enum Proxies {
    Table,
    Id,
    MemberId,
    SystemId,
    Proxy,
}

#[derive(DeriveIden)]
enum FluxerUsers {
    Table,
    Id,
    SystemId,
    InstanceUrl,
}
