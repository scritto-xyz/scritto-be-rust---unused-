use sea_orm_migration::prelude::*;
use sea_orm::{EnumIter, Iterable};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::FirstName).string().not_null())
                    .col(ColumnDef::new(Users::LastName).string().not_null())
                    .col(ColumnDef::new(Users::Email).string().not_null())
                    .col(ColumnDef::new(Users::Password).string().not_null())
                    .col(ColumnDef::new(Users::Country).string().not_null())
                    .col(ColumnDef::new(Users::State).string().not_null())
                    .col(ColumnDef::new(Users::City).string().not_null())
                    .col(ColumnDef::new(Users::UserType).enumeration(
                        UserType::Table, UserType::iter().skip(1)).not_null())
                    .col(ColumnDef::new(Users::CreatedTS).timestamp()
                        .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).not_null())
                    .col(ColumnDef::new(Users::UpdatedTS).timestamp()
                        .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
    Password,
    Country,
    State,
    City,
    UserType,
    CreatedTS,
    UpdatedTS,
}

#[derive(Iden, EnumIter)]
enum UserType {
    Table,
    Artist,
    Client,
}