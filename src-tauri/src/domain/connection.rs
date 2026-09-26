use tokio::net::{TcpStream};
use tokio::sync::mpsc::Sender;
use crate::domain::{peer::Peer};

#[derive(Debug, Clone)]
pub struct PeerConnection {
    pub sender: Sender<String>,
    pub peer: Peer,
}