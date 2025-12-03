# Implementation Guide - Step by Step

## Prerequisites
- Tauri 2.0 project initialized
- Vue 3 with Composition API
- SQLite database setup
- Assets copied from controller-pi folder

## Step 1: Database Setup

### 1.1 Add SQLite Dependency
Update `src-tauri/Cargo.toml`:
```toml
[dependencies]
rusqlite = { version = "0.31", features = ["bundled"] }
tauri-plugin-sql = { version = "2", features = ["sqlite"] }
```

### 1.2 Database Schema File
Create `src-tauri/src/db/schema.sql`:
```sql
-- Targets table
CREATE TABLE IF NOT EXISTS targets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    node_id INTEGER UNIQUE NOT NULL,
    distance REAL NOT NULL,
    image_num INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Games table
CREATE TABLE IF NOT EXISTS games (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    total_time INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Game targets junction table
CREATE TABLE IF NOT EXISTS game_targets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    game_id INTEGER NOT NULL,
    target_id INTEGER NOT NULL,
    start_time INTEGER NOT NULL,
    end_time INTEGER NOT NULL,
    FOREIGN KEY (game_id) REFERENCES games(id) ON DELETE CASCADE,
    FOREIGN KEY (target_id) REFERENCES targets(id) ON DELETE CASCADE
);

-- Distance markers table
CREATE TABLE IF NOT EXISTS distance_markers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    marker_number INTEGER UNIQUE NOT NULL,
    distance REAL NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_game_targets_game_id ON game_targets(game_id);
CREATE INDEX IF NOT EXISTS idx_game_targets_target_id ON game_targets(target_id);
```

## Step 2: Rust Backend Structure

### 2.1 Database Module
Create `src-tauri/src/db/mod.rs`:
```rust
use rusqlite::{Connection, Result};
use std::path::PathBuf;
use tauri::api::path::app_data_dir;
use tauri::Config;

pub fn init_db(app_handle: &tauri::AppHandle) -> Result<Connection> {
    let app_data = app_data_dir(&app_handle.config()).unwrap();
    std::fs::create_dir_all(&app_data)?;
    let db_path = app_data.join("shooting-range.db");
    
    let conn = Connection::open(db_path)?;
    
    // Run migrations
    conn.execute_batch(include_str!("schema.sql"))?;
    
    Ok(conn)
}
```

### 2.2 Models
Create `src-tauri/src/models/mod.rs`:
```rust
pub mod target;
pub mod game;
pub mod distance_marker;

pub use target::Target;
pub use game::{Game, GameTarget};
pub use distance_marker::DistanceMarker;
```

Create `src-tauri/src/models/target.rs`:
```rust
use serde::{Deserialize, Serialize};
use rusqlite::Row;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub id: Option<i32>,
    pub node_id: i32,
    pub distance: f64,
    pub image_num: i32,
}

impl Target {
    pub fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Target {
            id: Some(row.get(0)?),
            node_id: row.get(1)?,
            distance: row.get(2)?,
            image_num: row.get(3)?,
        })
    }
    
    pub fn image_path(&self) -> String {
        format!("assets/ShootingTarget_graphics{}.png", self.image_num)
    }
}
```

### 2.3 Tauri Commands
Create `src-tauri/src/commands/targets.rs`:
```rust
use crate::models::Target;
use rusqlite::Connection;
use tauri::State;

type DbState<'a> = State<'a, std::sync::Mutex<Connection>>;

#[tauri::command]
pub fn get_all_targets(db: DbState<'_>) -> Result<Vec<Target>, String> {
    let conn = db.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, node_id, distance, image_num FROM targets ORDER BY node_id")
        .map_err(|e| e.to_string())?;
    
    let targets = stmt.query_map([], |row| {
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
    db: DbState<'_>
) -> Result<Target, String> {
    let conn = db.lock().unwrap();
    conn.execute(
        "INSERT INTO targets (node_id, distance, image_num) VALUES (?1, ?2, ?3)",
        [&node_id, &distance, &image_num]
    ).map_err(|e| e.to_string())?;
    
    let id = conn.last_insert_rowid() as i32;
    Ok(Target {
        id: Some(id),
        node_id,
        distance,
        image_num,
    })
}

#[tauri::command]
pub fn delete_target(id: i32, db: DbState<'_>) -> Result<(), String> {
    let conn = db.lock().unwrap();
    conn.execute("DELETE FROM targets WHERE id = ?1", [id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
```

## Step 3: Frontend Setup

### 3.1 Install Dependencies
```bash
npm install vue-router@4 pinia@2
```

### 3.2 Router Setup
Create `src/router/index.ts`:
```typescript
import { createRouter, createWebHistory } from 'vue-router'
import StartupScreen from '../views/StartupScreen.vue'
import ConfigScreen from '../views/ConfigScreen.vue'
import GameScreen from '../views/GameScreen.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', name: 'startup', component: StartupScreen },
    { path: '/config', name: 'config', component: ConfigScreen },
    { path: '/game', name: 'game', component: GameScreen },
  ]
})

export default router
```

