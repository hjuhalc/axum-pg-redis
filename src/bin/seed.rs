use dotenv_codegen::dotenv;
use sea_orm::{
    entity::prelude::*, ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, Set,
    Statement,
};
use tokio;

use crate::models::*;

#[path = "../models/mod.rs"]
mod models;

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let result = rt.block_on(seed_database());

    println!("{:?}", result);
}

const NAMES: &[&str] = &[
    "John Doe",
    "Jane Doe",
    "Jim Smith",
    "Emily Davis",
    "Michael Johnson",
];
const POSITIONS: &[&str] = &[
    "Software Engineer",
    "Data Scientist",
    "Product Manager",
    "DevOps Engineer",
    "UX Designer",
];
const DEPARTMENTS: &[&str] = &["IT", "Data", "Product", "DevOps", "Design"];

async fn seed_database() -> Result<(), DbErr> {
    let db: DatabaseConnection = Database::connect(dotenv!("DATABASE_URL")).await.unwrap();

    println!("Seeding employees table...");

    let mut employees: Vec<employees::ActiveModel> = vec![];

    for i in 1..=10000 {
        let name = NAMES[i % NAMES.len()];
        let position = POSITIONS[i % POSITIONS.len()];
        let department = DEPARTMENTS[i % DEPARTMENTS.len()];

        let email = format!(
            "{}.{}.{}@example.com",
            name.to_lowercase().replace(" ", "."),
            position.to_lowercase().replace(" ", "."),
            i
        );

        let employee = employees::ActiveModel {
            id: Set(i.try_into().unwrap()),
            name: Set(name.to_owned()),
            position: Set(position.to_owned()),
            department: Set(department.to_owned()),
            email: Set(Some(email)),
            created_at: Set(DateTime::new(Date::default(), Time::default())),
            updated_at: Set(DateTime::new(Date::default(), Time::default())),
        };

        employees.push(employee);

        if employees.len() == 1000 {
            let to_insert = employees.clone();
            let _ = employees::Entity::insert_many(to_insert).exec(&db).await?;
            employees.clear();
        }
    }

    // Prevent duplicate primary key error when inserting new rows
    let _: Option<QueryResult> = db
        .query_one(Statement::from_string(
            DatabaseBackend::MySql,
            "SELECT setval('employees_id_seq', (SELECT MAX(id) FROM employees));".to_owned(),
        ))
        .await?;

    Ok(())
}
