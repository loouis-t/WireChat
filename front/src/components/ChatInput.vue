<template>
  <form class="chat-input" @submit.prevent="send">
    <input v-model="messageText" type="text" placeholder="Écrire un message..." />
    <button type="submit">Envoyer</button>
  </form>
</template>

<script setup>
import { ref } from 'vue'

const emit = defineEmits(['sendMessage'])
const messageText = ref('')

function send() {
  const text = messageText.value.trim()
  if (text !== '') {
    emit('sendMessage', text)
  }
  messageText.value = ''
}
</script>

<style scoped>
.chat-input {
  display: flex;
  align-items: center;
  height: 8%;
  padding: 8px;
  background-color: #202020;
  border-top: 1px solid #2a2a2a;
  border-bottom-right-radius: 8px;
}

.chat-input input {
  flex: 1;
  padding: 8px 12px;
  margin-right: 8px;
  background-color: #2f2f2f;
  border: none;
  border-radius: 20px;
  color: #eaeaea;
}

.chat-input button {
  padding: 8px 16px;
  background-color: #88171a;
  border: none;
  border-radius: 18px;
  color: #fff;
  cursor: pointer;
  font-weight: bold;
}

.chat-input button:hover {
  background-color: #701916;
}
</style>
