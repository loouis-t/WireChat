<template>
  <div class="chat-list">
    <div
      v-for="conv in conversations"
      :key="conv.id"
      class="conversation-item"
      :class="{ active: conv.id === selectedConversationId }"
      :data-conversation-id="conv.id"
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
const props = defineProps({
  conversations: Array,
  selectedConversationId: [String, Number],
})
</script>

<style scoped>
.chat-list {
  border-radius: 8px;
  overflow-y: auto;
  width: 98%;
}

.conversation-item {
  border-bottom: 1px solid #2a2a2a;
  cursor: pointer;
  padding: 12px;
}

.conversation-item.active {
  background-color: #333333;
}

.conversation-item:hover {
  background-color: #2a2a2a;
}

.conversation-line {
  align-items: center;
  display: flex;
  justify-content: space-between;
  min-width: 0;
}

.conversation-name {
  color: #e0e0e0;
  flex: 1;
  font-weight: bold;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.last-time {
  color: #aaaaaa;
  flex-shrink: 0;
  font-size: 0.85em;
  margin-left: 10px;
  white-space: nowrap;
}

.last-message {
  color: #cccccc;
  font-size: 0.9em;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
