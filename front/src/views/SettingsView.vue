<!-- SettingsView.vue -->
<template>
  <div class="settings-container" style="min-height: 700px">
    <div class="settings-header" style="margin-bottom: 40px">
      <h2>Paramètres</h2>
    </div>
    <form @submit.prevent="saveSettings" class="settings-form">
      <div class="form-group">
        <label for="ipv6">IPv6</label>
        <input id="ipv6" type="text" v-model="settings.ipv6" placeholder="Ex : fe80::1" />
      </div>
      <div class="form-group">
        <label for="port">Port</label>
        <input id="port" type="number" v-model.number="settings.port" placeholder="Ex : 51820" />
      </div>
      <div class="form-group">
        <label for="privateKey">Clé Privée</label>
        <input
          id="privateKey"
          type="text"
          v-model="settings.privateKey"
          placeholder="Votre clé privée"
        />
      </div>
      <div class="form-group">
        <label for="publicKey">Clé Publique</label>
        <input
          id="publicKey"
          type="text"
          v-model="settings.publicKey"
          placeholder="Votre clé publique"
        />
      </div>
      <div class="form-group">
        <label for="username">Nom / Prénom ou Pseudo</label>
        <input
          id="username"
          type="text"
          v-model="settings.username"
          placeholder="Votre nom ou pseudo"
        />
      </div>
      <div class="form-group">
        <label for="email">Email</label>
        <input id="email" type="email" v-model="settings.email" placeholder="Votre email" />
      </div>
      <div class="form-group" style="margin-top: 40px">
        <button type="submit">Enregistrer</button>
      </div>
    </form>
  </div>
</template>

<script setup>
import { reactive } from 'vue'

// Objet réactif contenant les paramètres WireGuard
const settings = reactive({
  ipv6: '',
  port: 51820,
  privateKey: '',
  publicKey: '',
  username: '',
  email: '',
})

// Fonction de sauvegarde des paramètres
function saveSettings() {
  // Ici, vous pouvez appeler une API pour sauvegarder les paramètres
  console.log('Paramètres enregistrés :', JSON.parse(JSON.stringify(settings)))
  alert('Paramètres enregistrés !')
}
</script>

<style scoped>
.settings-container {
  width: 35vw;
  height: 75vh;
  background-color: #1e1e1e;
  border-radius: 8px;
  padding: 20px;
  color: #eaeaea;
  font-family: Arial, sans-serif;
}

.settings-header {
  text-align: center;
  margin-bottom: 20px;
}

.settings-form .form-group {
  margin-bottom: 15px;
  display: flex;
  flex-direction: column;
}

.settings-form label {
  margin-bottom: 5px;
  font-size: 1rem;
  color: #ccc;
}

.settings-form input {
  padding: 10px;
  border-radius: 4px;
  border: none;
  background-color: #2b2b2b;
  color: #f5f5f5;
  font-size: 1rem;
}

.settings-form input:focus {
  border: none;
  outline: none;
  background-color: #3b3b3b;
}

.settings-form input::placeholder {
  color: #aaa;
}

.settings-form button {
  width: 100%;
  padding: 12px;
  border: none;
  border-radius: 4px;
  background-color: #8b0000;
  color: #fff;
  font-size: 1rem;
  cursor: pointer;
  transition: background-color 0.2s ease;
}

.settings-form button:hover {
  background-color: #5a0000;
}

@media (max-width: 768px) {
  .settings-container {
    width: 90vw;
    height: auto;
    padding: 10px;
  }

  .settings-form {
    width: 100%;
  }
}
</style>
