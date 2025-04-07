<!-- ChatList.vue -->
<template>
  <div class="chat-list">
    <div
      v-for="conv in conversations"
      :key="conv.id"
      class="conversation-item"
      :class="{ active: conv.id === selectedConversationId }"
      @click="$emit('selectConversation', conv.id)"
    >
      <div class="conversation-line">
        <span class="conversation-name">{{ conv.name }}</span>
        <span class="last-time">{{ conv.lastTime }}</span>
      </div>
      <div class="last-message">{{ conv.lastMessage }}</div>
    </div>
  </div>
</template>

<script setup>
import { defineProps } from 'vue'

const props = defineProps({
  conversations: Array, // Liste des conversations à afficher
  selectedConversationId: [String, Number], // ID de la conversation active
})
// Ce composant émettra un événement 'selectConversation' (pas de code supplémentaire requis grâce à $emit dans le template).
</script>

<style scoped>
.chat-list {
  width: 500px; /* largeur de la liste */
  background-color: #212121; /* fond sombre pour la liste */
  overflow-y: auto;
  border-right: 1px solid #2f2f2f; /* ligne de séparation */
  border-top-left-radius: 8px;
  border-bottom-left-radius: 8px;
}
.conversation-item {
  padding: 12px;
  cursor: pointer;
  border-bottom: 1px solid #2a2a2a;
}
.conversation-item:hover {
  background-color: #2a2a2a;
}
.conversation-item.active {
  background-color: #333333; /* fond légèrement plus clair pour la conv active */
}
.conversation-line {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.conversation-name {
  font-weight: bold;
  color: #e0e0e0;
}
.last-time {
  font-size: 0.85em;
  color: #aaa;
}
.last-message {
  font-size: 0.9em;
  color: #cccccc;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis; /* tronquer si trop long */
}
</style>
