use core::error;

use crate::domain::{peer::Peer, connection::PeerConnection};
use crate::interface::events::{message_received, message_sent, user_connected, error_occurred};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, BufReader, AsyncWriteExt, AsyncReadExt};
use crate::domain::error::Error;

use tauri::{EventTarget::App, Manager};
use crate::application::state::AppState;
use tauri::AppHandle;

use crate::infrastructure::puertos::bind_libre;

pub async fn accept_any_connection_loop(handle: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let listener = bind_libre(4000).await?;

    loop {
        let (stream, addr) = match listener.accept().await {
            Ok(connection) => connection,
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
                continue;
            }
        };

        let peer = Peer::new(addr.ip().to_string(), addr.port());
        user_connected(&handle, &peer.id);
        crate::application::connect_peer::register_connection(peer, stream, handle.clone());
    }
}

pub async fn try_to_connect_to(peer: &Peer, handle: AppHandle) -> Result<TcpStream, Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", peer.address, peer.port);
    let stream = match TcpStream::connect(addr).await {
        Ok(stream) => { user_connected(&handle, &peer.id); stream },
        Err(_) => { error_occurred(&handle, "Error connecting to peer"); return Err("Error connecting to peer".into()); },
    };

    Ok(stream)
}

pub fn write_and_read_stream_threads(stream: TcpStream, handle: tauri::AppHandle) -> Result<Sender<String>, Box<dyn std::error::Error>> {
    let (stream_reader, mut stream_writer) = stream.into_split();
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(32);

    let handle_clone = handle.clone();

    tokio::spawn(async move {
        let mut reader = BufReader::new(stream_reader).lines();
        while let Ok(Some(line)) = reader.next_line().await {
            println!("~ {}", line);
            // Emit the received message
            message_received(&handle, &line);
        }
    });

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Err(e) = stream_writer.write_all(msg.as_bytes()).await {
                eprintln!("Error sending message: {}", e);
                break;
            }

            message_sent(&handle_clone, &msg);
        }
    });

    Ok(tx)
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