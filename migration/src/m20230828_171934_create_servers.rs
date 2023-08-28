use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Server::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Server::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Server::IP).string().not_null())
                    .col(ColumnDef::new(Server::Port).unsigned().not_null())
                    .col(ColumnDef::new(Server::Name).string().not_null())
                    .col(ColumnDef::new(Server::Owner).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Server::Table).to_owned())
            .await
    }
}

enum Server {
    Table,
    Id,
    IP,
    Port,
    Name,
    Owner,
}

impl Iden for Server {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Table => "servers",
                Self::Id => "id",
                Self::IP => "ip",
                Self::Port => "port",
                Self::Name => "name",
                Self::Owner => "owner",
            }
        )
        .unwrap();
    }
}
