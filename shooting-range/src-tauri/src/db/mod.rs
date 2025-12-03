use rusqlite::Connection;
use tauri::Manager;

pub fn init_db(app_handle: &tauri::AppHandle) -> Result<Connection, Box<dyn std::error::Error>> {
    let app_data = app_handle
        .path()
        .app_data_dir()
        .expect("Failed to get app data directory");
    
    std::fs::create_dir_all(&app_data)?;
    let db_path = app_data.join("shooting-range.db");
    
    let conn = Connection::open(db_path)?;
    
    // Run migrations
    conn.execute_batch(include_str!("schema.sql"))?;
    
    Ok(conn)
}

