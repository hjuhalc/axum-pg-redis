use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde::{Deserialize, Serialize};

use crate::models::employees::{self, Entity as Employee, Model};
use crate::structs::AppState;

pub fn create_route() -> Router<AppState> {
    Router::new()
        .route("/employees", get(get_all_employees))
        .route("/employees/:id", get(get_employee_by_id))
        .route("/employees", post(create_employee))
        .route("/employees/:id", put(update_employee))
        .route("/employees/:id", delete(delete_employee))
}

async fn get_all_employees(State(state): State<AppState>) -> Json<Vec<Model>> {
    let employees = Employee::find().all(&state.db).await.unwrap();

    Json(employees)
}

async fn get_employee_by_id(Path(id): Path<String>, State(state): State<AppState>) -> Json<Model> {
    let employee = Employee::find_by_id(id.parse::<i32>().unwrap())
        .one(&state.db)
        .await
        .unwrap()
        .unwrap();

    Json(employee)
}

async fn create_employee(
    State(state): State<AppState>,
    payload: Json<EmployeePayload>,
) -> impl IntoResponse {
    let employee = employees::ActiveModel {
        name: Set(payload.name.to_owned()),
        position: Set(payload.position.to_owned()),
        department: Set(payload.department.to_owned()),
        email: Set(payload.email.to_owned()),
        ..Default::default()
    };

    let id = Employee::insert(employee)
        .exec(&state.db)
        .await
        .unwrap()
        .last_insert_id;

    let employee = Employee::find_by_id(id)
        .one(&state.db)
        .await
        .unwrap()
        .unwrap();

    (StatusCode::CREATED, Json(employee))
}

async fn update_employee(
    Path(id): Path<String>,
    State(state): State<AppState>,
    payload: Json<EmployeePayload>,
) -> Json<Model> {
    let mut employee: employees::ActiveModel = Employee::find_by_id(id.parse::<i32>().unwrap())
        .one(&state.db)
        .await
        .unwrap()
        .unwrap()
        .into();

    employee.name = Set(payload.name.to_owned());
    employee.position = Set(payload.position.to_owned());
    employee.department = Set(payload.department.to_owned());
    employee.email = Set(payload.email.to_owned());

    let result = employee.update(&state.db).await.unwrap();

    Json(result)
}

async fn delete_employee(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let _ = Employee::delete_by_id(id.parse::<i32>().unwrap())
        .exec(&state.db)
        .await
        .unwrap();

    StatusCode::NO_CONTENT
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeePayload {
    pub name: String,
    pub position: String,
    pub department: String,
    pub email: Option<String>,
}
