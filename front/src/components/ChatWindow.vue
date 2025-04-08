<!-- ChatWindow.vue -->
<template>
  <div class="chat-window">
    <!-- En-tête de la conversation avec bouton "Nouvelle conversation" -->
    <div class="chat-header">
      <span>{{ conversation.name }}</span>
      <button class="new-conversation-btn" @click="$emit('newConversation')">
        Nouvelle conversation
      </button>
    </div>

    <!-- Zone des messages -->
    <div ref="messagesContainer" class="messages">
      <MessageBubble v-for="msg in conversation.messages" :key="msg.id" :message="msg" />
    </div>

    <!-- Champ de saisie pour envoyer un nouveau message -->
    <ChatInput @sendMessage="handleSendMessage" />
  </div>
</template>

<script setup>
import { onUpdated, ref } from 'vue'
import MessageBubble from './MessageBubble.vue'
import ChatInput from './ChatInput.vue'

const props = defineProps({
  conversation: Object, // La conversation sélectionnée (avec { id, name, messages[] })
})
const emit = defineEmits(['sendMessage', 'newConversation'])

// Référence à la div des messages pour gérer le défilement automatique
const messagesContainer = ref(null)

// Émettre l'événement d'envoi de message vers le parent, avec l'ID conv et le texte
function handleSendMessage(messageText) {
  if (props.conversation) {
    emit('sendMessage', { conversationId: props.conversation.id, text: messageText })
  }
}

// Scroll automatique vers le bas quand de nouveaux messages sont ajoutés
onUpdated(() => {
  if (messagesContainer.value) {
    messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight
  }
})
</script>

<style scoped>
.chat-window {
  display: flex;
  flex-direction: column;
  height: 100%;
  width: 100%;
  background-color: #181818;
}

.chat-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  background-color: #202020;
  border-bottom: 1px solid #2a2a2a;
  font-weight: bold;
  color: #e0e0e0;
  border-top-right-radius: 8px;
}

.new-conversation-btn {
  background-color: transparent;
  border: none;
  color: #a52121;
  cursor: pointer;
  font-size: 0.9rem;
}

.new-conversation-btn:hover {
  text-decoration: underline;
}

.messages {
  flex: 1;
  padding: 16px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
}
</style>
