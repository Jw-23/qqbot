use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, DeriveEntityModel)]
#[sea_orm(table_name = "group")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    id: i32,
    group_id: i32,
}
#[derive(Debug, Serialize, Deserialize, DeriveRelation, EnumIter)]
pub enum Relation {
    #[sea_orm(has_many = "super::student::Entity")]
    Student,
}

impl ActiveModelBehavior for ActiveModel {}
impl Related<super::student::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Student.def()
    }
}

pub enum Group {
    Table,
    Id,
    GroupId
}
