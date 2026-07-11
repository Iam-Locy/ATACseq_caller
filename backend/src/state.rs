use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db_path: Arc<String>
}