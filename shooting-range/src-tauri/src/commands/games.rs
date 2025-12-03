use crate::models::{Game, GameTarget, GameTargetInput, Target};
use rusqlite::Connection;
use tauri::State;
use std::sync::Mutex;

type DbState<'a> = State<'a, Mutex<Connection>>;

#[tauri::command]
pub fn get_all_games(db: DbState<'_>) -> Result<Vec<Game>, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    
    // Get all games
    let mut stmt = conn
        .prepare("SELECT id, name, total_time FROM games ORDER BY name")
        .map_err(|e| e.to_string())?;
    
    let games: Vec<(i32, String, i32)> = stmt
        .query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    // For each game, get its targets
    let mut result = Vec::new();
    for (game_id, name, total_time) in games {
        let targets = get_game_targets(&conn, game_id)?;
        result.push(Game {
            id: Some(game_id),
            name,
            total_time,
            targets,
        });
    }
    
    Ok(result)
}

#[tauri::command]
pub fn get_game(id: i32, db: DbState<'_>) -> Result<Game, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    
    let mut stmt = conn
        .prepare("SELECT id, name, total_time FROM games WHERE id = ?1")
        .map_err(|e| e.to_string())?;
    
    let game = stmt
        .query_row(rusqlite::params![id], |row| {
            Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?, row.get::<_, i32>(2)?))
        })
        .map_err(|e| e.to_string())?;
    
    let targets = get_game_targets(&conn, game.0)?;
    
    Ok(Game {
        id: Some(game.0),
        name: game.1,
        total_time: game.2,
        targets,
    })
}

#[tauri::command]
pub fn create_game(
    name: String,
    total_time: i32,
    targets: Vec<GameTargetInput>,
    db: DbState<'_>,
) -> Result<Game, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    
    // Create game
    conn.execute(
        "INSERT INTO games (name, total_time) VALUES (?1, ?2)",
        rusqlite::params![name, total_time],
    )
    .map_err(|e| e.to_string())?;
    
    let game_id = conn.last_insert_rowid() as i32;
    
    // Add targets
    for target_input in &targets {
        conn.execute(
            "INSERT INTO game_targets (game_id, target_id, start_time, end_time) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![game_id, target_input.target_id, target_input.start_time, target_input.end_time],
        )
        .map_err(|e| e.to_string())?;
    }
    
    // Fetch the complete game
    let targets = get_game_targets(&conn, game_id)?;
    Ok(Game {
        id: Some(game_id),
        name,
        total_time,
        targets,
    })
}

#[tauri::command]
pub fn update_game(
    id: i32,
    name: String,
    total_time: i32,
    targets: Vec<GameTargetInput>,
    db: DbState<'_>,
) -> Result<Game, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    
    // Update game
    conn.execute(
        "UPDATE games SET name = ?1, total_time = ?2, updated_at = datetime('now') WHERE id = ?3",
        rusqlite::params![name, total_time, id],
    )
    .map_err(|e| e.to_string())?;
    
    // Delete existing targets
    conn.execute("DELETE FROM game_targets WHERE game_id = ?1", rusqlite::params![id])
        .map_err(|e| e.to_string())?;
    
    // Add new targets
    for target_input in &targets {
        conn.execute(
            "INSERT INTO game_targets (game_id, target_id, start_time, end_time) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![id, target_input.target_id, target_input.start_time, target_input.end_time],
        )
        .map_err(|e| e.to_string())?;
    }
    
    // Fetch the complete game
    let targets = get_game_targets(&conn, id)?;
    Ok(Game {
        id: Some(id),
        name,
        total_time,
        targets,
    })
}

#[tauri::command]
pub fn delete_game(id: i32, db: DbState<'_>) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM games WHERE id = ?1", rusqlite::params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn get_game_targets(conn: &Connection, game_id: i32) -> Result<Vec<GameTarget>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT gt.id, gt.start_time, gt.end_time, t.id, t.node_id, t.distance, t.image_num
             FROM game_targets gt
             JOIN targets t ON gt.target_id = t.id
             WHERE gt.game_id = ?1
             ORDER BY gt.start_time",
        )
        .map_err(|e| e.to_string())?;
    
    let targets = stmt
        .query_map(rusqlite::params![game_id], |row| {
            Ok(GameTarget {
                id: Some(row.get(0)?),
                target: Target {
                    id: Some(row.get(3)?),
                    node_id: row.get(4)?,
                    distance: row.get(5)?,
                    image_num: row.get(6)?,
                },
                start_time: row.get(1)?,
                end_time: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(targets)
}

