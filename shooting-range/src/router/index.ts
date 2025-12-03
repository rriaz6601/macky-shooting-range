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

