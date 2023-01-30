pub use sea_orm_migration::prelude::*;

mod m20230130_073549_create_employees_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20230130_073549_create_employees_table::Migration)]
    }
}