### 3.3 Main App Structure
Update `src/main.ts`:
```typescript
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'

const app = createApp(App)
app.use(createPinia())
app.use(router)
app.mount('#app')
```

## Step 4: UI Components

### 4.1 GameScreen Layout Structure
```vue
<template>
  <div class="game-screen">
    <!-- Background Image -->
    <div class="background-image"></div>
    
    <!-- Content Overlay -->
    <div class="content-overlay">
      <!-- Left: Distance Markers -->
      <div class="distance-markers-panel">
        <div class="marker-header">
          <span>Distance marker No:</span>
          <span class="marker-number">{{ currentMarker }}</span>
        </div>
        <div v-for="(marker, idx) in distanceMarkers" :key="idx" class="marker-item">
          <label>Distance</label>
          <span>{{ marker.distance }} meter</span>
          <img :src="targetIcon" class="marker-icon" />
        </div>
      </div>
      
      <!-- Center: Node Grid -->
      <div class="node-grid-container">
        <div class="targets-header">
          <span>Targets available: Nodes</span>
          <span class="node-count">{{ nodes.length }}</span>
        </div>
        <div class="node-grid">
          <NodeTarget
            v-for="node in nodes"
            :key="node.id"
            :node="node"
            :active="isNodeActive(node.id)"
            :time-remaining="getTimeRemaining(node.id)"
          />
        </div>
      </div>
      
      <!-- Right: Controls -->
      <div class="controls-panel">
        <div class="menu-section">
          <button class="menu-btn">Menu</button>
        </div>
        
        <div class="timer-section">
          <div class="timer-label">TIMER</div>
          <div class="timer-display">
            {{ formatTime(remainingTime) }}
          </div>
        </div>
        
        <div class="duration-section">
          <label>Program duration:</label>
          <div class="duration-inputs">
            <input v-model.number="duration.hours" type="number" placeholder="Hrs" />
            <input v-model.number="duration.minutes" type="number" placeholder="Mins" />
            <input v-model.number="duration.seconds" type="number" placeholder="Secs" />
          </div>
        </div>
        
        <button class="start-btn" @click="startGame">Start</button>
      </div>
    </div>
  </div>
</template>
```

### 4.2 CSS Styling for GameScreen
```css
.game-screen {
  position: relative;
  width: 100vw;
  height: 100vh;
  overflow: hidden;
}

.background-image {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-image: url('/background-landscape.jpg');
  background-size: cover;
  background-position: center;
  z-index: 0;
}

.content-overlay {
  position: relative;
  z-index: 1;
  display: grid;
  grid-template-columns: 200px 1fr 300px;
  height: 100vh;
  padding: 20px;
  gap: 20px;
}

.node-grid {
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  grid-template-rows: repeat(4, 1fr);
  gap: 20px;
  padding: 20px;
}

.node-target {
  position: relative;
  display: flex;
  flex-direction: column;
  align-items: center;
  cursor: pointer;
  transition: transform 0.2s;
}

.node-target.active {
  border: 3px solid #10B981;
  border-radius: 50%;
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.7; }
}

.timer-display {
  width: 150px;
  height: 150px;
  border-radius: 50%;
  background: #000;
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 2rem;
  font-weight: bold;
  font-family: monospace;
}

.start-btn {
  width: 120px;
  height: 120px;
  border-radius: 50%;
  background: #EF4444;
  color: white;
  border: none;
  font-size: 1.5rem;
  font-weight: bold;
  cursor: pointer;
  transition: transform 0.2s;
}

.start-btn:hover {
  transform: scale(1.05);
}

.start-btn:active {
  transform: scale(0.95);
}
```

## Step 5: Game Logic

