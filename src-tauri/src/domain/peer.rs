#[derive(Debug, Clone)]
pub struct Peer {
    pub id: String,
    pub address: String,
    pub port: u16,
    pub socket_addr: std::net::SocketAddr
}

impl Peer {
    pub fn new(address: String, port: u16) -> Self {
        let id = format!("{}:{}", address, port);
        let socket_addr = format!("{}:{}", address, port).parse().unwrap();
        Peer { id, address, port, socket_addr }
    }
}