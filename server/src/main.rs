use std::{net::SocketAddr, sync::Arc};
use anyhow::anyhow;

use quinn::{crypto::rustls::QuicServerConfig};
use rcgen::{CertifiedKey, KeyPair};
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};

use crate::server_networking::handle_connection;

mod password;
mod server_networking;
mod connection_handlers;
mod server_session;

#[tokio::main]
async fn main () -> anyhow::Result<()>{

    if let Err(e) = rustls::crypto::aws_lc_rs::default_provider().install_default() {
        return Err(anyhow!("Error installing aws_lc crypto provider: {:?}", e));
    }
    
    let addr: SocketAddr = "127.0.0.1:5250".parse()?;

    let cert: CertifiedKey<KeyPair> = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
    let key: PrivateKeyDer = PrivatePkcs8KeyDer::from(cert.signing_key.serialize_der()).into();
    let certs: Vec<CertificateDer> = vec![cert.cert.into()];

    let server_crypto: rustls::ServerConfig = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)?;

    let server_config: quinn::ServerConfig = quinn::ServerConfig::with_crypto(Arc::new(QuicServerConfig::try_from(server_crypto)?));

    let endpoint: quinn::Endpoint = quinn::Endpoint::server(server_config, addr)?;
    println!("Server listening on {addr}");

    loop {
        if let Some(conn) = endpoint.accept().await {
            if endpoint.open_connections() > 2 {
                println!("Can only have one connection during singleplayer mode");
                conn.refuse();
                continue;
            }
            tokio::spawn(async move {
                let addr = conn.remote_address();
                println!("Handling connection: {addr}");
                if let Err(e) = handle_connection(conn).await {
                    println!("Error handling connection from {addr}");
                    return;
                }
            });
        }
    }


    Ok(())
}