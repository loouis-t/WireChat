<!-- ChatView.vue -->
<template>
  <div class="app-container">
    <!-- Liste des conversations -->
    <ChatList
      :conversations="conversations"
      :selectedConversationId="selectedConversationId"
      @selectConversation="selectConversation"
    />

    <!-- Partie droite : ChatWindow ou NewConversation selon showNewConversation -->
    <div class="right-pane">
      <!-- Formulaire "Nouvelle conversation" -->
      <NewConversation
        v-if="showNewConversation"
        @createConversation="createConversation"
        @cancel="closeNewConversation"
      />

      <!-- Fenêtre de chat (si une conversation est sélectionnée et qu'on n'est pas en train de créer) -->
      <ChatWindow
        v-else-if="activeConversation"
        :conversation="activeConversation"
        @sendMessage="sendMessage"
        @newConversation="openNewConversation"
      />

      <!-- Affichage par défaut si aucune conversation n'est sélectionnée et qu'on ne crée pas de conversation -->
      <div v-else class="no-conversation">Sélectionnez une conversation</div>
    </div>
  </div>
</template>

<script setup>
import { reactive, ref, computed } from 'vue'
import ChatList from '../components/ChatList.vue'
import ChatWindow from '../components/ChatWindow.vue'
import NewConversation from '../components/NewConversation.vue'

// Exemple de données
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
  // ...autres conversations
])

const selectedConversationId = ref(conversations[0].id)
const activeConversation = computed(() =>
  conversations.find((conv) => conv.id === selectedConversationId.value),
)

// Gère l'affichage du formulaire "Nouvelle conversation"
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

// Événement déclenché depuis ChatWindow (bouton "Nouvelle conversation")
function openNewConversation() {
  showNewConversation.value = true
}

// Création d'une nouvelle conversation
function createConversation(contactInfo) {
  // Ajoute la nouvelle conversation à la liste
  const newConv = {
    id: Date.now(),
    name: contactInfo,
    messages: [],
    lastMessage: '',
    lastTime: '',
  }
  conversations.push(newConv)

  // Sélectionne la nouvelle conversation
  selectedConversationId.value = newConv.id

  // Masque le formulaire
  showNewConversation.value = false
}

// Annule la création
function closeNewConversation() {
  showNewConversation.value = false
}
</script>

<style scoped>
.app-container {
  display: flex;
  height: 90vh;
  width: 55vw;
  background-color: #181818;
  color: #eaeaea;
  font-family: Arial, sans-serif;
}

/* Colonne de droite pour le chat ou le formulaire */
.right-pane {
  flex: 1; /* occupe tout l'espace restant */
  display: flex;
  flex-direction: column;
}

/* Style si aucune conversation n'est sélectionnée */
.no-conversation {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #999;
  font-style: italic;
}
</style>
