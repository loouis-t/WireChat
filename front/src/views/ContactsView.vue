<template>
  <div class="contacts-container">
    <div class="contacts-header">
      <h2 class="titrePage">Contacts</h2>
    </div>
    <div>
      <table class="table">
        <tbody>
          <tr>
            <th class="cellulePhotoProfil" scope="row">
              <img class="photosProfil" src="../assets/skorfire.png" />
            </th>
            <td>SkorFire</td>
            <td class="celluleBoutonsOptions">
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
            <th class="cellulePhotoProfil" scope="row">
              <img class="photosProfil" src="../assets/automatic_edited.png" />
            </th>
            <td>Deniz Koyu</td>
            <td class="celluleBoutonsOptions">
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
            <th class="cellulePhotoProfil" scope="row">
              <img class="photosProfil" src="../assets/chopper_edited.png" />
            </th>
            <td>Tony Tony Chopper</td>
            <td class="celluleBoutonsOptions">
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
.boutonsOptions {
  background-color: #88171a;
  border-color: #88171a;
  border-radius: 8px;
  border-style: none;
  color: #ffffff;
  height: 38px;
  text-align: center;
  vertical-align: middle;
  width: 38px;
  margin: 0px 2px;
  transition: background-color 0.2s ease;
}

.boutonsOptions:hover {
  background-color: #a52a2a;
  border-color: #a52a2a;
  cursor: pointer;
}

.celluleBoutonsOptions {
  text-align: right;
}

.cellulePhotoProfil {
  text-align: left;
}

.contacts-container {
  color: #eaeaea;
  background-color: #1e1e1e;
  border-radius: 8px;
  height: 75vh;
  padding: 20px;
  width: 35vw;
  overflow-y: scroll;
}

.contacts-header {
  text-align: center;
}

.photosProfil {
  height: 48px;
}

.table {
  width: 100%;
  height: 100%;
}

.titrePage {
  margin-bottom: 20px;
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
  .contacts-container {
    width: 95%;
    height: 75vh;
    padding: 20px;
  }

  .contacts-form {
    width: 100%;
  }
}
</style>
