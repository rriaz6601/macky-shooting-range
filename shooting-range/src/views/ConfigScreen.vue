<template>
  <div class="config-screen">
    <div class="header">
      <button class="back-btn" @click="goBack">← Back</button>
      <h1>System Configuration</h1>
    </div>
    
    <div class="content">
      <!-- Targets Section -->
      <section class="config-section">
        <h2>Global Targets</h2>
        <button class="add-btn" @click="showAddTargetDialog = true">Add Target</button>
        <table class="config-table">
          <thead>
            <tr>
              <th>Node ID</th>
              <th>Distance (m)</th>
              <th>Image</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="target in targets" :key="target.id">
              <td>{{ target.node_id }}</td>
              <td>{{ target.distance }}</td>
              <td>{{ target.image_num }}</td>
              <td>
                <button class="delete-btn" @click="deleteTarget(target.id!)">Delete</button>
              </td>
            </tr>
          </tbody>
        </table>
      </section>
      
      <!-- Games Section -->
      <section class="config-section">
        <h2>Game Configurations</h2>
        <button class="add-btn" @click="showAddGameDialog = true">Add Game</button>
        <table class="config-table">
          <thead>
            <tr>
              <th>Game Name</th>
              <th>Duration (s)</th>
              <th>Targets</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="game in games" :key="game.id">
              <td>{{ game.name }}</td>
              <td>{{ game.total_time }}</td>
              <td>{{ game.targets.length }}</td>
              <td>
                <button class="delete-btn" @click="deleteGame(game.id!)">Delete</button>
              </td>
            </tr>
          </tbody>
        </table>
      </section>
    </div>
    
    <!-- Add Target Dialog -->
    <div v-if="showAddTargetDialog" class="dialog-overlay" @click="showAddTargetDialog = false">
      <div class="dialog" @click.stop>
        <h3>Add New Target</h3>
        <form @submit.prevent="addTarget">
          <label>
            Node ID:
            <input v-model.number="newTarget.node_id" type="number" min="1" required />
          </label>
          <label>
            Distance (m):
            <input v-model.number="newTarget.distance" type="number" min="1" step="0.1" required />
          </label>
          <label>
            Image Number:
            <select v-model.number="newTarget.image_num" required>
              <option v-for="i in 7" :key="i" :value="i + 1">{{ i + 1 }}</option>
            </select>
          </label>
          <div class="dialog-buttons">
            <button type="submit">Add</button>
            <button type="button" @click="showAddTargetDialog = false">Cancel</button>
          </div>
        </form>
      </div>
    </div>
    
    <!-- Add Game Dialog -->
    <div v-if="showAddGameDialog" class="dialog-overlay" @click="showAddGameDialog = false">
      <div class="dialog" @click.stop>
        <h3>Add New Game</h3>
        <form @submit.prevent="addGame">
          <label>
            Game Name:
            <input v-model="newGame.name" type="text" required />
          </label>
          <label>
            Duration (seconds):
            <input v-model.number="newGame.total_time" type="number" min="60" required />
          </label>
          <div class="dialog-buttons">
            <button type="submit">Add</button>
            <button type="button" @click="showAddGameDialog = false">Cancel</button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'

const router = useRouter()

interface Target {
  id?: number
  node_id: number
  distance: number
  image_num: number
}

interface Game {
  id?: number
  name: string
  total_time: number
  targets: any[]
}

const targets = ref<Target[]>([])
const games = ref<Game[]>([])
const showAddTargetDialog = ref(false)
const showAddGameDialog = ref(false)

const newTarget = ref({
  node_id: 1,
  distance: 10,
  image_num: 2
})

const newGame = ref({
  name: '',
  total_time: 60,
  targets: []
})

async function loadTargets() {
  try {
    targets.value = await invoke<Target[]>('get_all_targets')
  } catch (error) {
    console.error('Failed to load targets:', error)
  }
}

async function loadGames() {
  try {
    games.value = await invoke<Game[]>('get_all_games')
  } catch (error) {
    console.error('Failed to load games:', error)
  }
}

