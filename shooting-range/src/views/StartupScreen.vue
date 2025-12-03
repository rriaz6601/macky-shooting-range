<template>
  <div class="startup-screen">
    <div class="header">
      <h1>Shooting Range Controller</h1>
    </div>
    
    <div class="content">
      <button class="configure-btn" @click="goToConfig">
        Configure System
      </button>
      
      <div class="games-section">
        <h2>Available Games</h2>
        <div v-if="loading" class="loading">Loading games...</div>
        <div v-else-if="games.length === 0" class="empty">
          No games configured. Click "Configure System" to create one.
        </div>
        <table v-else class="games-table">
          <thead>
            <tr>
              <th>Game Name</th>
              <th>Duration (seconds)</th>
            </tr>
          </thead>
          <tbody>
            <tr
              v-for="game in games"
              :key="game.id"
              @dblclick="startGame(game)"
              class="game-row"
            >
              <td>{{ game.name }}</td>
              <td>{{ game.total_time }}s</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { useGameStore } from '../stores/game'

const router = useRouter()
const gameStore = useGameStore()

interface Game {
  id?: number
  name: string
  total_time: number
  targets: any[]
}

const games = ref<Game[]>([])
const loading = ref(true)

async function loadGames() {
  try {
    loading.value = true
    games.value = await invoke<Game[]>('get_all_games')
  } catch (error) {
    console.error('Failed to load games:', error)
  } finally {
    loading.value = false
  }
}

function goToConfig() {
  router.push('/config')
}

function startGame(game: Game) {
  gameStore.startGame(game)
  router.push('/game')
}

onMounted(() => {
  loadGames()
})
</script>

<style scoped>
.startup-screen {
  width: 100vw;
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.header {
  padding: 2rem;
  text-align: center;
}

.header h1 {
  font-size: 2.5rem;
  font-weight: bold;
}

.content {
  flex: 1;
  padding: 2rem;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2rem;
}

.configure-btn {
  padding: 1rem 2rem;
  font-size: 1.2rem;
  background: #10b981;
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: transform 0.2s, box-shadow 0.2s;
}

.configure-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

.configure-btn:active {
  transform: translateY(0);
}

.games-section {
  width: 100%;
  max-width: 800px;
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(10px);
  border-radius: 12px;
  padding: 2rem;
}

.games-section h2 {
  margin-bottom: 1rem;
  font-size: 1.5rem;
}

.games-table {
  width: 100%;
  border-collapse: collapse;
}

.games-table thead {
  background: rgba(255, 255, 255, 0.2);
}

.games-table th {
  padding: 1rem;
  text-align: left;
  font-weight: bold;
}

.games-table td {
  padding: 1rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.game-row {
  cursor: pointer;
  transition: background 0.2s;
}

.game-row:hover {
  background: rgba(255, 255, 255, 0.1);
}

.loading, .empty {
  text-align: center;
  padding: 2rem;
  color: rgba(255, 255, 255, 0.8);
}
</style>

