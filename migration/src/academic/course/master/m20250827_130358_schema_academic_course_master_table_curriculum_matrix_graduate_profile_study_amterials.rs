use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        todo!()
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

