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
        <div
          v-for="(marker, idx) in distanceMarkers"
          :key="idx"
          class="marker-item"
        >
          <label>Distance</label>
          <span class="marker-distance">{{ marker.distance }} meter</span>
          <div class="marker-icon">
            <img
              :src="getTargetImage(2)"
              alt="Target"
              class="target-icon-small"
            />
          </div>
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
            :key="node.id || node.node_id"
            :node="node"
            :active="gameStore.isNodeActive(node.node_id)"
            :time-remaining="gameStore.getTimeRemaining(node.node_id)"
          />
        </div>
      </div>
      
      <!-- Right: Controls -->
      <div class="controls-panel">
        <div class="menu-section">
          <button class="menu-btn" @click="endGame">Menu</button>
        </div>
        
        <div class="timer-section">
          <div class="timer-label">TIMER</div>
          <div class="timer-display">
            {{ formatTime(gameStore.remainingTime) }}
          </div>
        </div>
        
        <div class="duration-section">
          <label>Program duration:</label>
          <div class="duration-inputs">
            <input
              v-model.number="duration.hours"
              type="number"
              min="0"
              placeholder="Hrs"
              :disabled="gameStore.isRunning"
            />
            <input
              v-model.number="duration.minutes"
              type="number"
              min="0"
              max="59"
              placeholder="Mins"
              :disabled="gameStore.isRunning"
            />
            <input
              v-model.number="duration.seconds"
              type="number"
              min="0"
              max="59"
              placeholder="Secs"
              :disabled="gameStore.isRunning"
            />
          </div>
        </div>
        
        <button
          class="start-btn"
          :disabled="gameStore.isRunning"
          @click="startGame"
        >
          {{ gameStore.isRunning ? 'Running...' : 'Start' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { useGameStore } from '../stores/game'
import NodeTarget from '../components/NodeTarget.vue'

const router = useRouter()
const gameStore = useGameStore()

interface Target {
  id?: number
  node_id: number
  distance: number
  image_num: number
}

interface DistanceMarker {
  id?: number
  marker_number: number
  distance: number
}

const nodes = ref<Target[]>([])
const distanceMarkers = ref<DistanceMarker[]>([])
const currentMarker = ref(4)

const duration = ref({
  hours: 0,
  minutes: 0,
  seconds: 0
})

async function loadTargets() {
  try {
    nodes.value = await invoke<Target[]>('get_all_targets')
    // Ensure we have 20 nodes (pad if needed)
    while (nodes.value.length < 20) {
      nodes.value.push({
        node_id: nodes.value.length + 1,
        distance: 10,
        image_num: 2
      })
    }
    nodes.value = nodes.value.slice(0, 20)
  } catch (error) {
    console.error('Failed to load targets:', error)
  }
}

async function loadDistanceMarkers() {
  try {
    distanceMarkers.value = await invoke<DistanceMarker[]>('get_distance_markers')
    // If no markers, create default ones
    if (distanceMarkers.value.length === 0) {
      for (let i = 0; i < 4; i++) {
        await invoke('upsert_distance_marker', {
          markerNumber: i + 1,
          distance: (i + 1) * 10
        })
      }
      distanceMarkers.value = await invoke<DistanceMarker[]>('get_distance_markers')
    }
  } catch (error) {
    console.error('Failed to load distance markers:', error)
  }
}

function getTargetImage(imageNum: number): string {
  return new URL(`../assets/ShootingTarget_graphics${imageNum}.png`, import.meta.url).href
}

function formatTime(seconds: number): string {
  if (seconds <= 0) return '0:00'
  const hrs = Math.floor(seconds / 3600)
  const mins = Math.floor((seconds % 3600) / 60)
  const secs = seconds % 60
  
  if (hrs > 0) {
    return `${hrs}:${String(mins).padStart(2, '0')}:${String(secs).padStart(2, '0')}`
  }
  return `${mins}:${String(secs).padStart(2, '0')}`
}

async function startGame() {
  const totalSeconds = duration.value.hours * 3600 +
                      duration.value.minutes * 60 +
                      duration.value.seconds
  
  if (totalSeconds <= 0) {
    alert('Please set a valid duration')
    return
  }
  
  // Create a temporary game config
  const gameConfig = {
    name: 'Quick Game',
    total_time: totalSeconds,
    targets: nodes.value.map(node => ({
      target: node,
      start_time: 0,
      end_time: totalSeconds
    }))
  }
  
  await gameStore.startGame(gameConfig)
}

async function endGame() {
  await gameStore.endGame()
  router.push('/')
}

onMounted(() => {
  loadTargets()
  loadDistanceMarkers()
})

onUnmounted(() => {
  if (gameStore.isRunning) {
    gameStore.endGame()
  }
})
</script>

<style scoped>
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
  background: linear-gradient(
    to bottom,
    #87ceeb 0%,
    #87ceeb 30%,
    #f0e68c 30%,
    #f0e68c 50%,
    #90ee90 50%,
    #90ee90 100%
  );
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

.distance-markers-panel {
  background: rgba(55, 65, 81, 0.9);
  border-radius: 12px;
  padding: 1rem;
  color: white;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.marker-header {
  text-align: center;
  padding: 0.5rem;
  background: rgba(0, 0, 0, 0.3);
  border-radius: 6px;
}

.marker-number {
  display: block;
  font-size: 2rem;
  font-weight: bold;
  margin-top: 0.5rem;
}

.marker-item {
  background: rgba(0, 0, 0, 0.2);
  border-radius: 8px;
  padding: 1rem;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.marker-item label {
  font-size: 0.75rem;
  color: rgba(255, 255, 255, 0.8);
}

.marker-distance {
  font-weight: bold;
  font-size: 1rem;
}

.marker-icon {
  width: 60px;
  height: 60px;
  margin: 0.5rem auto 0;
}

.target-icon-small {
  width: 100%;
  height: 100%;
  object-fit: contain;
}

.node-grid-container {
  display: flex;
  flex-direction: column;
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(10px);
  border-radius: 12px;
  padding: 1rem;
}

.targets-header {
  text-align: center;
  margin-bottom: 1rem;
  font-weight: bold;
  color: #333;
  font-size: 1.1rem;
}

.node-count {
  display: block;
  font-size: 1.5rem;
  margin-top: 0.5rem;
}

.node-grid {
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  grid-template-rows: repeat(4, 1fr);
  gap: 15px;
  flex: 1;
  padding: 1rem;
}

.controls-panel {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  background: rgba(255, 255, 255, 0.95);
  border-radius: 12px;
  padding: 1.5rem;
}

.menu-section {
  display: flex;
  justify-content: flex-end;
}

.menu-btn {
  background: #3b82f6;
  color: white;
  border: none;
  padding: 0.5rem 1rem;
  border-radius: 6px;
  cursor: pointer;
}

.menu-btn:hover {
  background: #2563eb;
}

.timer-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
}

.timer-label {
  font-size: 0.9rem;
  font-weight: bold;
  color: #333;
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
  font-family: 'Courier New', monospace;
  border: 4px solid #333;
}

.duration-section {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.duration-section label {
  font-weight: bold;
  color: #333;
}

.duration-inputs {
  display: flex;
  gap: 0.5rem;
}

.duration-inputs input {
  flex: 1;
  padding: 0.5rem;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  text-align: center;
}

.duration-inputs input:disabled {
  background: #f3f4f6;
  cursor: not-allowed;
}

.start-btn {
  width: 120px;
  height: 120px;
  border-radius: 50%;
  background: #ef4444;
  color: white;
  border: none;
  font-size: 1.5rem;
  font-weight: bold;
  cursor: pointer;
  transition: transform 0.2s, box-shadow 0.2s;
  align-self: center;
  margin-top: auto;
}

.start-btn:hover:not(:disabled) {
  transform: scale(1.05);
  box-shadow: 0 4px 12px rgba(239, 68, 68, 0.4);
}

.start-btn:active:not(:disabled) {
  transform: scale(0.95);
}

.start-btn:disabled {
  background: #9ca3af;
  cursor: not-allowed;
}
</style>

