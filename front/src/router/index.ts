import { createRouter, createWebHistory } from 'vue-router'
import ChatList from '@/components/ChatList.vue'
import ChatView from '@/views/ChatView.vue'
import SettingsView from '@/views/SettingsView.vue'
import ShareView from '@/views/ShareView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'chat',
      component: ChatView,
    },
    {
      path: '/chatlist',
      name: 'chatlist',
      component: ChatList,
    },
    {
      path: '/settings',
      name: 'settings',
      component: SettingsView,
    },
    {
      path: '/share',
      name: 'share',
      component: ShareView,
    },
  ],
})

export default router
