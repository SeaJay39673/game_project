use std::net::SocketAddr;

use quinn::{Connection, Incoming, RecvStream, SendStream};
use shared::{StreamKind, receive_message};

use crate::connection_handlers::{handle_chat, handle_chunk_stream, handle_crafting, handle_gameplay, handle_inventory};

pub async fn handle_stream(mut send: &mut SendStream, mut recv: &mut RecvStream, addr: SocketAddr) -> anyhow::Result<()> {
    match receive_message(&mut recv).await? {
        StreamKind::Chat => handle_chat(&mut send, &mut recv, addr).await?,
        StreamKind::Inventory => handle_inventory(&mut send, &mut recv, addr).await?,
        StreamKind::Crafting => handle_crafting(&mut send, &mut recv, addr).await?,
        StreamKind::ChunkStreaming => handle_chunk_stream(&mut send, &mut recv, addr).await?,
        StreamKind::Gameplay => handle_gameplay(&mut send, &mut recv, addr).await?,
        _ => println!("Error: Not implemented yet!")
    }

    Ok(())
} 

pub async fn handle_connection(conn: Incoming) -> anyhow::Result<()> {
    let connection: Connection = conn.await?;
    let addr: SocketAddr = connection.remote_address();
    
    tokio::spawn({
        let conn_streams: Connection = connection.clone();
        async move {
            loop {
                match conn_streams.accept_bi().await {
                    Ok((mut send, mut recv)) => {
                        tokio::spawn(async move {
                            if let Err(e) = handle_stream(&mut send, &mut recv, addr.clone()).await {
                                println!("Could not handle stream correctly: {e}");
                            }
                        });
                    },
                    Err(e) => {
                        println!("Connection closed or errored: {e}");
                        break;
                    }
                }
            }
        }
    });


    Ok(())
}