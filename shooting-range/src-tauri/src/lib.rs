mod db;
mod models;
mod commands;
mod serial;

use db::init_db;
use serial::SerialHandler;
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Initialize database
            let db = init_db(app.handle())?;
            app.manage(Mutex::new(db));
            
            // Initialize serial handler (default port)
            let serial_handler = SerialHandler::new("/dev/ttyACM0".to_string());
            // Try to connect, but don't fail if it doesn't work
            let _ = serial_handler.connect();
            app.manage(Mutex::new(serial_handler));
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Target commands
            commands::get_all_targets,
            commands::create_target,
            commands::update_target,
            commands::delete_target,
            // Game commands
            commands::get_all_games,
            commands::get_game,
            commands::create_game,
            commands::update_game,
            commands::delete_game,
            // Distance marker commands
            commands::get_distance_markers,
            commands::upsert_distance_marker,
            // Serial commands
            commands::send_target_command,
            commands::get_serial_ports,
            commands::connect_serial,
            commands::disconnect_serial,
            commands::is_serial_connected,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
