pub use sea_orm_migration::prelude::*;

mod m20230828_171934_create_servers;
mod m20230828_180153_create_applications;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230828_171934_create_servers::Migration),
            Box::new(m20230828_180153_create_applications::Migration),
        ]
    }
}
