<template>
  <div
    class="app-container"
    :class="{ mobile: isMobile }"
    @touchstart="handleTouchStart"
    @touchend="handleTouchEnd"
  >
    <!-- Liste des conversations -->
    <div v-if="!isMobile || (isMobile && !inConversationView)" class="left-pane">
      <h2 class="titrePage">Conversations</h2>
      <ChatList
        :conversations="conversations"
        :selectedConversationId="selectedConversationId"
        @selectConversation="handleSelectConversation"
      />
    </div>

    <!-- Conversation sélectionnée -->
    <div v-if="!isMobile || (isMobile && inConversationView)" class="right-pane">
      <button v-if="isMobile" class="back-button" @click="backToList">← Retour</button>

      <NewConversation
        v-if="showNewConversation"
        @createConversation="createConversation"
        @cancel="closeNewConversation"
      />

      <ChatWindow
        v-else-if="activeConversation"
        :conversation="activeConversation"
        @sendMessage="sendMessage"
        @newConversation="openNewConversation"
      />

      <div v-else class="no-conversation">Sélectionnez une conversation</div>
    </div>
  </div>
</template>

<script setup>
import { reactive, ref, computed, onMounted, onBeforeUnmount } from 'vue'
import ChatList from '../components/ChatList.vue'
import ChatWindow from '../components/ChatWindow.vue'
import NewConversation from '../components/NewConversation.vue'

const conversations = reactive([
  {
    id: 1,
    name: 'Alice',
    messages: [
      { text: 'Hello, ça fait longtemps !', self: false },
      { text: 'Salut Alice, oui en effet :)', self: true },
    ],
    lastMessage: 'Salut Alice, oui en effet :)',
    lastTime: '10:15',
  },
  {
    id: 2,
    name: 'Bob',
    messages: [
      { text: 'Hey, tu as vu le projet ?', self: false },
      { text: "Pas encore, je regarde ça aujourd'hui.", self: true },
    ],
    lastMessage: "Pas encore, je regarde ça aujourd'hui.",
    lastTime: 'Hier',
  },
])

const selectedConversationId = ref(null)
const activeConversation = computed(() =>
  conversations.find((conv) => conv.id === selectedConversationId.value),
)
const showNewConversation = ref(false)
const isMobile = ref(window.innerWidth <= 768)
const inConversationView = ref(false)

let touchStartX = 0
let touchEndX = 0
let touchedConversationId = null

function findConversationIdElement(el) {
  while (el && el !== document.body) {
    if (el.dataset?.conversationId) {
      return parseInt(el.dataset.conversationId, 10)
    }
    el = el.parentElement
  }
  return null
}

function handleTouchStart(e) {
  touchStartX = e.changedTouches[0].screenX
  const touchY = e.changedTouches[0].clientY
  const touchX = e.changedTouches[0].clientX
  const element = document.elementFromPoint(touchX, touchY)
  touchedConversationId = findConversationIdElement(element)
}

function handleTouchEnd(e) {
  touchEndX = e.changedTouches[0].screenX
  const deltaX = touchEndX - touchStartX
  if (Math.abs(deltaX) > 50 && isMobile.value) {
    if (deltaX < 0 && !inConversationView.value) {
      if (touchedConversationId != null) {
        selectedConversationId.value = touchedConversationId
      }
      inConversationView.value = true
    } else if (deltaX > 0 && inConversationView.value) {
      inConversationView.value = false
    }
  }
}

function handleResize() {
  isMobile.value = window.innerWidth <= 768
  if (!isMobile.value) inConversationView.value = false
}

function handleSelectConversation(id) {
  selectedConversationId.value = id
  if (isMobile.value) inConversationView.value = true
}

function backToList() {
  inConversationView.value = false
}

function sendMessage({ conversationId, text }) {
  const conv = conversations.find((c) => c.id === conversationId)
  if (conv) {
    conv.messages.push({ text: text, self: true })
    conv.lastMessage = text
    const now = new Date()
    conv.lastTime =
      now.getHours().toString().padStart(2, '0') +
      ':' +
      now.getMinutes().toString().padStart(2, '0')
  }
}

function openNewConversation() {
  showNewConversation.value = true
}

function createConversation(contactInfo) {
  const newConv = {
    id: Date.now(),
    name: contactInfo,
    messages: [],
    lastMessage: '',
    lastTime: '',
  }
  conversations.push(newConv)
  selectedConversationId.value = newConv.id
  showNewConversation.value = false
  if (isMobile.value) {
    inConversationView.value = true
  }
}

function closeNewConversation() {
  showNewConversation.value = false
}

onMounted(() => {
  window.addEventListener('resize', handleResize)
})

onBeforeUnmount(() => {
  window.removeEventListener('resize', handleResize)
})
</script>

<style scoped>
.app-container {
  background-color: #1e1e1e;
  border-radius: 8px;
  box-sizing: border-box;
  color: #eaeaea;
  display: flex;
  flex-grow: 1;
  margin: 20px auto 0 auto;
  max-width: 1200px;
  overflow: hidden;
  padding: 16px;
  width: 100%;
}

.app-container.mobile {
  flex-direction: column;
  height: auto;
  width: 100%;
}

.back-button {
  align-self: flex-start;
  background: none;
  border: none;
  color: #eaeaea;
  cursor: pointer;
  font-size: 1rem;
  margin-bottom: 10px;
}

.left-pane,
.right-pane {
  display: flex;
  flex: 1;
  flex-direction: column;
  padding: 8px;
}

.no-conversation {
  align-items: center;
  color: #999999;
  display: flex;
  flex: 1;
  font-style: italic;
  justify-content: center;
}

.titrePage {
  font-size: 1.2rem;
  font-weight: bold;
  margin-bottom: 20px;
  text-align: center;
}
</style>
