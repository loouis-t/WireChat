<template>
  <div class="chat-window">
    <div class="chat-header">
      <span>{{ conversation.name }}</span>
    </div>

    <div ref="messagesContainer" class="messages">
      <MessageBubble v-for="msg in conversation.messages" :key="msg.id" :message="msg" />
    </div>

    <ChatInput @sendMessage="handleSendMessage" />
  </div>
</template>

<script setup>
import { onUpdated, ref, onMounted } from 'vue'
import MessageBubble from './MessageBubble.vue'
import ChatInput from './ChatInput.vue'

const props = defineProps({
  conversation: Object,
})
const emit = defineEmits(['sendMessage'])

const messagesContainer = ref(null)

function handleSendMessage(messageText) {
  if (props.conversation) {
    emit('sendMessage', { conversationId: props.conversation.id, text: messageText })
  }
}

onUpdated(() => {
  if (messagesContainer.value) {
    messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight
  }
})
</script>

<style scoped>
.boutonNouvelleConversation {
  color: #ffffff;
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
}

.chat-window {
  display: flex;
  flex-direction: column;
  height: 100%;
  width: 100%;
  background-color: #181818;
}

.messages {
  flex: 1;
  padding: 16px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
}
</style>
