use std::net::SocketAddr;

use quinn::{RecvStream, SendStream};
use shared::{CHUNK, ClientMessage, ServerMessage, receive_message, send_message};

pub async fn handle_chunk_stream(
    mut send: &mut SendStream,
    mut recv: &mut RecvStream,
    addr: SocketAddr,
) -> anyhow::Result<()> {
    loop {
        tokio::select! {
            incoming = receive_message::<ClientMessage>(&mut recv) => {
                match incoming {
                    Ok(ClientMessage::ChunkRequest{ session, pos }) => {
                        println!("Handling chunk request from {addr}");
                        let chunk = CHUNK.read().await;
                        send_message(&mut send, ServerMessage::ChunkResponse { pos, chunk: chunk.clone() }).await?;
                    }
                    Ok(_) => {
                        println!("Received non-chunk message on chat stream from {addr}");
                    }
                    Err(e) => {
                        println!("Error handling chunk stream read from {addr}: {e}");
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}
