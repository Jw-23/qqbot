use sea_orm::entity::prelude::*;
use sea_query::Iden;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "grade")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub student_name: String,
    pub exam_name: String,
    pub course_id: i32, // 课程号
    pub course_seq: i8,
    pub student_id: i64,  // 关联学生ID
    pub score: i8,        // 成绩
    pub category: String, // 类型
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::student::Entity",
        from = "Column::StudentId",
        to = "super::student::Column::StudentId"
    )]
    Student,
}

impl Related<super::student::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Student.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(sea_orm::DeriveIden)]
pub enum Grade {
    Table,
    Id,
    CourseId,
    CourseSeq,
    StudentId,
    Score,
    Category,
    StudentName,
    ExamName
}

#[derive(Iden, EnumIter)]
pub enum Category {
    #[iden = "Quiz-1"]
    Quiz1,
    #[iden = "Quiz-2"]
    Quiz2,
    #[iden = "Quiz-3"]
    Quiz3,
    #[iden = "Quiz-4"]
    Quiz4,
    #[iden = "Mid"]
    Mid,
}
