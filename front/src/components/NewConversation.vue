<!-- NewConversation.vue -->
<template>
  <div class="new-conversation">
    <div class="header">
      <h2>Nouvelle conversation</h2>
    </div>
    <form @submit.prevent="submitForm" class="form">
      <label for="contactInfo">Nom ou numéro de téléphone</label>
      <input
        id="contactInfo"
        v-model="contactInfo"
        type="text"
        placeholder="Ex : Jean Dupont ou 0123456789"
      />
      <div class="buttons">
        <button type="submit" class="create-btn">Créer</button>
        <button type="button" class="cancel-btn" @click="cancel">Annuler</button>
      </div>
    </form>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const contactInfo = ref('')
const emit = defineEmits(['createConversation', 'cancel'])

function submitForm() {
  if (contactInfo.value.trim() !== '') {
    emit('createConversation', contactInfo.value.trim())
    contactInfo.value = ''
  }
}

function cancel() {
  emit('cancel')
}
</script>

<style scoped>
.new-conversation {
  max-width: 400px;
  height: 27%;
  margin: 20px auto;
  background-color: #1e1e1e;
  border-radius: 12px;
  padding: 20px;
  color: #e0e0e0;
}

.header {
  text-align: center;
  margin-bottom: 20px;
}

.form {
  display: flex;
  flex-direction: column;
}

.form label {
  margin-bottom: 8px;
  font-size: 1rem;
}

.form input {
  padding: 10px 14px;
  border-radius: 20px;
  border: 1px solid #8b0000;
  background-color: #2b2b2b;
  color: #f5f5f5;
  font-size: 1rem;
  margin-bottom: 16px;
}

.buttons {
  display: flex;
  justify-content: space-between;
}

.create-btn,
.cancel-btn {
  padding: 10px 20px;
  border: none;
  border-radius: 20px;
  font-size: 1rem;
  cursor: pointer;
  transition: background-color 0.2s ease;
}

.create-btn {
  background-color: #8b0000;
  color: #fff;
}

.create-btn:hover {
  background-color: #5a0000;
}

.cancel-btn {
  background-color: transparent;
  color: #0a84ff;
}

.cancel-btn:hover {
  text-decoration: underline;
}
</style>
