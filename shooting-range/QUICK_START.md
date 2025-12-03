# Quick Start Guide

## Overview
This document provides a quick reference for the Shooting Range Controller migration from PyQt6 to Tauri + Vue 3 + SQLite.

## Key Changes from PyQt6

| PyQt6 | Tauri + Vue 3 |
|-------|---------------|
| JSON config file | SQLite database |
| QStackedWidget | Vue Router |
| QTimer | JavaScript setInterval |
| Python serial | Rust serialport |
| QWidget components | Vue components |

## Project Structure

```
shooting-range/
├── src/                          # Vue frontend
│   ├── views/
│   │   ├── StartupScreen.vue
│   │   ├── ConfigScreen.vue
│   │   └── GameScreen.vue
│   ├── components/
│   │   ├── NodeTarget.vue
│   │   ├── DistanceMarkerPanel.vue
│   │   └── TimerDisplay.vue
│   ├── stores/                   # Pinia stores
│   │   └── game.ts
│   ├── router/
│   │   └── index.ts
│   └── assets/                   # Target images
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── db/                   # Database module
│   │   ├── models/               # Data models
│   │   ├── commands/             # Tauri commands
│   │   └── serial/               # Serial handler
│   └── Cargo.toml
└── public/                       # Static assets
    └── background-landscape.jpg
```

## Database Schema Summary

### Tables
1. **targets** - Physical target nodes (node_id, distance, image_num)
2. **games** - Game configurations (name, total_time)
3. **game_targets** - Target instances in games (start_time, end_time)
4. **distance_markers** - Distance marker settings

## Key Tauri Commands

### Targets
- `get_all_targets()` → `Vec<Target>`
- `create_target(node_id, distance, image_num)` → `Target`
- `delete_target(id)` → `Result<()>`

### Games
- `get_all_games()` → `Vec<Game>`
- `create_game(name, total_time, targets)` → `Game`
- `delete_game(id)` → `Result<()>`

### Serial
- `send_target_command(node_id, state)` → `Result<()>`
- `connect_serial(port)` → `Result<()>`

## UI Layout (GameScreen)

```
┌─────────────────────────────────────────────────────────┐
│  [Distance Markers]  │  [Node Grid]  │  [Controls]     │
│                       │               │                 │
│  Marker No: 4         │  Node:1 ...   │  Menu           │
│  10 meter [icon]      │  Node:2 ...   │  TIMER: 0:00    │
│  20 meter [icon]      │  ...          │  Duration:      │
│  30 meter [icon]      │  Node:20      │  [Hrs][Min][Sec]│
│  40 meter [icon]      │               │  [Start]        │
└─────────────────────────────────────────────────────────┘
```

## Component Props & Events

### NodeTarget.vue
```typescript
interface Props {
  node: {
    id: number
    node_id: number
    distance: number
    image_num: number
  }
  active: boolean
  timeRemaining: number
}
```

### GameScreen.vue
```typescript
// State
const gameStore = useGameStore()
const nodes = ref<Target[]>([])
const distanceMarkers = ref<DistanceMarker[]>([])

// Methods
function startGame() { ... }
function endGame() { ... }
function formatTime(seconds: number): string { ... }
```

## Serial Protocol

**Format**: `{node_id},{true|false}\n`
**Example**: `1,true\n` or `2,false\n`
**Baud Rate**: 115200
**Default Port**: `/dev/ttyACM0` (Linux)

## Styling Guidelines

### Colors
- Primary Blue: `#3B82F6`
- Active Green: `#10B981`
- Start Red: `#EF4444`
- Background: Natural landscape image

### Layout
- CSS Grid for node arrangement (4x5)
- Flexbox for panels
- Full-screen background with overlay

### Animations
- Pulse for active targets
- Smooth transitions (0.2s)
- Fade in/out for dialogs

## Development Workflow

1. **Backend First**: Set up database and Rust commands
2. **Frontend Shell**: Create component structure
3. **Connect**: Wire up Tauri commands
4. **Polish**: Add styling and animations
5. **Test**: Verify with serial device

## Dependencies

### Rust (Cargo.toml)
```toml
rusqlite = { version = "0.31", features = ["bundled"] }
serialport = "4.6.0"  # Already added
anyhow = "1.0"        # Already added
```

### Frontend (package.json)
```json
{
  "vue-router": "^4.2.5",
  "pinia": "^2.1.7"
}
```

## Migration Path

1. ✅ Tauri project initialized
2. ⏳ Database schema created
3. ⏳ Rust models implemented
4. ⏳ Tauri commands added
5. ⏳ Vue components created
6. ⏳ Serial integration
7. ⏳ Styling and polish

## Reference Documents

- `ARCHITECTURE_PLAN.md` - Full architecture details
- `IMPLEMENTATION_GUIDE.md` - Step-by-step code examples
- `QUICK_START.md` - This document

## Next Actions

1. Review architecture plan
2. Set up database schema
3. Implement basic Rust backend
4. Create Vue component structure
5. Connect frontend to backend
6. Test and iterate