### 5.1 Game State Management
Create `src/stores/game.ts`:
```typescript
import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export const useGameStore = defineStore('game', () => {
  const isRunning = ref(false)
  const startTime = ref<number | null>(null)
  const currentTime = ref(0)
  const gameConfig = ref<any>(null)
  const activeNodes = ref<Set<number>>(new Set())
  
  const remainingTime = computed(() => {
    if (!gameConfig.value || !startTime.value) return 0
    const elapsed = currentTime.value
    return Math.max(0, gameConfig.value.total_time - elapsed)
  })
  
  async function startGame(config: any) {
    gameConfig.value = config
    isRunning.value = true
    startTime.value = Date.now()
    currentTime.value = 0
    
    // Start timer
    const interval = setInterval(() => {
      if (!isRunning.value) {
        clearInterval(interval)
        return
      }
      
      const elapsed = Math.floor((Date.now() - startTime.value!) / 1000)
      currentTime.value = elapsed
      
      // Update active nodes
      updateActiveNodes(elapsed)
      
      if (elapsed >= gameConfig.value.total_time) {
        endGame()
        clearInterval(interval)
      }
    }, 200)
  }
  
  function updateActiveNodes(elapsed: number) {
    const newActiveNodes = new Set<number>()
    
    for (const target of gameConfig.value.targets) {
      for (const [start, end] of target.active_times) {
        if (elapsed >= start && elapsed < end) {
          newActiveNodes.add(target.node_id)
          
          // Send serial command if state changed
          if (!activeNodes.value.has(target.node_id)) {
            invoke('send_target_command', {
              nodeId: target.node_id,
              state: true
            })
          }
        }
      }
    }
    
    // Deactivate nodes that are no longer active
    for (const nodeId of activeNodes.value) {
      if (!newActiveNodes.has(nodeId)) {
        invoke('send_target_command', {
          nodeId,
          state: false
        })
      }
    }
    
    activeNodes.value = newActiveNodes
  }
  
  async function endGame() {
    isRunning.value = false
    
    // Deactivate all nodes
    for (const nodeId of activeNodes.value) {
      await invoke('send_target_command', {
        nodeId,
        state: false
      })
    }
    
    activeNodes.value.clear()
  }
  
  return {
    isRunning,
    currentTime,
    remainingTime,
    activeNodes,
    startGame,
    endGame
  }
})
```

## Step 6: Serial Communication

### 6.1 Rust Serial Handler
Create `src-tauri/src/serial/handler.rs`:
```rust
use serialport::{SerialPort, SerialPortBuilder};
use std::sync::{Arc, Mutex};
use std::io::Write;

pub struct SerialHandler {
    port: Arc<Mutex<Option<Box<dyn SerialPort>>>>,
    port_name: String,
}

impl SerialHandler {
    pub fn new(port_name: String) -> Self {
        Self {
            port: Arc::new(Mutex::new(None)),
            port_name,
        }
    }
    
    pub fn connect(&self) -> Result<(), String> {
        let port = serialport::new(&self.port_name, 115200)
            .timeout(std::time::Duration::from_millis(1000))
            .open()
            .map_err(|e| format!("Failed to open port: {}", e))?;
        
        *self.port.lock().unwrap() = Some(port);
        Ok(())
    }
    
    pub fn send_command(&self, node_id: i32, state: bool) -> Result<(), String> {
        let mut port_guard = self.port.lock().unwrap();
        if let Some(ref mut port) = *port_guard {
            let command = format!("{},{}\n", node_id, if state { "true" } else { "false" });
            port.write_all(command.as_bytes())
                .map_err(|e| format!("Failed to write: {}", e))?;
            Ok(())
        } else {
            Err("Port not connected".to_string())
        }
    }
    
    pub fn disconnect(&self) {
        *self.port.lock().unwrap() = None;
    }
}
```

### 6.2 Tauri Command for Serial
Add to `src-tauri/src/lib.rs`:
```rust
mod serial;
use serial::handler::SerialHandler;
use std::sync::Mutex;

#[tauri::command]
fn send_target_command(
    node_id: i32,
    state: bool,
    serial: tauri::State<Mutex<SerialHandler>>
) -> Result<(), String> {
    let handler = serial.lock().unwrap();
    handler.send_command(node_id, state)
}
```

## Step 7: Asset Management

### 7.1 Copy Assets
```bash
# Copy target graphics
cp build/macky-shooting-range-delivery/controller-pi/assets/*.png shooting-range/src/assets/

# Add background image to public folder
# (User needs to provide or create background-landscape.jpg)
```

### 7.2 Asset Import Helper
Create `src/utils/assets.ts`:
```typescript
export function getTargetImage(imageNum: number): string {
  return new URL(`../assets/ShootingTarget_graphics${imageNum}.png`, import.meta.url).href
}
```

## Implementation Checklist

- [ ] Step 1: Database setup and schema
- [ ] Step 2: Rust models and database operations
- [ ] Step 3: Tauri commands for CRUD operations
- [ ] Step 4: Serial communication handler
- [ ] Step 5: Vue router setup
- [ ] Step 6: StartupScreen component
- [ ] Step 7: ConfigScreen component
- [ ] Step 8: GameScreen layout
- [ ] Step 9: NodeGrid component
- [ ] Step 10: Distance marker panel
- [ ] Step 11: Timer display
- [ ] Step 12: Game state management
- [ ] Step 13: Serial command integration
- [ ] Step 14: Styling and animations
- [ ] Step 15: Background image integration
- [ ] Step 16: Testing and bug fixes

## Next Steps

1. Start with database setup
2. Implement basic Rust backend
3. Create minimal frontend components
4. Connect frontend to backend
5. Add styling and polish
6. Test with real serial device

