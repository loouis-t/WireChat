<template>
  <div class="message-bubble" :class="{ theirs: message.self, mine: !message.self }">
    {{ message.text }}
  </div>
</template>

<script setup>
const props = defineProps({
  message: Object,
})
</script>

<style scoped>
.message-bubble {
  max-width: 70%;
  padding: 4px 12px;
  margin: 4px 0;
  border-radius: 18px;
  color: #eaeaea;
}

.message-bubble.mine {
  align-self: flex-end;
  background-color: #88171a;
  color: #ffffff;
}

.message-bubble.theirs {
  align-self: flex-start;
  background-color: #2f2f2f;
  color: #e0e0e0;
}
</style>
