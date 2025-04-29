use super::{grade, group};
use sea_orm::entity::prelude::*;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sea_query::Expr;


#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "student")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    #[sea_orm(unique_key)] // 课序号
    pub student_id: i64, // 学号
    pub name: String,   // 学生姓名
    pub qq_number: i64, // QQ号
    pub group_id: i64,  // QQ群号
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeWithTimeZone, // 创建时间
    #[sea_orm(
        default_expr = "Expr::current_timestamp()",
        on_update = "Expr::current_timestamp()"
    )]
    pub updated_at: DateTimeWithTimeZone, // 更新时间
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::grade::Entity")]
    Grade,
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<grade::Entity> for Entity {
    fn to() -> RelationDef {
        grade::Relation::Student.def()
    }
}
impl Related<group::Entity> for Entity {
    fn to() -> RelationDef {
        group::Relation::Student.def()
    }
}
#[derive(sea_orm::DeriveIden)]
pub enum Student {
    Table,
    Id,
    StudentId,
    Name,
    QqNumber,
    GroupId,
    CreatedAt,
    UpdatedAt,
}


