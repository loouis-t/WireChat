<template>
  <div class="new-conversation">
    <div class="header">
      <h2>Nouvelle conversation</h2>
    </div>
    <form @submit.prevent="submitForm" class="form">
      <div class="first-container">
        <div class="input-group">
          <label for="configFile">Importer une configuration WireGuard</label>
          <input type="file" @change="handleFileUpload" accept=".conf,.txt" />
        </div>

        <div class="or-separator">OU</div>

        <div class="qr-code-section">
          <button type="button" class="qr-scan-btn" @click="showScanner = true">
            Scanner un QR code
          </button>
        </div>

        <div v-if="showScanner" class="scanner-container">
          <qrcode-stream
            @decode="onDecode"
            @init="onInit"
            @error="onError"
            style="width: 100%; height: 100%"
          />
          <button type="button" @click="closeScanner" class="close-scanner-btn">
            Fermer le scanner
          </button>
        </div>
      </div>
      <div class="buttons">
        <button type="submit" class="create-btn">Créer</button>
        <button type="button" class="cancel-btn" @click="cancel">Annuler</button>
      </div>
    </form>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { QrcodeStream } from 'vue-qrcode-reader'

const configData = ref(null)
const qrResult = ref(null)
const showScanner = ref(false)

const emit = defineEmits(['createConversation', 'cancel'])

function submitForm() {
  if (configData.value && configData.value.publicKey) {
    emit('createConversation', configData.value.publicKey)
    configData.value = null
  } else if (qrResult.value && qrResult.value.publicKey) {
    emit('createConversation', qrResult.value.publicKey)
    qrResult.value = null
  } else {
    alert('Veuillez importer une configuration contenant une clé publique ou scanner un QR code.')
  }
}

function cancel() {
  emit('cancel')
}

function handleFileUpload(event) {
  const file = event.target.files[0]
  if (file) {
    const reader = new FileReader()
    reader.onload = (e) => {
      const content = e.target.result
      configData.value = parseConfig(content)
    }
    reader.readAsText(file)
  }
}

function parseConfig(content) {
  const lines = content.split('\n')
  let publicKey = ''
  for (const line of lines) {
    const trimmedLine = line.trim()
    if (!trimmedLine || trimmedLine.startsWith('#') || trimmedLine.startsWith(';')) continue
    if (trimmedLine.startsWith('PublicKey')) {
      const parts = trimmedLine.split('=')
      if (parts.length >= 2) {
        publicKey = parts[1].trim()
        break
      }
    }
  }
  return { publicKey }
}

function onDecode(decodedString) {
  qrResult.value = parseQrData(decodedString)
  showScanner.value = false
}

function onInit(promise) {
  promise.catch((error) => {
    console.error("Erreur d'initialisation du scanner:", error)
    alert("Erreur lors de l'initialisation du scanner QR : " + error.message)
  })
}

function onError(error) {
  console.error('Erreur pendant le scan du QR Code:', error)
  alert('Erreur lors du scan du QR code : ' + error.message)
}

function parseQrData(data) {
  return parseConfig(data)
}

function closeScanner() {
  showScanner.value = false
}
</script>

<style scoped>
.buttons {
  display: flex;
  justify-content: space-between;
  margin-top: 30px;
}

.cancel-btn,
.create-btn {
  background-color: #88171a;
  border: none;
  border-radius: 8px;
  color: #ffffff;
  cursor: pointer;
  font-size: 1rem;
  margin-top: 8%;
  padding: 10px 20px;
  transition: background-color 0.2s ease;
}

.cancel-btn:hover,
.create-btn:hover {
  background-color: #5a0000;
}

.close-scanner-btn {
  border: none;
  border-radius: 8px;
  cursor: pointer;
  margin-top: 8px;
  padding: 6px 12px;
}

.form {
  width: 70%;
  height: 90%;
  display: flex;
  padding: 20px;
  flex-direction: column;
  justify-content: space-between;
}

.form input[type='file'] {
  border-radius: 8px;
  color: #f5f5f5;
  font-size: 1rem;
  margin-top: 2%;
}

.form label {
  margin-right: 50%;
}

.header {
  text-align: center;
  margin-bottom: 30px;
}

.input-group {
  margin-bottom: 16px;
  text-align: left;
}

.new-conversation {
  width: 100%;
  height: 100%;
  background-color: #1e1e1e;
  padding: 20px;
  color: #e0e0e0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

.or-separator {
  text-align: center;
  font-size: 0.9rem;
  margin-bottom: 16px;
  color: #aaaaaa;
}

.qr-code-section {
  display: flex;
  margin-bottom: 16px;
}

.qr-scan-btn {
  padding: 10px 20px;
  border: none;
  border-radius: 8px;
  font-size: 1rem;
  cursor: pointer;
  transition: background-color 0.2s ease;
  background-color: #88171a;
  color: #ffffff;
}

.qr-scan-btn:hover {
  background-color: #5a0000;
}

.scanner-container {
  width: 550px;
  height: 380px;
  margin: 8px auto;
  position: relative;
}

@media screen and (max-width: 768px) {
  .scanner-container {
    width: 100%;
    height: 100%;
    padding: 0;
  }

  .form {
    width: 100%;
    height: 100%;
    padding: 10px;
  }

  .scanner-container {
    width: 320px;
    height: 340px;
  }
}
</style>
