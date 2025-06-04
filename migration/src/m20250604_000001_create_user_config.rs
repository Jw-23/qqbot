use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserConfig::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserConfig::UserId)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserConfig::Strategy)
                            .string()
                            .not_null()
                            .default("llm_strategy"),
                    )
                    .col(
                        ColumnDef::new(UserConfig::Model)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(UserConfig::CustomPrompt)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(UserConfig::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(UserConfig::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserConfig::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserConfig {
    Table,
    UserId,
    Strategy,
    Model,
    CustomPrompt,
    CreatedAt,
    UpdatedAt,
}
