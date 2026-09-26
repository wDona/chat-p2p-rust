use crate::application::state::AppState;
use crate::domain::connection::PeerConnection;
use crate::domain::peer::Peer;
use crate::infrastructure::tcp_transport::write_and_read_stream_threads;
use tauri::AppHandle;
use tauri::Manager;
use tokio::net::TcpStream;
use crate::infrastructure::tcp_transport::try_to_connect_to;


pub fn create_peer_connection(peer_connection: PeerConnection, handle: AppHandle) {
    let state = handle.state::<AppState>();

    state.peer_connections.lock().unwrap().push(peer_connection);
}

pub fn try_to_connect_to_peer(peer: Peer, handle: AppHandle)  {
    tauri::async_runtime::spawn(async move {

        let stream = match try_to_connect_to(&peer, handle.clone()).await {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Error connecting to peer: {}", e);
                return;
            }
        };

        register_connection(peer, stream, handle);
    });

}

pub fn register_connection(peer: Peer, stream: TcpStream, handle: AppHandle) {
    let sender = match write_and_read_stream_threads(stream, handle.clone()) {
        Ok(sender) => sender,
        Err(e) => {
            eprintln!("Error setting up stream threads: {}", e);
            return;
        }
    };

    let peer_connection = PeerConnection { peer, sender };
    create_peer_connection(peer_connection, handle);
}
