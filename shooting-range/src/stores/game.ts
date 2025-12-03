import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface Game {
  id?: number
  name: string
  total_time: number
  targets: GameTarget[]
}

export interface GameTarget {
  id?: number
  target: {
    id?: number
    node_id: number
    distance: number
    image_num: number
  }
  start_time: number
  end_time: number
}

export const useGameStore = defineStore('game', () => {
  const isRunning = ref(false)
  const startTime = ref<number | null>(null)
  const currentTime = ref(0)
  const gameConfig = ref<Game | null>(null)
  const activeNodes = ref<Set<number>>(new Set())
  let timerInterval: number | null = null
  
  const remainingTime = computed(() => {
    if (!gameConfig.value || !startTime.value) return 0
    const elapsed = currentTime.value
    return Math.max(0, gameConfig.value.total_time - elapsed)
  })
  
  async function startGame(config: Game) {
    gameConfig.value = config
    isRunning.value = true
    startTime.value = Date.now()
    currentTime.value = 0
    activeNodes.value.clear()
    
    // Start timer
    timerInterval = window.setInterval(() => {
      if (!isRunning.value) {
        if (timerInterval) clearInterval(timerInterval)
        return
      }
      
      const elapsed = Math.floor((Date.now() - startTime.value!) / 1000)
      currentTime.value = elapsed
      
      // Update active nodes
      updateActiveNodes(elapsed)
      
      if (elapsed >= gameConfig.value!.total_time) {
        endGame()
        if (timerInterval) clearInterval(timerInterval)
      }
    }, 200)
  }
  
  async function updateActiveNodes(elapsed: number) {
    if (!gameConfig.value) return
    
    const newActiveNodes = new Set<number>()
    
    for (const gameTarget of gameConfig.value.targets) {
      if (elapsed >= gameTarget.start_time && elapsed < gameTarget.end_time) {
        newActiveNodes.add(gameTarget.target.node_id)
        
        // Send serial command if state changed
        if (!activeNodes.value.has(gameTarget.target.node_id)) {
          try {
            await invoke('send_target_command', {
              nodeId: gameTarget.target.node_id,
              state: true
            })
          } catch (error) {
            console.error('Failed to send command:', error)
          }
        }
      }
    }
    
    // Deactivate nodes that are no longer active
    for (const nodeId of activeNodes.value) {
      if (!newActiveNodes.has(nodeId)) {
        try {
          await invoke('send_target_command', {
            nodeId,
            state: false
          })
        } catch (error) {
          console.error('Failed to send command:', error)
        }
      }
    }
    
    activeNodes.value = newActiveNodes
  }
  
  async function endGame() {
    isRunning.value = false
    
    if (timerInterval) {
      clearInterval(timerInterval)
      timerInterval = null
    }
    
    // Deactivate all nodes
    for (const nodeId of activeNodes.value) {
      try {
        await invoke('send_target_command', {
          nodeId,
          state: false
        })
      } catch (error) {
        console.error('Failed to send command:', error)
      }
    }
    
    activeNodes.value.clear()
    gameConfig.value = null
    startTime.value = null
    currentTime.value = 0
  }
  
  function isNodeActive(nodeId: number): boolean {
    return activeNodes.value.has(nodeId)
  }
  
  function getTimeRemaining(nodeId: number): number {
    if (!gameConfig.value || !isNodeActive(nodeId)) return 0
    
    const elapsed = currentTime.value
    for (const gameTarget of gameConfig.value.targets) {
      if (gameTarget.target.node_id === nodeId) {
        if (elapsed >= gameTarget.start_time && elapsed < gameTarget.end_time) {
          return gameTarget.end_time - elapsed
        }
      }
    }
    return 0
  }
  
  return {
    isRunning,
    currentTime,
    remainingTime,
    activeNodes,
    gameConfig,
    startGame,
    endGame,
    isNodeActive,
    getTimeRemaining
  }
})

