use qqbot_core::models::{grade::*, student::*};
use sea_orm_migration::{prelude::*, schema::enumeration_null, sea_orm::Iterable};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Student::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Student::Id)
                            .big_integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(Student::StudentId)
                            .big_integer()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Student::Name).string().not_null())
                    .col(ColumnDef::new(Student::QqNumber).big_integer())
                    .col(ColumnDef::new(Student::GroupId).big_integer())
                    .col(
                        ColumnDef::new(Student::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Student::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Grade::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Grade::Id)
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .big_integer(),
                    )
                    .col(
                        ColumnDef::new(Grade::Score)
                            .tiny_integer()
                            .default(0)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Grade::CourseId).not_null().big_integer())
                    .col(ColumnDef::new(Grade::CourseSeq).not_null().tiny_integer())
                    .col(ColumnDef::new(Grade::StudentId).not_null().big_integer())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Grade::Table, Grade::StudentId)
                            .to(Student::Table, Student::StudentId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(enumeration_null(
                        Grade::Category,
                        Alias::new("category"),
                        Category::iter(),
                    ))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Student::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Grade::Table).to_owned())
            .await?;
        Ok(())
    }
}
