use crate::models::DistanceMarker;
use rusqlite::Connection;
use tauri::State;
use std::sync::Mutex;

type DbState<'a> = State<'a, Mutex<Connection>>;

#[tauri::command]
pub fn get_distance_markers(db: DbState<'_>) -> Result<Vec<DistanceMarker>, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, marker_number, distance FROM distance_markers ORDER BY marker_number")
        .map_err(|e| e.to_string())?;
    
    let markers = stmt
        .query_map([], |row| {
            Ok(DistanceMarker {
                id: Some(row.get(0)?),
                marker_number: row.get(1)?,
                distance: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(markers)
}

#[tauri::command]
pub fn upsert_distance_marker(
    marker_number: i32,
    distance: f64,
    db: DbState<'_>,
) -> Result<DistanceMarker, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    
    // Try to update first
    let updated = conn
        .execute(
            "UPDATE distance_markers SET distance = ?1 WHERE marker_number = ?2",
            rusqlite::params![distance, marker_number],
        )
        .map_err(|e| e.to_string())?;
    
    if updated == 0 {
        // Insert if not exists
        conn.execute(
            "INSERT INTO distance_markers (marker_number, distance) VALUES (?1, ?2)",
            rusqlite::params![marker_number, distance],
        )
        .map_err(|e| e.to_string())?;
    }
    
    // Fetch the marker
    let mut stmt = conn
        .prepare("SELECT id, marker_number, distance FROM distance_markers WHERE marker_number = ?1")
        .map_err(|e| e.to_string())?;
    
    let marker = stmt
        .query_row(rusqlite::params![marker_number], |row| {
            Ok(DistanceMarker {
                id: Some(row.get(0)?),
                marker_number: row.get(1)?,
                distance: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?;
    
    Ok(marker)
}

