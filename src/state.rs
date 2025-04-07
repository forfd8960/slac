use sqlx::PgPool;
use std::{collections::HashMap, ops::Deref, sync::Arc};

#[derive(Debug, Clone)]
pub struct AppState {
    pub inner: Arc<AppStateInner>,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        let inner = Arc::new(AppStateInner {
            users_subs: HashMap::new(),
            pool,
            host: "http://localhost:8989".to_string(),
        });

        Self { inner }
    }
}

impl Deref for AppState {
    type Target = AppStateInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug, Clone)]
pub struct AppStateInner {
    pub users_subs: HashMap<String, Vec<String>>,
    pub pool: PgPool,
    pub host: String,
}
