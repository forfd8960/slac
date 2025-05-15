use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};

use crate::{
    dto::channel::{CreateChannelRequest, JoinChanReq, ListChanReq},
    errors::AppError,
    models::{channel::ChanRepository, user::UserRepository},
    service::channel::ChannelService,
    state::AppState,
};

pub async fn create_channel(
    State(state): State<AppState>,
    Json(req): Json<CreateChannelRequest>,
) -> Result<impl IntoResponse, AppError> {
    println!("create channel: {:?}", req);

    let user_repo = UserRepository::new(&state.pool);

    let chan_repo = ChanRepository::new(&state.pool);
    let chan_service = ChannelService::new(&chan_repo, &user_repo);

    let resp = chan_service.create_channel(&req).await?;
    println!("created channel: {:?}", resp.channel);
    Ok(Json(resp))
}

pub async fn list_channels(
    State(state): State<AppState>,
    Json(req): Json<ListChanReq>,
) -> Result<impl IntoResponse, AppError> {
    println!("list channel req: {:?}", req);

    let user_repo = UserRepository::new(&state.pool);

    let chan_repo = ChanRepository::new(&state.pool);
    let chan_service = ChannelService::new(&chan_repo, &user_repo);

    let resp = chan_service.list_channels(&req).await?;
    println!("created channel: {:?}", resp.channels);
    Ok(Json(resp))
}

pub async fn get_channel(
    State(state): State<AppState>,
    Path(channel_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    println!("get channel req: {}", channel_id);

    let user_repo = UserRepository::new(&state.pool);

    let chan_repo = ChanRepository::new(&state.pool);
    let chan_service = ChannelService::new(&chan_repo, &user_repo);

    let resp = chan_service.get_channel(channel_id).await?;
    println!("get channel response: {:?}", resp);
    Ok(Json(resp))
}

pub async fn join_channel(
    State(state): State<AppState>,
    Path(channel_id): Path<i64>,
    Json(req): Json<JoinChanReq>,
) -> Result<impl IntoResponse, AppError> {
    println!("join channel req: {:?}", req);

    let user_repo = UserRepository::new(&state.pool);
    let chan_repo = ChanRepository::new(&state.pool);
    let chan_service = ChannelService::new(&chan_repo, &user_repo);

    let resp = chan_service.join_channel(req.user_id, channel_id).await?;
    println!("join channel response: {:?}", resp);
    Ok(Json(resp))
}

pub async fn leave_channel(
    State(state): State<AppState>,
    Path(channel_id): Path<i64>,
    Json(req): Json<JoinChanReq>,
) -> Result<impl IntoResponse, AppError> {
    let user_repo = UserRepository::new(&state.pool);
    let chan_repo = ChanRepository::new(&state.pool);
    let chan_service = ChannelService::new(&chan_repo, &user_repo);

    let resp = chan_service.leave_channel(req.user_id, channel_id).await?;
    println!("leave channel response: {:?}", resp);
    Ok(Json(resp))
}
