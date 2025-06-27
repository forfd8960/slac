use crate::{
    dto::message::{SendMessageInSocket, WebSocketMessage},
    errors::AppError,
    handlers::{list_channel_memebers, list_simple_users, send_message_to_channel},
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
    let resp = ws.on_upgrade(move |socket| handle_socket(user_id, socket, state));
    Ok(resp)
}

async fn handle_socket(user_id: i64, stream: WebSocket, state: AppState) {
    // Use state and socket here
    let (mut sender, mut receiver) = stream.split();

    let (tx, mut rx) = broadcast::channel(100);
    {
        let mut hash_map = state.tx_set.write().await;
        hash_map.insert(user_id, tx);
    }

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
                Err(e) => {
                    println!("get send message from socket error: {}", e);
                    None
                }
            };

            if send_msg_opt.is_none() {
                continue;
            }

            let send_msg = send_msg_opt.unwrap();
            let members_res = list_channel_memebers(&state.pool, send_msg.channel_id).await;

            match members_res {
                Ok(members) => {
                    let tx_set = state.tx_set.read().await;
                    let member_ids: Vec<i64> = members
                        .chan_members_list
                        .iter()
                        .map(|m| m.user_id)
                        .collect();

                    let senders_res = list_simple_users(&state.pool, member_ids).await;

                    match senders_res {
                        Ok(senders) => {
                            for sender in senders {
                                let sender_opt = tx_set.get(&sender.id);
                                if sender_opt.is_some() {
                                    for msg in &send_msg.msgs {
                                        let socket_msg = WebSocketMessage {
                                            sender: sender.clone(),
                                            parent_msg_id: msg.parent_msg_id,
                                            content_type: msg.content_type.clone(),
                                            text_content: msg.text_content.clone(),
                                            media_url: msg.media_url.clone(),
                                            media_metadata: msg.media_metadata.clone(),
                                        };

                                        let msg_data = serde_json::to_string(&socket_msg).unwrap();

                                        let _ = sender_opt.unwrap().send(msg_data);
                                        println!("created msg in channel: {}", send_msg.channel_id)
                                    }
                                }
                            }
                        }
                        Err(e) => println!("list simple users error: {}", e),
                    }
                }
                Err(e) => println!("list channel members error: {}", e),
            }
        }
    });

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    };
}

#[cfg(test)]
mod tests {
    use crate::dto::SimpleUser;
    use crate::dto::message::{MessageContentType, WebSocketMessage};

    #[test]
    fn test_websocket_message_output() {
        let msg = WebSocketMessage {
            sender: SimpleUser {
                id: 1,
                avatar_url: "http://localhost:8888/users/1/avatar".to_string(),
                display_name: "Alice".to_string(),
            },
            parent_msg_id: None,
            content_type: MessageContentType::Text,
            text_content: "How do you do".to_string(),
            media_url: None,
            media_metadata: None,
        };

        let data = serde_json::to_string(&msg).unwrap();
        println!("{}", data);
    }
}
