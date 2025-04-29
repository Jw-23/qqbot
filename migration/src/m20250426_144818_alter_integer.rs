use qqbot_core::models::{grade::Grade, student::Student};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("grade_ibfk_1")
                    .table(Grade::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Student::Table)
                    .modify_column(big_integer(Student::GroupId).default(0))
                    .modify_column(big_integer(Student::QqNumber).default(0))
                    .modify_column(big_integer(Student::StudentId).not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Grade::Table)
                    .modify_column(big_integer(Grade::StudentId).not_null())
                    .modify_column(big_integer(Grade::CourseId).not_null())
                    .add_foreign_key(
                        ForeignKey::create()
                            .from(Grade::Table, Grade::StudentId)
                            .to(Student::Table, Student::StudentId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .get_foreign_key(),
                    )
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

