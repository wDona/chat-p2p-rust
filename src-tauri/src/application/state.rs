use std::sync::Mutex;
use crate::domain::{peer::Peer, connection::PeerConnection};

#[derive(Debug)]
pub struct AppState {
    pub peer_connections: Mutex<Vec<PeerConnection>>
}