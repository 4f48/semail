use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum Accounts {
    Table,
    Id,
    Name,
    PublicKey,
    PrivateKey,
    Password,
}

#[derive(Iden)]
enum Mails {
    Table,
    Id,
    From,
    To,
    Subject,
    Body,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Accounts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Accounts::Id)
                            .primary_key()
                            .uuid()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Accounts::Name)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Accounts::PublicKey)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Accounts::PrivateKey)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Accounts::Password).string().not_null())
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_table(
                Table::create()
                    .table(Mails::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Mails::Id)
                            .primary_key()
                            .uuid()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Mails::From).string().not_null())
                    .col(ColumnDef::new(Mails::To).string().not_null())
                    .col(ColumnDef::new(Mails::Subject).string())
                    .col(ColumnDef::new(Mails::Body).string())
                    .to_owned(),
            )
            .await?;
        
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Accounts::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Mails::Table).to_owned()).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
}
