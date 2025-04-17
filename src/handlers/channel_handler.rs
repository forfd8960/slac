use axum::{Json, extract::State, response::IntoResponse};

use crate::{
    dto::channel::CreateChannelRequest, errors::AppError, models::channel::ChanRepository,
    service::channel::ChannelService, state::AppState,
};

pub async fn create_channel(
    State(state): State<AppState>,
    Json(req): Json<CreateChannelRequest>,
) -> Result<impl IntoResponse, AppError> {
    println!("create channel: {:?}", req);

    let chan_repo = ChanRepository::new(&state.pool);
    let chan_service = ChannelService::new(&chan_repo);

    let resp = chan_service.create_channel(&req).await?;
    println!("created channel: {:?}", resp.channel);
    Ok(Json(resp))
}

pub async fn list_channels() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}

pub async fn get_channel() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}

pub async fn join_channel() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}

pub async fn leave_channel() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}
