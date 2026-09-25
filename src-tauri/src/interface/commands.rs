#[tauri::command]
pub fn greet(name: &str) -> String {
    String::from(format!("Hello, {}! You've been greeted from Rust!", name))
}

#[tauri::command]
pub fn send_message(message: &str) -> String {
    // Aquí iría la lógica para enviar el mensaje a través de la red
    String::from(format!("Message sent: {}", message))
}

#[tauri::command]
pub fn receive_message() -> String {
    // Aquí iría la lógica para recibir un mensaje de la red
    String::from("Received message: Hello from the network!")
}

#[tauri::command]
pub fn connect_peer(address: &str, port: u16) -> String {
    // Aquí iría la lógica para conectar con un peer en la red
    String::from(format!("Connected to peer at {}:{}", address, port))
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

