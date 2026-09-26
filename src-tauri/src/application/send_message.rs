// enviar mensaje en texto sin "saber" que es TCP/UDP...
use crate::domain::connection::PeerConnection;

pub async fn send_message_to_peer(peer_connection: PeerConnection, message: String) {
    peer_connection.sender.send(message).await.unwrap();
}