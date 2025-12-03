<template>
  <div
    class="node-target"
    :class="{ active: active, inactive: !active }"
  >
    <div class="target-icon">
      <img
        :src="getTargetImage(node.image_num)"
        :alt="`Node ${node.node_id}`"
        class="target-image"
      />
    </div>
    <div class="node-label">Node:{{ node.node_id }}</div>
    <div v-if="active" class="time-remaining">{{ formatTime(timeRemaining) }}</div>
    <div v-else class="status">OFF</div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  node: {
    id?: number
    node_id: number
    distance: number
    image_num: number
  }
  active: boolean
  timeRemaining: number
}

defineProps<Props>()

function getTargetImage(imageNum: number): string {
  return new URL(`../assets/ShootingTarget_graphics${imageNum}.png`, import.meta.url).href
}

function formatTime(seconds: number): string {
  if (seconds <= 0) return '0s'
  if (seconds < 60) return `${seconds}s`
  const mins = Math.floor(seconds / 60)
  const secs = seconds % 60
  return `${mins}m ${secs}s`
}
</script>

<style scoped>
.node-target {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 0.5rem;
  border-radius: 50%;
  transition: all 0.3s ease;
  cursor: pointer;
  background: rgba(255, 255, 255, 0.9);
  border: 2px solid transparent;
  min-width: 80px;
  min-height: 80px;
}

.node-target.active {
  border: 3px solid #10b981;
  box-shadow: 0 0 20px rgba(16, 185, 129, 0.5);
  animation: pulse 2s infinite;
  background: rgba(16, 185, 129, 0.1);
}

.node-target.inactive {
  opacity: 0.7;
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
    transform: scale(1);
  }
  50% {
    opacity: 0.8;
    transform: scale(1.05);
  }
}

.target-icon {
  width: 50px;
  height: 50px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.target-image {
  width: 100%;
  height: 100%;
  object-fit: contain;
}

.node-label {
  font-size: 0.75rem;
  font-weight: bold;
  margin-top: 0.25rem;
  color: #333;
}

.time-remaining {
  font-size: 0.7rem;
  color: #10b981;
  font-weight: bold;
  margin-top: 0.25rem;
}

.status {
  font-size: 0.7rem;
  color: #6b7280;
  margin-top: 0.25rem;
}
</style>

