// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod interface;
mod infrastructure;
mod domain;
mod application;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(
                tauri::generate_handler![
                        interface::commands::send_message, 
                        interface::commands::receive_message, 
                        interface::commands::connect_peer, 
                        interface::commands::disconnect_peer
                ]
        )
        .manage(
        application::state::AppState {
                peer_connections: std::sync::Mutex::new(Vec::new()),
        })
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = infrastructure::tcp_transport::accept_any_connection_loop(handle).await {
                    eprintln!("Listener stopped: {}", e);
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
