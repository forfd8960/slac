use crate::{
    dto::{message::SendMessageInSocket, user},
    errors::AppError,
    handlers::list_channel_memebers,
    state::AppState,
};
use axum::{
    extract::{
        Path, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use tokio::sync::broadcast;

pub async fn message_loop(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    Ok(ws.on_upgrade(move |socket| handle_socket(user_id, socket, state)))
}

async fn handle_socket(user_id: i64, stream: WebSocket, state: AppState) {
    // Use state and socket here
    let (mut sender, mut receiver) = stream.split();

    let (tx, mut rx) = broadcast::channel(100);
    let mut hash_map = state.tx_set.write().await;
    hash_map.insert(user_id, tx);

    // Spawn the first task that will receive broadcast messages and send text
    // messages over the websocket to our client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            println!("received: {}", msg);
            // In any websocket error, break loop.
            if sender.send(Message::text(msg)).await.is_err() {
                break;
            }
        }
    });

    let state = state.clone();

    // Spawn a task that takes messages from the websocket, prepends the user
    // name, and sends them to all broadcast subscribers.
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            // Add username before message.
            println!("received msg from client: {}", text);
            let send_msg_opt: Option<SendMessageInSocket> = match serde_json::from_str(&text) {
                Ok(v) => Some(v),
                Err(e) => None,
            };

            if send_msg_opt.is_none() {
                continue;
            }

            let send_msg = send_msg_opt.unwrap();
            let members_res = list_channel_memebers(&state.pool, send_msg.channel_id).await;

            match members_res {
                Ok(members) => {
                    let tx_set = state.tx_set.read().await;
                    for member in members.chan_members_list {
                        let sender_opt = tx_set.get(&member.user_id);
                        if sender_opt.is_some() {
                            for msg in &send_msg.msgs {
                                let _ = sender_opt.unwrap().send(format!("{}", msg.text_content));
                            }
                        }
                    }
                }
                Err(e) => println!("{}", e),
            }
        }
    });

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    };
}
