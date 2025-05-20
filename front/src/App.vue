<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import { ref, onMounted, onBeforeUnmount } from 'vue'

const isMobile = ref(false)
const showMobileMenu = ref(false)

function toggleMobileMenu() {
  showMobileMenu.value = !showMobileMenu.value
}

function checkMobile() {
  isMobile.value = window.innerWidth <= 768
}

onMounted(() => {
  checkMobile()
  window.addEventListener('resize', checkMobile)
})

onBeforeUnmount(() => {
  window.removeEventListener('resize', checkMobile)
})
</script>

<template>
  <div class="app-container">
    <!-- Sidebar / navigation fixe
    <aside class="sidebar" :class="{ mobileOpen: isMobile && showMobileMenu }">
      <header class="sidebar-header">
        <img alt="WG logo" class="logo" src="@/assets/logo.svg" />
        <button v-if="isMobile" class="close-menu" @click="toggleMobileMenu">×</button>
      </header>
      <nav class="sidebar-nav">
        <RouterLink to="/" @click="isMobile && toggleMobileMenu()">Chat</RouterLink>
        <RouterLink to="/settings" @click="isMobile && toggleMobileMenu()">Paramètres</RouterLink>
        <RouterLink to="/share" @click="isMobile && toggleMobileMenu()">Partager</RouterLink>
      </nav>
    </aside> -->

    <!-- Main content -->
    <div class="main-column">
      <header class="main-header">
        <button v-if="isMobile" class="burger-btn" @click="toggleMobileMenu">
          <img src="./assets/menu.png" alt="menu" class="burger-icon" />
        </button>
        <img alt="WG logo" class="logo-desktop" src="@/assets/logo.svg" />
        <nav class="main-nav" v-if="!isMobile">
          <RouterLink to="/">Chat</RouterLink>
          <RouterLink to="/settings">Paramètres</RouterLink>
          <RouterLink to="/share">Partager</RouterLink>
        </nav>
      </header>

      <!-- Vue de la route -->
      <main class="content-area">
        <RouterView />
      </main>
    </div>
  </div>
</template>

<style scoped>
/* Container principal en deux colonnes */
.app-container {
  display: flex;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
}

/* Bouton fermer */
.close-menu {
  background: none;
  border: none;
  font-size: 1.5rem;
  color: #fff;
  cursor: pointer;
}

/* Colonne principale */
.main-column {
  flex: 1;
  display: flex;
  flex-direction: column;
}

/* En-tête principal */
.main-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  background-color: var(--color-secondary);
  padding: 1rem;
  height: 80px;
}
.logo-desktop {
  display: none;
  height: 60px;
}
.main-nav {
  display: flex;
}

.main-nav a {
  position: relative;
  display: inline-block;  /* indispensable pour que ::after corresponde à la largeur du lien */
  margin-left: 3rem;
  color: #fff;
  text-decoration: none;
  padding-bottom: 4px;     /* espace sous le texte pour l’underline */
}

.main-nav a::after {
  content: "";
  position: absolute;
  left: 0;
  bottom: 0;
  height: 2px;
  width: 0;                /* commence à 0 */
  background-color: #8b0000;
  transition: width 0.2s ease;  /* animation de la largeur */
  border-radius: 25px;
}

.main-nav a:hover::after {
  width: 100%;             /* s’étend jusqu’à 100% */
}

/* Burger menu */
.burger-btn {
  background: none;
  border: none;
  cursor: pointer;
}
.burger-icon {
  width: 28px;
  height: 28px;
}

/* Zone de contenu */
.content-area {
  flex: 1;
  overflow-y: auto;
  background-color: #f5f5f5;
}

/* Affichage desktop */
@media (min-width: 769px) {
  .main-header .logo-desktop {
    display: block;
  }
  .burger-btn {
    display: none;
  }
}
</style>
