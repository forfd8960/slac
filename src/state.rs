use sqlx::PgPool;
use std::{ops::Deref, sync::Arc};

use crate::{
    auth::{DecodingKey, EncodingKey},
    errors::AppError,
    service::user::UserService,
};

#[derive(Clone)]
pub struct AppState {
    pub inner: Arc<AppStateInner>,
}

impl AppState {
    pub fn new(pool: PgPool) -> Result<Self, AppError> {
        let ek = EncodingKey::load(include_str!("../private_key.pem"))?;
        let dk = DecodingKey::load(include_str!("../public_key.pem"))?;
        let inner = Arc::new(AppStateInner { pool, ek, dk });

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
}
