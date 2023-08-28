use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Application::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Application::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Application::Image).string().not_null())
                    .col(ColumnDef::new(Application::Alias).string().not_null())
                    .col(ColumnDef::new(Application::Owner).string().not_null())
                    .col(ColumnDef::new(Application::Server).integer().not_null())
                    //.col(ColumnDef::new(Application::Ports).array(ColumnType::Json))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Application::Table).to_owned())
            .await
    }
}

enum Application {
    Table,
    Id,
    Image,
    Alias,
    Owner,
    Server, //Ports
}

impl Iden for Application {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Table => "applications",
                Self::Id => "id",
                Self::Image => "image",
                Self::Alias => "alias",
                Self::Owner => "owner",
                Self::Server => "server",
                //Self::Ports => "ports",
            }
        )
        .unwrap();
    }
}
