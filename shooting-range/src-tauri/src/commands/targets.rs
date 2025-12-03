use crate::models::Target;
use rusqlite::Connection;
use tauri::State;
use std::sync::Mutex;

type DbState<'a> = State<'a, Mutex<Connection>>;

#[tauri::command]
pub fn get_all_targets(db: DbState<'_>) -> Result<Vec<Target>, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, node_id, distance, image_num FROM targets ORDER BY node_id")
        .map_err(|e| e.to_string())?;
    
    let targets = stmt
        .query_map([], |row| {
            Ok(Target {
                id: Some(row.get(0)?),
                node_id: row.get(1)?,
                distance: row.get(2)?,
                image_num: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(targets)
}

#[tauri::command]
pub fn create_target(
    node_id: i32,
    distance: f64,
    image_num: i32,
    db: DbState<'_>,
) -> Result<Target, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO targets (node_id, distance, image_num) VALUES (?1, ?2, ?3)",
        rusqlite::params![node_id, distance, image_num],
    )
    .map_err(|e| e.to_string())?;
    
    let id = conn.last_insert_rowid() as i32;
    Ok(Target {
        id: Some(id),
        node_id,
        distance,
        image_num,
    })
}

#[tauri::command]
pub fn update_target(
    id: i32,
    node_id: i32,
    distance: f64,
    image_num: i32,
    db: DbState<'_>,
) -> Result<Target, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE targets SET node_id = ?1, distance = ?2, image_num = ?3, updated_at = datetime('now') WHERE id = ?4",
        rusqlite::params![node_id, distance, image_num, id],
    )
    .map_err(|e| e.to_string())?;
    
    Ok(Target {
        id: Some(id),
        node_id,
        distance,
        image_num,
    })
}

#[tauri::command]
pub fn delete_target(id: i32, db: DbState<'_>) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM targets WHERE id = ?1", rusqlite::params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

