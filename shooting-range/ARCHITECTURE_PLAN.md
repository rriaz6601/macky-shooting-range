# Shooting Range Controller - Architecture Plan

## Overview
Migration from PyQt6 to Tauri + Vue 3 + SQLite with a modern, vivid frontend matching the mobile app design shown in the reference images.

## Technology Stack

### Frontend
- **Framework**: Vue 3 (Composition API)
- **Build Tool**: Vite
- **UI Library**: Custom components with modern CSS (CSS Grid, Flexbox)
- **State Management**: Pinia (optional, for complex state)
- **Styling**: CSS Modules or Scoped Styles

### Backend (Rust)
- **Framework**: Tauri 2.0
- **Database**: SQLite with `rusqlite` or `sqlx`
- **Serial Communication**: `serialport` (already in dependencies)
- **Error Handling**: `anyhow` (already in dependencies)

## Database Schema

### Tables

#### 1. `targets` - Physical target nodes
```sql
CREATE TABLE targets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    node_id INTEGER UNIQUE NOT NULL,
    distance REAL NOT NULL,
    image_num INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

#### 2. `games` - Game configurations
```sql
CREATE TABLE games (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    total_time INTEGER NOT NULL, -- in seconds
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

#### 3. `game_targets` - Target instances in games
```sql
CREATE TABLE game_targets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    game_id INTEGER NOT NULL,
    target_id INTEGER NOT NULL,
    start_time INTEGER NOT NULL, -- in seconds
    end_time INTEGER NOT NULL,   -- in seconds
    FOREIGN KEY (game_id) REFERENCES games(id) ON DELETE CASCADE,
    FOREIGN KEY (target_id) REFERENCES targets(id) ON DELETE CASCADE
);
```

#### 4. `distance_markers` - Distance marker configurations
```sql
CREATE TABLE distance_markers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    marker_number INTEGER UNIQUE NOT NULL,
    distance REAL NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

## Rust Backend Structure

### File Organization
```
src-tauri/src/
├── main.rs              # Entry point
├── lib.rs               # Tauri app setup
├── db/
│   ├── mod.rs           # Database module
│   ├── schema.rs        # SQL schema definitions
│   └── migrations.rs    # Database migrations
├── models/
│   ├── mod.rs
│   ├── target.rs        # Target model
│   ├── game.rs          # Game model
│   └── game_target.rs   # GameTarget model
├── commands/
│   ├── mod.rs
│   ├── targets.rs       # Target CRUD commands
│   ├── games.rs         # Game CRUD commands
│   ├── distance_markers.rs
│   └── serial.rs        # Serial communication commands
└── serial/
    ├── mod.rs
    └── handler.rs       # Serial port handler
```

### Key Rust Commands (Tauri)

#### Target Commands
- `get_all_targets() -> Vec<Target>`
- `create_target(node_id: i32, distance: f64, image_num: i32) -> Result<Target>`
- `update_target(id: i32, node_id: i32, distance: f64, image_num: i32) -> Result<Target>`
- `delete_target(id: i32) -> Result<()>`

#### Game Commands
- `get_all_games() -> Vec<Game>`
- `get_game(id: i32) -> Result<Game>`
- `create_game(name: String, total_time: i32, targets: Vec<GameTargetInput>) -> Result<Game>`
- `update_game(id: i32, name: String, total_time: i32, targets: Vec<GameTargetInput>) -> Result<Game>`
- `delete_game(id: i32) -> Result<()>`

#### Serial Commands
- `send_target_command(node_id: i32, state: bool) -> Result<()>`
- `get_serial_ports() -> Vec<String>`
- `connect_serial(port: String) -> Result<()>`
- `disconnect_serial() -> Result<()>`

#### Distance Marker Commands
- `get_distance_markers() -> Vec<DistanceMarker>`
- `update_distance_marker(marker_number: i32, distance: f64) -> Result<DistanceMarker>`

## Frontend Structure

### Component Hierarchy
```
App.vue
├── RouterView
    ├── StartupScreen.vue
    │   ├── GameList.vue
    │   └── ConfigureButton.vue
    ├── ConfigScreen.vue
    │   ├── TargetTable.vue
    │   ├── GameTable.vue
    │   ├── DistanceMarkerPanel.vue
    │   └── AddTargetDialog.vue
    └── GameScreen.vue
        ├── TimerDisplay.vue
        ├── NodeGrid.vue
        │   └── NodeTarget.vue (x20)
        ├── DistanceMarkerPanel.vue
        ├── ProgramDurationInput.vue
        └── StartButton.vue
```

### Key Vue Components

#### 1. StartupScreen.vue
- Game list table
- "Configure System" button
- Double-click to start game

#### 2. ConfigScreen.vue
- Target configuration table
- Game configuration table
- Distance marker settings
- Add/Edit/Delete functionality
- Save/Load config buttons

#### 3. GameScreen.vue
- **Left Panel**: Distance markers (4 markers with target icons)
- **Center**: Node grid (20 nodes arranged 4x5)
- **Right Panel**: 
  - Menu button
  - Timer display (large circular)
  - Program duration inputs (Hrs, Mins, Secs)
  - Start button (large red circular)
- Background: Landscape image (sky + field)

#### 4. NodeTarget.vue
- Circular target icon
- Node label
- Active/inactive state visualization
- Time remaining display

#### 5. DistanceMarkerPanel.vue
- Distance marker number selector
- Distance input fields
- Target icon display

## UI Design Specifications

### Color Scheme
- **Primary**: Modern blue (#3B82F6)
- **Active Target**: Green border (#10B981)
- **Inactive Target**: Gray (#6B7280)
- **Start Button**: Red (#EF4444)
- **Background**: Natural landscape image

### Layout
- **Responsive Grid**: CSS Grid for node arrangement
- **Flexbox**: For panels and controls
- **Background Image**: Full-screen with overlay content
- **Card-based**: Modern card design for panels

### Typography
- **Font**: System font stack (Inter, Avenir, Helvetica)
- **Headings**: Bold, larger size
- **Labels**: Medium weight
- **Timer**: Large, monospace font

### Animations
- Smooth transitions for state changes
- Pulse animation for active targets
- Fade in/out for dialogs

## Data Flow

### Game Execution Flow
1. User selects game from StartupScreen
2. GameScreen loads with game configuration
3. Timer starts counting down
4. For each target in game:
   - Check if current time is within active window
   - Send serial command to activate/deactivate
   - Update UI state
5. On game end, deactivate all targets

### Serial Communication
- Format: `{node_id},{true|false}\n`
- Baud rate: 115200
- Port: Configurable (default: `/dev/ttyACM0` on Linux)

## Implementation Phases

### Phase 1: Database & Backend Foundation
1. Set up SQLite database with schema
2. Implement Rust models
3. Create database commands
4. Set up serial communication handler

### Phase 2: Basic Frontend
1. Create router setup
2. Build StartupScreen
3. Build ConfigScreen (basic)
4. Connect to backend commands

### Phase 3: Game Screen UI
1. Create GameScreen layout
2. Implement NodeGrid component
3. Add distance marker panel
4. Style with background image

### Phase 4: Game Logic
1. Implement timer logic
2. Connect serial commands
3. Real-time target state updates
4. Game start/stop functionality

### Phase 5: Polish & Enhancement
1. Add animations
2. Improve styling
3. Error handling
4. Loading states
5. Configuration persistence

## Dependencies to Add

### Rust (Cargo.toml)
```toml
[dependencies]
rusqlite = { version = "0.31", features = ["bundled"] }
# or
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "sqlite"] }
tokio = { version = "1", features = ["full"] } # if using sqlx
```

### Frontend (package.json)
```json
{
  "dependencies": {
    "vue-router": "^4.2.5",
    "pinia": "^2.1.7" // optional
  }
}
```

## Asset Management

### Asset Structure
```
shooting-range/
├── public/
│   └── background-landscape.jpg  # Main background image
└── src/
    └── assets/
        ├── ShootingTarget_graphics.png
        ├── ShootingTarget_graphics2.png
        └── ... (copy from controller-pi/assets)
