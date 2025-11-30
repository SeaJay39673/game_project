use std::net::SocketAddr;

use quinn::{RecvStream, SendStream};



pub async fn handle_inventory(mut send: &mut SendStream, mut recv: &mut RecvStream, addr: SocketAddr) -> anyhow::Result<()> {

    Ok(())
}