use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};

use crate::{
    dto::user::{LoginReq, RegisterRequest},
    errors::AppError,
    models::user::UserRepository,
    service::user::UserService,
    state::AppState,
};

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    println!("register user: {:?}", payload);

    let user_repo = UserRepository::new(&state.pool);
    let user_service = UserService::new(&user_repo, &state.ek, &state.dk);

    let resp = user_service.create_user(&payload).await?;
    println!("created user: {:?}", resp.user);
    Ok(Json(resp))
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginReq>,
) -> Result<impl IntoResponse, AppError> {
    println!("login user: {:?}", req);

    let user_repo = UserRepository::new(&state.pool);
    let user_service = UserService::new(&user_repo, &state.ek, &state.dk);

    let resp = user_service.login(&req).await?;
    Ok(Json(resp))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let user_repo = UserRepository::new(&state.pool);
    let user_service = UserService::new(&user_repo, &state.ek, &state.dk);

    let resp = user_service.get_user(user_id).await?;
    Ok(Json(resp))
}