```

### Asset Copying
- Copy all target graphics from `build/macky-shooting-range-delivery/controller-pi/assets/` to `shooting-range/src/assets/`
- Add background landscape image to `public/`

## Configuration

### Tauri Configuration
- Window size: Match mobile app dimensions (or responsive)
- Fullscreen option: Optional
- Title: "Shooting Range Controller"

### Database Location
- Default: `{app_data_dir}/shooting-range.db`
- Use Tauri's `app_data_dir()` for cross-platform support

## Testing Strategy

1. **Unit Tests**: Rust models and database operations
2. **Integration Tests**: Tauri commands
3. **E2E Tests**: Game flow with mock serial port
4. **UI Tests**: Component rendering and interactions

## Migration Notes

### From PyQt6 to Tauri
- JSON config → SQLite database
- QStackedWidget → Vue Router
- QTimer → JavaScript setInterval/requestAnimationFrame
- Serial port handling → Rust backend
- File dialogs → Tauri dialog API

### Data Migration
- Script to convert existing `config.json` to SQLite
- Preserve all existing targets and games

## Future Enhancements

1. **Real-time Statistics**: Hit tracking, accuracy
2. **Game Templates**: Pre-configured game types
3. **Export/Import**: Share game configurations
4. **Multi-language Support**: i18n
5. **Dark Mode**: Theme switching
6. **Sound Effects**: Audio feedback
7. **Network Mode**: Remote control capability

