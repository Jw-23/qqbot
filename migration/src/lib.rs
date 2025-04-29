pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20250421_140838_add_grade_category;
mod m20250422_103936_add_student_id_name;
mod m20250426_144818_alter_integer;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20250421_140838_add_grade_category::Migration),
            Box::new(m20250422_103936_add_student_id_name::Migration),
            Box::new(m20250426_144818_alter_integer::Migration),
        ]
    }
}
