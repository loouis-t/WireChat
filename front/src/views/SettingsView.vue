<script setup>
import { reactive } from 'vue'

const settings = reactive({
  ipv6: '',
  port: 51820,
  privateKey: '',
  publicKey: '',
})

const profile = reactive({
  pseudo: '',
  bio: '',
})

function saveSettings() {
  console.log('Paramètres enregistrés :', JSON.parse(JSON.stringify(settings)))
  // Ici, vous pouvez intégrer un appel à une API pour persister ces données.
}

function saveProfile() {
  console.log('Profil enregistré :', JSON.parse(JSON.stringify(profile)))
  // Ici, vous pouvez intégrer un appel à une API pour persister ces données.
}

function handleFileUpload(event) {
  const file = event.target.files[0]
  if (file) {
    console.log('Photo téléchargée :', file)
  }
}
</script>

<template>
  <div class="profile-container">
    <div class="profile-header">
      <h2>Profil</h2>
    </div>
    <div class="blocPhotoProfil">
      <img class="photoProfil" src="../assets/profilePicture.png" />
      <div class="blocImportPhotoProfil input-group">
        <label for="configFile">Importer une photo de profil</label>
        <input type="file" @change="handleFileUpload" accept=".jpg,.jpeg,.png" />
      </div>
    </div>
    <form @submit.prevent="saveProfile" class="profile-form">
      <div class="form-group">
        <label for="pseudo">Pseudo</label>
        <input
          id="pseudo"
          type="text"
          v-model="profile.pseudo"
          placeholder="Le nom avec lequel vous apparaîtrez dans l'application"
        />
      </div>
      <div class="form-group">
        <label for="bio">Bio</label>
        <textarea
          class="bio"
          id="bio"
          v-model="profile.bio"
          placeholder="Votre bio ici..."
        ></textarea>
      </div>
      <div class="page-container">
        <div class="settings-container">
          <div class="settings-header">
            <h2>Paramètres WireGuard</h2>
          </div>
          <form @submit.prevent="saveSettings" class="settings-form">
            <div class="form-group">
              <label class="ipv6" for="ipv6">IPv6</label>
              <input id="ipv6" type="text" v-model="settings.ipv6" placeholder="Ex : fe80::1" />
            </div>
            <div class="form-group">
              <label for="port">Port</label>
              <input
                id="port"
                type="number"
                v-model.number="settings.port"
                placeholder="Ex : 51820"
              />
            </div>
            <div class="form-group">
              <label for="privateKey">Clé privée</label>
              <input
                id="privateKey"
                type="text"
                v-model="settings.privateKey"
                placeholder="Votre clé privée"
              />
            </div>
            <div class="form-group">
              <label for="publicKey">Clé publique</label>
              <input
                id="publicKey"
                type="text"
                v-model="settings.publicKey"
                placeholder="Votre clé publique"
              />
            </div>
            <div class="form-group">
              <button class="boutonSubmit" type="submit">Enregistrer les paramètres</button>
            </div>
          </form>
        </div>
      </div>
    </form>
  </div>
</template>

<style>
.bio {
  background-color: #2b2b2b;
  border: none;
  border-radius: 4px;
  color: #f5f5f5;
  font-family: Arial, sans-serif;
  padding: 10px;
  height: 8vh;
  resize: none;
}

.blocImportPhotoProfil {
  display: grid;
  padding-top: 5px;
  text-align: left;
  width: 70%;
}

.blocPhotoProfil {
  display: flex;
  margin-bottom: 10px;
}

.boutonSubmit {
  margin-top: 40px;
}

.ipv6 {
  margin-top: 0px !important;
}

.page-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.photoProfil {
  height: 56px;
  margin-right: 20px;
}

.profile-container {
  color: #eaeaea;
  background-color: #1e1e1e;
  border-radius: 8px;
  height: 75vh;
  padding: 20px;
  width: 35vw;
}

.profile-form .form-group {
  display: flex;
  flex-direction: column;
}

.profile-form button {
  border: none;
  border-radius: 4px;
  background-color: #8b0000;
  color: #ffffff;
  cursor: pointer;
  font-size: 1rem;
  padding: 10px;
  transition: background-color 0.2s ease;
  width: 100%;
  margin-top: 15px;
}

.profile-form button:hover {
  background-color: #5a0000;
}

.profile-form input,
.profile-form textarea {
  background-color: #2b2b2b;
  border: none;
  border-radius: 4px;
  color: #f5f5f5;
  font-size: 1rem;
  padding: 10px;
}

.profile-form input:focus,
.profile-form textarea:focus {
  background-color: #3b3b3b;
  border: none;
  outline: none;
}

.profile-form input::placeholder,
.profile-form textarea::placeholder {
  color: #aaaaaa;
}

.profile-form label {
  color: #cccccc;
  font-size: 1rem;
  margin-top: 10px;
}

.profile-header {
  margin-bottom: 10px;
  text-align: center;
}

.settings-container {
  background-color: #1e1e1e;
  border-radius: 8px;
  color: #eaeaea;
  padding: 20px;
  width: 35vw;
}

.settings-form button {
  width: 100%;
  border: none;
  border-radius: 4px;
  background-color: #8b0000;
  color: #ffffff;
  font-size: 1rem;
  cursor: pointer;
  transition: background-color 0.2s ease;
}

.settings-form button:hover {
  background-color: #5a0000;
}

.settings-form .form-group {
  display: flex;
  flex-direction: column;
}

.settings-form input {
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
  color: #aaaaaa;
}

.settings-form label {
  font-size: 1rem;
  color: #cccccc;
}

.settings-header {
  text-align: center;
}

/* Responsivité */
@media (max-width: 768px) {
  .bio {
    width: 100%;
    height: 15vh;
  }

  .blocImportPhotoProfil {
    width: 70%;
    display: block;
  }

  .profile-container,
  .settings-container {
    width: 90vw;
  }

  .profile-form {
    width: 100%;
  }
}
</style>
