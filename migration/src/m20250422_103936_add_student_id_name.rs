use qqbot_core::models::grade::Grade;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Grade::Table)
                    .add_column(string(Grade::StudentName).default("nickname"))
                    .add_column(string(Grade::ExamName).default("unknown exam"))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Grade::Table)
                    .drop_column(Grade::StudentName)
                    .drop_column(Grade::ExamName)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
