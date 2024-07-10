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
    Salt,
    Verifier,
}

#[derive(Iden)]
enum Mails {
    Table,
    Owner,
    Id,
    From,
    To,
    Subject,
    Body,
    Folder,
    Tag,
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
                    .col(ColumnDef::new(Accounts::Verifier).string().not_null())
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
                    .col(ColumnDef::new(Mails::Owner).uuid().not_null())
                    .col(ColumnDef::new(Mails::From).string().not_null())
                    .col(ColumnDef::new(Mails::To).string().not_null())
                    .col(ColumnDef::new(Mails::Subject).string())
                    .col(ColumnDef::new(Mails::Body).string())
                    .col(ColumnDef::new(Mails::Folder).string())
                    .col(ColumnDef::new(Mails::Tag).string())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK-mails-owner")
                            .from(Mails::Table, Mails::Owner)
                            .to(Accounts::Table, Accounts::Id),
                    )
                    // .index(Index::create().name("IDX-mails-owner").col(Mails::Owner))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Accounts::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Mails::Table).to_owned())
            .await?;
        manager
            .drop_foreign_key(ForeignKey::drop().name("FK-mails-owner").to_owned())
            .await?;

        Ok(())
    }
}
