use tokio::net::TcpStream;

pub struct PeerConnection {
    pub stream: TcpStream,
    pub peer: super::peer::Peer,
}