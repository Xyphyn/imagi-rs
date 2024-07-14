use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Post {
    pub id: Option<String>,
    pub title: String,
    pub image: String,
    pub created_at: Option<i64>,
}

pub struct AppState {
    pub post_db: Arc<Mutex<Vec<Post>>>,
}

impl AppState {
    pub fn init() -> AppState {
        AppState {
            post_db: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct QueryOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize)]
pub struct UpdatePost {
    pub title: Option<String>,
    pub delete: Option<bool>,
}
