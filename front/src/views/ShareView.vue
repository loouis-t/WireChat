<script setup>
import { ref, computed, onMounted } from 'vue'
import QrcodeVue from 'qrcode.vue'

const settings = ref({
  ipv6: '',
  port: 51820,
  privateKey: '',
  publicKey: '',
  pseudo: '',
})

onMounted(() => {
  const stored = localStorage.getItem('wireguard-settings')
  if (stored) {
    settings.value = JSON.parse(stored)
  }
})

const configQR = computed(() => {
  if (
    !settings.value.publicKey ||
    !settings.value.ipv6 ||
    !settings.value.port ||
    !settings.value.pseudo
  ) {
    return ''
  }
  return `[Peer]
# Pseudo: ${settings.value.pseudo}
PublicKey = ${settings.value.publicKey}
Endpoint = ${settings.value.ipv6}:${settings.value.port}
AllowedIPs = 0.0.0.0/0`
})

const configQRValid = computed(() => configQR.value !== '')
</script>

<template>
  <div class="share-view">
    <h2>Partager votre configuration WireGuard</h2>
    <p>
      Ce QR code contient vos informations publiques nécessaires à l'établissement d'une connexion.
      Faites-le scanner par votre interlocuteur.
    </p>
    <div class="qr-code-container">
      <qrcode-vue :value="configQR" :size="250" />
    </div>
    <div class="config-display" v-if="configQRValid">
      <pre>{{ configQR }}</pre>
    </div>
    <p v-else class="warning">
      Veuillez compléter vos paramètres WireGuard (y compris votre pseudo) dans la section
      Paramètres.
    </p>
  </div>
</template>

<style scoped>
.share-view {
  padding: 20px;
  width: 40%;
  height: 75vh;
  background-color: #1e1e1e;
  color: #eaeaea;
  border-radius: 8px;
  text-align: center;
  display: flex;
  flex-direction: column;
}
.share-view h2 {
  margin-bottom: 10px;
}
.share-view p {
  margin-bottom: 20px;
}
.qr-code-container {
  margin: auto;
  width: 250px;
  height: 250px;
}
.config-display {
  margin-top: 20px;
  text-align: left;
  background-color: #2b2b2b;
  padding: 10px;
  border-radius: 4px;
}
pre {
  margin: 0;
  font-family: monospace;
  font-size: 0.9rem;
  color: #f5f5f5;
}
.warning {
  color: #ffa500;
  font-style: italic;
}

@media (max-width: 768px) {
  .share-view {
    width: 90%;
    height: 75vh;
    padding: 10px;
  }
  .qr-code-container {
    width: 100%;
  }
}
</style>