async function addTarget() {
  try {
    await invoke('create_target', {
      nodeId: newTarget.value.node_id,
      distance: newTarget.value.distance,
      imageNum: newTarget.value.image_num
    })
    showAddTargetDialog.value = false
    newTarget.value = { node_id: 1, distance: 10, image_num: 2 }
    await loadTargets()
  } catch (error) {
    console.error('Failed to add target:', error)
    alert('Failed to add target: ' + error)
  }
}

async function deleteTarget(id: number) {
  if (!confirm('Are you sure you want to delete this target?')) return
  try {
    await invoke('delete_target', { id })
    await loadTargets()
  } catch (error) {
    console.error('Failed to delete target:', error)
    alert('Failed to delete target: ' + error)
  }
}

async function addGame() {
  try {
    await invoke('create_game', {
      name: newGame.value.name,
      totalTime: newGame.value.total_time,
      targets: []
    })
    showAddGameDialog.value = false
    newGame.value = { name: '', total_time: 60, targets: [] }
    await loadGames()
  } catch (error) {
    console.error('Failed to add game:', error)
    alert('Failed to add game: ' + error)
  }
}

async function deleteGame(id: number) {
  if (!confirm('Are you sure you want to delete this game?')) return
  try {
    await invoke('delete_game', { id })
    await loadGames()
  } catch (error) {
    console.error('Failed to delete game:', error)
    alert('Failed to delete game: ' + error)
  }
}

function goBack() {
  router.push('/')
}

onMounted(() => {
  loadTargets()
  loadGames()
})
</script>

<style scoped>
.config-screen {
  width: 100vw;
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: #f5f5f5;
  overflow-y: auto;
}

.header {
  background: #667eea;
  color: white;
  padding: 1rem 2rem;
  display: flex;
  align-items: center;
  gap: 1rem;
}

.back-btn {
  background: rgba(255, 255, 255, 0.2);
  color: white;
  border: none;
  padding: 0.5rem 1rem;
  border-radius: 6px;
  cursor: pointer;
}

.back-btn:hover {
  background: rgba(255, 255, 255, 0.3);
}

.header h1 {
  font-size: 1.5rem;
}

.content {
  flex: 1;
  padding: 2rem;
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.config-section {
  background: white;
  border-radius: 8px;
  padding: 1.5rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.config-section h2 {
  margin-bottom: 1rem;
  color: #333;
}

.add-btn {
  background: #10b981;
  color: white;
  border: none;
  padding: 0.5rem 1rem;
  border-radius: 6px;
  cursor: pointer;
  margin-bottom: 1rem;
}

.add-btn:hover {
  background: #059669;
}

.config-table {
  width: 100%;
  border-collapse: collapse;
}

.config-table th {
  background: #f3f4f6;
  padding: 0.75rem;
  text-align: left;
  font-weight: bold;
  color: #333;
}

.config-table td {
  padding: 0.75rem;
  border-bottom: 1px solid #e5e7eb;
}

.delete-btn {
  background: #ef4444;
  color: white;
  border: none;
  padding: 0.25rem 0.75rem;
  border-radius: 4px;
  cursor: pointer;
}

.delete-btn:hover {
  background: #dc2626;
}

.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.dialog {
  background: white;
  border-radius: 8px;
  padding: 2rem;
  min-width: 400px;
  max-width: 90vw;
}

.dialog h3 {
  margin-bottom: 1rem;
  color: #333;
}

.dialog label {
  display: block;
  margin-bottom: 1rem;
  color: #333;
}

.dialog input,
.dialog select {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid #d1d5db;
  border-radius: 4px;
  margin-top: 0.25rem;
}

.dialog-buttons {
  display: flex;
  gap: 1rem;
  margin-top: 1.5rem;
}

.dialog-buttons button {
  flex: 1;
  padding: 0.75rem;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-weight: bold;
}

.dialog-buttons button[type="submit"] {
  background: #10b981;
  color: white;
}

.dialog-buttons button[type="button"] {
  background: #6b7280;
  color: white;
}
</style>

