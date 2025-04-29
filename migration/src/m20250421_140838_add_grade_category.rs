use qqbot_core::models::{grade::{Category, Grade}, student::Student};
use sea_orm_migration::{prelude::*, schema::*, sea_orm::{EnumIter, Iterable}};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Grade::Table)
                    .modify_column(enumeration_null(
                        Grade::Category,
                        Alias::new("category"),
                        Category::iter(),
                    ))
                    
                    .to_owned(),
            )
            .await?;
        manager.alter_table(
            Table::alter().table(Student::Table).modify_column(
                ColumnDef::new(Student::CreatedAt).timestamp().default(Expr::current_timestamp()).not_null()
            ).modify_column(ColumnDef::new(Student::UpdatedAt).timestamp().default(Expr::current_timestamp()).not_null())
            .to_owned()
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
        .alter_table(
            Table::alter()
                .table(Grade::Table)
                .modify_column(enumeration_null(
                    Grade::Category,
                    Alias::new("category"),
                    OldCategory::iter()
                ))
                .to_owned(),
        )
        .await?;
    Ok(())
    }
}
#[derive(Iden,EnumIter)]
pub enum OldCategory {
    #[iden="Quiz"]
    Quiz,
    #[iden="Mid"]
    Mid,
}
