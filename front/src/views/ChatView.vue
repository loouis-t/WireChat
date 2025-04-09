<template>
  <div class="app-container">
    <div class="left-pane">
      <ChatList
        :conversations="conversations"
        :selectedConversationId="selectedConversationId"
        @selectConversation="selectConversation"
      />
    </div>

    <div class="right-pane">
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
import { reactive, ref, computed } from 'vue'
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

const selectedConversationId = ref(conversations[0].id)
const activeConversation = computed(() =>
  conversations.find((conv) => conv.id === selectedConversationId.value),
)

const showNewConversation = ref(false)

function selectConversation(id) {
  selectedConversationId.value = id
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
}

function closeNewConversation() {
  showNewConversation.value = false
}
</script>

<style scoped>
.app-container {
  display: flex;
  height: 75vh;
  width: 70vw;
  background-color: #181818;
  color: #eaeaea;
  font-family: Arial, sans-serif;
  border-radius: 8px;
}

.right-pane {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 8px;
  background-color: #202020;
  border-top-right-radius: 8px;
  border-bottom-right-radius: 8px;
}

.no-conversation {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #999;
  font-style: italic;
}

@media (max-width: 768px) {
  .app-container {
    width: 100%;
    height: 80%;
    flex-direction: column;
  }

  .right-pane {
    width: 100%;
    height: 100%;
    border-radius: 0;
  }

  .left-pane {
    display: none;
  }
}
</style>
