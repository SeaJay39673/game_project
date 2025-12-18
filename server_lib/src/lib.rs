use crate::{
    server_networking::handle_connection, state::GameManager, thread_manager::ThreadManager,
};
use migration::{Migrator, MigratorTrait};
use quinn::{
    Endpoint,
    crypto::rustls::QuicServerConfig,
    rustls::pki_types::{CertificateDer, PrivateKeyDer},
};
use rcgen::{CertifiedKey, KeyPair};
use rustls::pki_types::PrivatePkcs8KeyDer;
use sea_orm::Database;
use std::{fs, net::SocketAddr, path::Path, sync::Arc};
use tokio::sync::{Notify};

mod server_networking;
mod server_session;
mod state;
pub mod thread_manager;

pub enum GameStartOption {
    NewGame(String),
    LoadGame(String),
}

async fn run_accept_loop(
    endpoint: Endpoint,
    thread_manager: Arc<ThreadManager>,
    game_manager: Arc<GameManager>,
) {
    let child = thread_manager.child().await;
    thread_manager
        .spawn_loop({
            let game_manager = game_manager.clone();
            move || {
                let game_manager = game_manager.clone();
                let endpoint = endpoint.clone();
                let child = child.clone();
                async move {
                    if let Some(conn) = endpoint.accept().await {
                        let gm = game_manager.clone();
                        let child_2 = child.child().await;
                        child
                            .spawn(move || async move {
                                let addr = conn.remote_address();
                                if let Err(e) = handle_connection(conn, gm, child_2).await {
                                    println!("Error handling connection from {addr}: {e}");
                                }
                            })
                            .await;
                    }
                }
            }
        })
        .await;
}

pub async fn start_single_player(
    option: GameStartOption,
    ready: Arc<Notify>,
    thread_manager: Arc<ThreadManager>,
) -> anyhow::Result<()> {

    let data_path = Path::new("src/data");
    if !data_path.exists() {
        fs::create_dir(data_path)?;
    }
    
    let db = Database::connect("sqlite://src/data/db.sqlite?mode=rwc").await?;
    Migrator::up(&db, None).await?;
    let game_manager = GameManager::new(db.clone(), option)?;

    let addr: SocketAddr = "127.0.0.1:5250".parse()?;

    let cert: CertifiedKey<KeyPair> = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
    let key: PrivateKeyDer = PrivatePkcs8KeyDer::from(cert.signing_key.serialize_der()).into();
    let certs: Vec<CertificateDer> = vec![cert.cert.into()];

    let server_crypto: rustls::ServerConfig = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)?;

    let server_config: quinn::ServerConfig =
        quinn::ServerConfig::with_crypto(Arc::new(QuicServerConfig::try_from(server_crypto)?));

    let endpoint: quinn::Endpoint = quinn::Endpoint::server(server_config, addr)?;
    println!("Server listening on {addr}");

    let _ = ready.notify_waiters();

    run_accept_loop(endpoint.clone(), thread_manager, game_manager).await;

    drop(db);

    Ok(())
}
