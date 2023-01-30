use crate::models::employees::Entity as EmployeeModel;
use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use sea_orm::EntityTrait;
use serde::Serialize;

use crate::structs::AppState;

pub fn create_route() -> Router<AppState> {
    Router::new()
        .route("/employee", get(get_all_employees))
        .route("/employee/:id", get(get_employee_by_id))
        .route("/employee", post(create_employee))
        .route("/employee/:id", put(update_employee))
        .route("/employee/:id", delete(delete_employee))
}

#[derive(Serialize)]
struct Employee {
    id: String,
    name: String,
    position: String,
    department: String,
    email: Option<String>,
}

async fn get_all_employees(State(state): State<AppState>) -> Json<Vec<Employee>> {
    let employees = EmployeeModel::find().all(&state.db).await.unwrap();
    let mut employees_vec = Vec::new();

    for employee in employees {
        employees_vec.push(Employee {
            id: employee.id.to_string(),
            name: employee.name,
            position: employee.position,
            department: employee.department,
            email: employee.email,
        });
    }

    Json(employees_vec)
}

async fn get_employee_by_id(Path(id): Path<String>) -> Json<String> {
    Json(format!("Employee: {}", id).to_string())
}

async fn create_employee() -> Json<String> {
    Json("Hello, World!".to_string())
}

async fn update_employee(Path(id): Path<String>) -> Json<String> {
    Json(format!("Update emplyee: {}", id).to_string())
}

async fn delete_employee(Path(id): Path<String>) -> Json<String> {
    Json(format!("Deleted employee: {}", id).to_string())
}
