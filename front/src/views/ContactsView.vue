<template>
  <div class="contacts-container">
    <div class="contacts-header">
      <h2 class="titrePage">Contacts</h2>
    </div>
    <div></div>
  </div>
</template>

<style scoped>
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
