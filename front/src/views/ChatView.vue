<template>
  <div
    class="app-container"
    :class="{ mobile: isMobile }"
    @touchstart="handleTouchStart"
    @touchend="handleTouchEnd"
  >
    <div v-if="!isMobile || (isMobile && !inConversationView)" class="left-pane">
      <h2 class="titrePage">Conversations</h2>
      <div class="contacts">
        <div class="contacts-container">
          <div class="cellulePhotoProfil" scope="row">
            <img class="photosProfil" src="../assets/skorfire.png" />
            <p>SkorFire</p>
          </div>
          <div class="cellulePhotoProfil" scope="row">
            <img class="photosProfil" src="../assets/automatic_edited.png" />
            <p>DenizKoyu</p>
          </div>
          <div class="cellulePhotoProfil" scope="row">
            <img class="photosProfil" src="../assets/chopper_edited.png" />
            <p>TonyChopper</p>
          </div>
          <div class="cellulePhotoProfil" scope="row">
            <img class="photosProfil" src="../assets/skorfire.png" />
            <p>SkorFire</p>
          </div>
          <div class="cellulePhotoProfil" scope="row">
            <img class="photosProfil" src="../assets/automatic_edited.png" />
            <p>DenizKoyu</p>
          </div>
          <div class="cellulePhotoProfil" scope="row">
            <img class="photosProfil" src="../assets/chopper_edited.png" />
            <p>TonyChopper</p>
          </div>
        </div>
        <div class="cellulePhotoProfil" scope="row">
          <button class="new-conversation-btn" @click="openNewConversation">
            <i class="boutonNouvelleConversation fa-solid fa-plus"></i>
          </button>
        </div>
      </div>
      <ChatList
        :conversations="conversations"
        :selectedConversationId="selectedConversationId"
        @selectConversation="handleSelectConversation"
      />
    </div>

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
import { reactive, ref, computed, onBeforeUnmount, onMounted } from 'vue'
import ChatList from '../components/ChatList.vue'
import ChatWindow from '../components/ChatWindow.vue'
import NewConversation from '../components/NewConversation.vue'

const conversations = reactive([])
const selectedConversationId = ref(null)
const activeConversation = computed(() =>
  conversations.find((c) => c.id === selectedConversationId.value),
)
const showNewConversation = ref(false)
const isMobile = ref(window.innerWidth <= 768)
const inConversationView = ref(false)

const myPublicKey = ref('')

let ws = null

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
  if (!text.trim() || !activeConversation.value) return
  const to = activeConversation.value.publicKey
  const payload = { to, content: text }

  if (ws && ws.readyState === WebSocket.OPEN) {
    ws.send(JSON.stringify(payload))
  }

  const conv = activeConversation.value
  conv.messages.push({ id: Date.now(), text, self: false })
  conv.lastMessage = text
  conv.lastTime = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
}

function openNewConversation() {
  showNewConversation.value = true
  if (isMobile.value) inConversationView.value = true
}

function closeNewConversation() {
  showNewConversation.value = false
  if (isMobile.value) inConversationView.value = false
}

function createConversation(contactInfo) {
  const newId = Date.now()
  conversations.push({
    id: newId,
    name: contactInfo,
    publicKey: contactInfo,
    messages: [],
    lastMessage: '',
    lastTime: '',
  })
  showNewConversation.value = false
  handleSelectConversation(newId)
}

onMounted(async () => {
  window.addEventListener('resize', handleResize)

  try {
    const res = await fetch('http://localhost:49369/get_peers')
    const { peers } = await res.json()

    const me = peers[0].public_key
    peers
      .filter((p) => p.public_key !== me)
      .forEach((p) => {
        conversations.push({
          id: p.public_key,
          name: p.name || p.public_key,
          publicKey: p.public_key,
          messages: [],
          lastMessage: '',
          lastTime: '',
        })
      })

    ws = new WebSocket(`ws://localhost:49369/?peer_key=${encodeURIComponent(me)}`)

    ws.onmessage = (evt) => {
      console.log('Message reçu:', evt)
      const msg = JSON.parse(evt.data)

      conversations.forEach((conv) => {
        if (msg.from === conv.id || msg.to === conv.id) {
          conv.messages.push({
            id: `${msg.timestamp}-${Math.random()}`,
            text: msg.content,
            self:
              msg.to === conv.id
                ? false
                : true,
          })
          conv.lastMessage = msg.content
          conv.lastTime = new Date(msg.timestamp).toLocaleTimeString([], {
            hour: '2-digit',
            minute: '2-digit',
          })
        }
      })
    }
  } catch (err) {
    console.error('Erreur fetch peers ou WS:', err)
  }
})

onBeforeUnmount(() => {
  window.removeEventListener('resize', handleResize)
  ws?.close()
})
</script>

<style scoped>
.app-container {
  background-color: #fff6ec;
  box-sizing: border-box;
  color: #eaeaea;
  display: flex;
  flex-direction: row;
  justify-content: space-evenly;
  height: 100%;
  width: 100%;
  overflow-x: hidden;
}

.app-container.mobile {
  flex-direction: column;
}

.back-button {
  position: absolute;
  top: 1.5rem;
  right: 1rem;
  background: rgba(255, 255, 255, 0.8);
  border: none;
  padding: 0.5rem 0.75rem;
  font-size: 1rem;
  color: #202020;
  cursor: pointer;
  border-radius: 4px;
  z-index: 10;
}

.left-pane {
  width: 30%;
  background-color: #202020;
  padding: 1rem;
  box-sizing: border-box;
}
.right-pane {
  flex: 1;
  display: flex;
  flex-direction: column;
  box-sizing: border-box;
}

@media (max-width: 768px) {
  .left-pane,
  .right-pane {
    width: 100vw;
    box-sizing: border-box;
    margin: 0;
  }
  .left-pane {
    overflow-x: hidden;
    height: 100%;
  }
}

.no-conversation {
  align-items: center;
  color: #999999;
  display: flex;
  font-style: italic;
  justify-content: center;
  height: 100%;
}

.titrePage {
  margin-bottom: 20px;
  text-align: center;
}

.new-conversation-btn {
  background-color: #ffffff;
  border: none;
  cursor: pointer;
  font-size: 0.9rem;
}

.new-conversation-btn:hover {
  text-decoration: underline;
}

.contacts {
  display: flex;
  flex-direction: row;
  justify-content: space-around;
  margin-bottom: 20px;
  max-width: 100%;
}

.contacts-container {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-evenly;
  max-width: 70%;
  overflow-y: scroll;
}

.cellulePhotoProfil {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  margin: 10px;
}
.photosProfil {
  height: 48px;
}
</style>
