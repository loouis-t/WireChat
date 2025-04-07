<!-- MessageBubble.vue -->
<template>
  <div class="message-bubble" :class="{ mine: message.self, theirs: !message.self }">
    {{ message.text }}
  </div>
</template>

<script setup>
import { defineProps } from 'vue'

const props = defineProps({
  message: Object, // { text: String, self: Boolean, ... }
})
</script>

<style scoped>
.message-bubble {
  max-width: 60%;
  padding: 8px 12px;
  margin: 4px 0;
  border-radius: 18px;
  color: #eaeaea;
  /* Par défaut, prendra l'alignement du conteneur flex */
}
.message-bubble.mine {
  align-self: flex-end; /* aligne à droite pour l'utilisateur courant */
  background-color: #7d2320; /* couleur d'accent (bleu) pour messages envoyés */
  color: #ffffff;
}
.message-bubble.theirs {
  align-self: flex-start; /* aligne à gauche pour les messages reçus */
  background-color: #2f2f2f; /* gris foncé pour messages reçus */
  color: #e0e0e0;
}
</style>
