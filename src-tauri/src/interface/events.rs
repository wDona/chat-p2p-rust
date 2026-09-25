use tauri::{AppHandle, Emitter, Manager};

pub fn user_disconnect() {
    // Lógica para manejar la desconexión del usuario
}

pub fn user_connect() {
    // Lógica para manejar la conexión del usuario
}

pub fn message_received(message: &str) {
    // Lógica para manejar la recepción de un mensaje
}

pub fn message_sent(message: &str) {
    // Lógica para manejar el envío de un mensaje
}

pub fn error_occurred(error: &str) {
    // Lógica para manejar un error ocurrido
}
