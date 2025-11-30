use std::net::SocketAddr;

use quinn::{RecvStream, SendStream};
use shared::{ClientMessage, ServerMessage, receive_message, send_message};

use crate::server_session::generate_session_token;

pub async fn handle_gameplay(
    mut send: &mut SendStream,
    mut recv: &mut RecvStream,
    addr: SocketAddr,
) -> anyhow::Result<()> {
    if addr.ip().is_loopback() {
        send_message(
            &mut send,
            ServerMessage::PlayerAuthResponse {
                username: "Master".to_string(),
                session: generate_session_token()?,
            },
        )
        .await?;
        // TODO: Add Master Player to Global??
    };

    loop {
        tokio::select! {
            incoming = receive_message::<ClientMessage>(&mut recv) => {
                match incoming {
                    Ok(_) => {
                        println!("Received non-gameplay message on chat stream from {addr}");
                    }
                    Err(e) => {
                        println!("Error handling gameplay read from {addr}: {e}");
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}
