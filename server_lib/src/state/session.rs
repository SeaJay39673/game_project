use std::{net::SocketAddr, sync::Arc};

use dashmap::DashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionRole {
    LocalMaster,
    RemoteClient,
}

#[derive(Debug, Clone)]
pub enum AuthState {
    WaitingForCredentials,
    Authenticated { username: String },
    Rejected(String),
}

#[derive(Debug, Clone)]
pub struct ServerSession {
    pub role: ConnectionRole,
    pub auth: AuthState,
    pub addr: SocketAddr,
    pub control_stream_opened: bool,
}

impl ServerSession {
    pub fn new(addr: SocketAddr) -> Self {
        let role = if addr.ip().is_loopback() {
            ConnectionRole::LocalMaster
        } else {
            ConnectionRole::RemoteClient
        };

        Self {
            role,
            auth: AuthState::WaitingForCredentials,
            addr,
            control_stream_opened: false,
        }
    }
}

pub struct SessionManager {
    pub sessions: DashMap<SocketAddr, Arc<tokio::sync::Mutex<ServerSession>>>,
}

impl SessionManager {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            sessions: DashMap::new(),
        })
    }
}