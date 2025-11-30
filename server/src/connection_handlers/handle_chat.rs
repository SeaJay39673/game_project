use std::net::SocketAddr;

use quinn::{RecvStream, SendStream};
use shared::{ClientMessage, receive_message};

pub async fn handle_chat(
    mut send: &mut SendStream,
    mut recv: &mut RecvStream,
    addr: SocketAddr,
) -> anyhow::Result<()> {
    loop {
        tokio::select! {
            incoming = receive_message::<ClientMessage>(&mut recv) => {
                match incoming {
                    Ok(ClientMessage::ChatMessage {session, message }) => {
                        println!("[{addr}] chat: {message}");
                    }
                    Ok(_) => {
                        println!("Received non-chat message on chat stream from {addr}");
                    }
                    Err(e) => {
                        println!("Client {addr} disconnected or error: {e}");
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}
