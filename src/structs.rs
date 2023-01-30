use sea_orm::DatabaseConnection;

#[allow(dead_code)]
#[derive(Clone)]
pub struct AppState {
    pub(crate) db: DatabaseConnection,
}
