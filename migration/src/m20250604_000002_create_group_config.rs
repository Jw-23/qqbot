use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GroupConfig::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GroupConfig::GroupId)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(GroupConfig::Strategy)
                            .string()
                            .not_null()
                            .default("llm_strategy"),
                    )
                    .col(
                        ColumnDef::new(GroupConfig::Model)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(GroupConfig::CustomPrompt)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(GroupConfig::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(GroupConfig::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .extra("ON UPDATE CURRENT_TIMESTAMP".to_owned()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GroupConfig::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum GroupConfig {
    Table,
    GroupId,
    Strategy,
    Model,
    CustomPrompt,
    CreatedAt,
    UpdatedAt,
}
