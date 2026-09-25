pub struct Peer {
    pub id: String,
    pub address: String,
    pub port: u16,
    pub socket_addr: std::net::SocketAddr
}

