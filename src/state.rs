use sqlx::PgPool;
use std::{ops::Deref, sync::Arc};
use tokio::sync::broadcast;

use crate::{
    auth::{DecodingKey, EncodingKey},
    errors::AppError,
};

#[derive(Clone)]
pub struct AppState {
    pub inner: Arc<AppStateInner>,
}

impl AppState {
    pub fn new(pool: PgPool, tx: broadcast::Sender<String>) -> Result<Self, AppError> {
        let ek = EncodingKey::load(include_str!("../private_key.pem"))?;
        let dk = DecodingKey::load(include_str!("../public_key.pem"))?;
        let inner = Arc::new(AppStateInner { pool, ek, dk, tx });

        Ok(Self { inner })
    }
}

impl Deref for AppState {
    type Target = AppStateInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Clone)]
pub struct AppStateInner {
    pub pool: PgPool,
    pub ek: EncodingKey,
    pub dk: DecodingKey,
    pub tx: broadcast::Sender<String>,
}
