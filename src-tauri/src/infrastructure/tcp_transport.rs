use core::error;

use crate::domain::{peer::Peer, connection::PeerConnection};
use tokio::net::{TcpListener, TcpStream};

pub async fn listen_to_peer(peer: &Peer) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:4000").await?;

    loop {
        let (stream, addr) = listener.accept().await?;
    }
}

pub async fn connect_to_peer(peer: &Peer) -> Result<PeerConnection, Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", peer.address, peer.port);
    let stream = TcpStream::connect(addr).await?;

    let connection = PeerConnection {
        stream,
        peer: Peer {
            id: peer.id.clone(),
            address: peer.address.clone(),
            port: peer.port,
            socket_addr: peer.socket_addr,
        }
    };
    Ok(connection)
}

/*
No copies el código en los dos. Crea una función aparte en el mismo archivo que:

- reciba el TcpStream como parámetro,
- haga la división y lance las dos tareas,
- devuelva el Sender (mpsc::Sender<String>).

listen_to_peer:   accept()  ──► stream ──┐
                                         ├──► función nueva(stream) ──► Sender
connect_to_peer:  connect() ──► stream ──┘
*/