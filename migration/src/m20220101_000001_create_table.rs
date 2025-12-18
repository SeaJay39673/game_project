use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Accounts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Accounts::Username)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Accounts::PasswordHash)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Accounts::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Characters::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Characters::CharacterId)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(Characters::AccountUsername)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Characters::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Characters::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Characters::Table, Characters::AccountUsername)
                            .to(Accounts::Table, Accounts::Username),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Characters::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Accounts::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Accounts {
    Table,
    Username,
    PasswordHash,
    CreatedAt,
}

#[derive(Iden)]
enum Characters {
    Table,
    CharacterId,
    AccountUsername,
    Name,
    CreatedAt,
}