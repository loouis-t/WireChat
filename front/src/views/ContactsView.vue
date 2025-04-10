<template>
  <div class="contacts-container">
    <div class="contacts-header">
      <h2>Contacts</h2>
    </div>
    <div>
      <table style="width: 632px">
        <!--
        <thead>
          <tr>
            <th scope="col">Photo de profil</th>
            <th scope="col">Nom d'utilisateur</th>
            <th scope="col">Actions (ouvrir le profil/envoyer un message)</th>
          </tr>
        </thead>
        -->
        <tbody>
          <tr>
            <th scope="row"><img class="photosProfil" src="../assets/skorfire.png" /></th>
            <td class="celluleTableau">SkorFire</td>
            <td class="celluleBoutonsOptions celluleTableau">
              <div>
                <RouterLink to="/settings">
                  <button class="boutonsOptions"><i class="fa-solid fa-user icones"></i></button
                ></RouterLink>
                <RouterLink to="/"
                  ><button class="boutonsOptions">
                    <i class="fa-solid fa-comments icones"></i></button
                ></RouterLink>
              </div>
            </td>
          </tr>
          <tr>
            <th scope="row"><img class="photosProfil" src="../assets/automatic_edited.png" /></th>
            <td class="celluleTableau">Deniz Koyu</td>
            <td class="celluleBoutonsOptions celluleTableau">
              <div>
                <RouterLink to="/settings">
                  <button class="boutonsOptions"><i class="fa-solid fa-user icones"></i></button
                ></RouterLink>
                <RouterLink to="/"
                  ><button class="boutonsOptions">
                    <i class="fa-solid fa-comments icones"></i></button
                ></RouterLink>
              </div>
            </td>
          </tr>
          <tr>
            <th scope="row"><img class="photosProfil" src="../assets/chopper_edited.png" /></th>
            <td class="celluleTableau">Tony Tony Chopper</td>
            <td class="celluleBoutonsOptions celluleTableau">
              <div>
                <RouterLink to="/settings">
                  <button class="boutonsOptions"><i class="fa-solid fa-user icones"></i></button
                ></RouterLink>
                <RouterLink to="/"
                  ><button class="boutonsOptions">
                    <i class="fa-solid fa-comments icones"></i></button
                ></RouterLink>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

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

.boutonsOptions {
  background-color: #7d2320;
  border-color: #7d2320;
  border-radius: 8px;
  border-style: none;
  color: #ffffff;
  height: 32px;
  margin: 2px;
  text-align: center;
  vertical-align: middle;
  width: 32px;
}

.boutonsOptions:hover {
  background-color: #691b1a;
  border-color: #691b1a;
  border-radius: 8px;
  border-style: none;
  color: #ffffffbb;
  cursor: pointer;
  height: 32px;
  margin: 2px;
  text-align: center;
  vertical-align: middle;
  width: 32px;
}

.celluleBoutonsOptions {
  text-align: right;
}

.celluleTableau {
  padding: 10px;
}

.contacts-container {
  color: #eaeaea;
  background-color: #1e1e1e;
  border-radius: 8px;
  height: 75vh;
  padding: 20px;
  width: 35vw;
}

.contacts-header {
  margin-bottom: 40px;
  text-align: center;
}

.contacts-form .form-group {
  display: flex;
  margin-bottom: 15px;
  flex-direction: column;
}

.contacts-form button {
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

.contacts-form button:hover {
  background-color: #5a0000;
}

.contacts-form input {
  border-radius: 4px;
  border: none;
  background-color: #2b2b2b;
  color: #f5f5f5;
  font-size: 1rem;
  padding: 10px;
}

.contacts-form input:focus {
  background-color: #3b3b3b;
  border: none;
  outline: none;
}

.contacts-form input::placeholder {
  color: #aaaaaa;
}

.contacts-form label {
  margin-bottom: 5px;
  font-size: 1rem;
  color: #cccccc;
}

.photosProfil {
  height: 48px;
}

a {
  background-color: unset;
  color: unset;
  padding: unset;
}

a:hover {
  background-color: unset;
  color: unset;
  padding: unset;
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

  .contacts-container {
    height: auto;
    padding: 10px;
    width: 90%;
  }

  .contacts-form {
    width: 100%;
  }
}
</style>
