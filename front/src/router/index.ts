import { createRouter, createWebHistory } from 'vue-router'
import ChatView from '../views/ChatView.vue'
const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'chat',
      component: ChatView,
    },
    {
      path: '/settings',
      name: 'settings',
      component: () => import('../views/SettingsView.vue'),
    },
    {
      path: '/share',
      name: 'share',
      component: () => import('../views/ShareView.vue'),
    },
  ],
})

export default router
