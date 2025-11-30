use bytes::Bytes;
use quinn::{Connection, RecvStream, SendStream};
use serde::Serialize;



pub async fn send_message<T: Serialize>(send: &mut SendStream, msg: T) -> anyhow::Result<()> {
    let bytes: Vec<u8> = bincode::serde::encode_to_vec(msg, bincode::config::standard())?;
    let len: [u8; _] = (bytes.len() as u32).to_be_bytes();

    send.write_all(&len).await?;
    send.write_all(&bytes).await?;
    
    Ok(())
}

pub async fn receive_message<T: serde::de::DeserializeOwned>(recv: &mut RecvStream) -> anyhow::Result<T> {
    let mut len_buf: [u8; 4] = [0u8;4];
    recv.read_exact(&mut len_buf).await?;
    let size: usize = u32::from_be_bytes(len_buf) as usize;

    let mut data:Vec<u8> = vec![0u8;size];
    recv.read_exact(&mut data).await?;
    let (msg, _): (T, usize) = bincode::serde::decode_from_slice(&data, bincode::config::standard())?;

    Ok(msg)
}

pub async fn send_datagram<T: Serialize>(conn: &Connection, msg: &T) -> anyhow::Result<()> {
    let bytes: Vec<u8> = bincode::serde::encode_to_vec(msg, bincode::config::standard())?;

    conn.send_datagram(bytes.into())?;

    Ok(())
}

pub async fn receive_datagram<T: serde::de::DeserializeOwned>(conn: &Connection) -> anyhow::Result<T> {
    let bytes: Bytes = conn.read_datagram().await?;

    let (msg, _): (T, usize) = bincode::serde::decode_from_slice(&bytes, bincode::config::standard())?;

    Ok(msg)
}