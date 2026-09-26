use tauri::{AppHandle, Emitter, Manager};

pub fn user_disconnect(handle: &AppHandle, peer_id: &str) {
    // Lógica para manejar la desconexión del usuario
    handle.emit("user_disconnect", peer_id).unwrap();
}

pub fn user_connected(handle: &AppHandle, peer_id: &str) {
    // Lógica para manejar la conexión del usuario
    handle.emit("user_connected", peer_id).unwrap();
}

pub fn message_received(handle: &AppHandle, message: &str) {
    // Lógica para manejar la recepción de un mensaje
    handle.emit("message_received", message).unwrap();
}

pub fn message_sent(handle: &AppHandle, message: &str) {
    // Lógica para manejar el envío de un mensaje
    handle.emit("message_sent", message).unwrap();
}
pub fn error_occurred(handle: &AppHandle, error: &str) {
    // Lógica para manejar un error ocurrido
    handle.emit("error_occurred", error).unwrap();
}
