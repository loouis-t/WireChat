<template>
  <div class="profile-container">
    <div class="profile-header">
      <h2>Profil</h2>
    </div>
    <div class="blocPhotoProfil">
      <img id="photoProfil" src="../assets/profilePicture.png" />
      <div class="blocImportPhotoProfil input-group">
        <label for="configFile">Importer une photo de profil</label>
        <input id="configFile" type="file" @change="handleFileUpload" accept=".jpg,.jpeg,.png" />
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
        <textarea id="bio" v-model="profile.bio" placeholder="..."> </textarea>
      </div>
      <div class="form-group" style="margin-top: 40px">
        <button type="submit">Enregistrer</button>
      </div>
    </form>
  </div>
</template>

<script setup>
import { reactive } from 'vue'

// Objet réactif contenant les paramètres du profil
const profile = reactive({
  pseudo: '',
  bio: '',
})

// Fonction de sauvegarde du profil
function saveProfile() {
  console.log('Profil enregistré :', JSON.parse(JSON.stringify(profile)))
  alert('Votre profil a bien été enregistré.')
}
</script>

<style scoped>
#bio {
  background-color: #2b2b2b;
  border: none;
  border-radius: 4px;
  color: #f5f5f5;
  font-family: Arial, sans-serif;
  padding: 10px;
  height: 20vh;
  resize: none;
}

#configFile {
  margin: 8px 0px 0px 50px;
}

#photoProfil {
  height: 96px;
  margin-left: 40px;
}

.blocImportPhotoProfil {
  display: inline;
  margin-top: 20px;
  text-align: center;
  vertical-align: middle;
  width: 70%;
}

.blocPhotoProfil {
  display: flex;
  margin-bottom: 40px;
}

.profile-container {
  color: #eaeaea;
  background-color: #1e1e1e;
  border-radius: 8px;
  height: 75vh;
  padding: 20px;
  width: 35vw;
}

.profile-header {
  margin-bottom: 40px;
  text-align: center;
}

.profile-form .form-group {
  display: flex;
  margin-bottom: 15px;
  flex-direction: column;
}

.profile-form button {
  border: none;
  border-radius: 4px;
  background-color: #8b0000;
  color: #ffffff;
  cursor: pointer;
  font-size: 1rem;
  padding: 12px;
  transition: background-color 0.2s ease;
  width: 100%;
}

.profile-form button:hover {
  background-color: #5a0000;
}

.profile-form input {
  border-radius: 4px;
  border: none;
  background-color: #2b2b2b;
  color: #f5f5f5;
  font-size: 1rem;
  padding: 10px;
}

.profile-form input:focus {
  background-color: #3b3b3b;
  border: none;
  outline: none;
}

.profile-form input::placeholder {
  color: #aaaaaa;
}

.profile-form label {
  margin-bottom: 5px;
  font-size: 1rem;
  color: #cccccc;
}

@media (max-width: 768px) {
  #bio {
    width: 100%;
  }

  #configFile {
    width: 70%;
    text-align: left;
  }

  .blocImportPhotoProfil {
    width: 70%;
    display: block;
  }

  .profile-container {
    height: auto;
    padding: 10px;
    width: 90%;
  }

  .profile-form {
    width: 100%;
  }
}
</style>
