use std::os::linux::raw::stat;

use crate::application::{self, state};
use crate::application::{connect_peer::try_to_connect_to_peer, send_message};
use crate::domain::peer::Peer;


#[tauri::command]
pub fn greet(name: &str) -> String {
    String::from(format!("Hello, {}! You've been greeted from Rust!", name))
}

#[tauri::command]
pub async fn send_message(message: &str, handle: tauri::AppHandle, state: tauri::State<'_, state::AppState>) -> Result<(), String> {
    // Aquí iría la lógica para enviar el mensaje a través de la red
    let peer_connection = {
        let connections = state.peer_connections.lock().unwrap();
        connections.last().ok_or("Sin conexiones")?.clone()
    };

    send_message::send_message_to_peer(peer_connection, message.to_string() + "\n").await;
    Ok(())
}

#[tauri::command]
pub fn receive_message() {
    // Aquí iría la lógica para recibir un mensaje de la red
    
}

#[tauri::command]
pub fn connect_peer(address: &str, port: u16, handle: tauri::AppHandle) {
    let peer = Peer::new(address.to_string(), port);
    
    try_to_connect_to_peer(peer, handle);
}

#[tauri::command]
pub fn disconnect_peer(address: &str, port: u16) -> String {
    // Aquí iría la lógica para desconectar de un peer en la red
    String::from(format!("Disconnected from peer at {}:{}", address, port))
}

#[tauri::command]
pub fn handle_error(error: &str) -> String {
    // Aquí iría la lógica para manejar errores
    String::from(format!("Error handled: {}", error))
}

pub fn save_message(message: &str) -> String {
    // Aquí iría la lógica para guardar un mensaje en la base de datos
    String::from(format!("Message saved: {}", message))
}

