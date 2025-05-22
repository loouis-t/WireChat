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
    <div v-if="isMobile && showMobileMenu" class="overlay" @click="toggleMobileMenu"></div>

    <aside class="sidebar" :class="{ open: isMobile && showMobileMenu }">
      <nav class="sidebar-nav">
        <RouterLink to="/" @click="toggleMobileMenu">Chat</RouterLink>
        <RouterLink to="/settings" @click="toggleMobileMenu">Paramètres</RouterLink>
        <RouterLink to="/share" @click="toggleMobileMenu">Partager</RouterLink>
      </nav>
    </aside>

    <div class="main-column">
      <header class="main-header">
        <button v-if="isMobile" class="burger-btn" @click="toggleMobileMenu" aria-label="Menu">
          <img v-if="!showMobileMenu" src="./assets/menu.png" alt="menu" class="burger-icon" />
          <span v-else class="close-icon">×</span>
        </button>
        <img alt="WG logo" class="logo-desktop" src="@/assets/logo.svg" />
        <nav class="main-nav" v-if="!isMobile">
          <RouterLink to="/">Chat</RouterLink>
          <RouterLink to="/settings">Paramètres</RouterLink>
          <RouterLink to="/share">Partager</RouterLink>
        </nav>
      </header>

      <main class="content-area">
        <RouterView />
      </main>
    </div>
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  height: 100%;
  width: 100%;
  overflow: hidden;
}

.overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0, 0, 0, 0.9);
  z-index: 29;
}

.sidebar {
  position: fixed;
  top: 50px;
  left: 7px;
  width: 300px;
  height: 100%;
  background-color: var(--color-secondary);
  transform: translateX(-100%);
  transition: transform 0.2s ease;
  z-index: 30;
  padding: 1rem;
  box-sizing: border-box;
}
.sidebar.open {
  transform: translateX(0);
}
.sidebar-nav {
  display: flex;
  flex-direction: column;
}
.sidebar-nav a {
  color: #fff;
  padding: 0.75rem 0;
  text-decoration: none;
}
.sidebar-nav a.router-link-exact-active {
  font-weight: bold;
  text-decoration: underline;
  text-decoration-color: rgb(170, 30, 30);
  text-decoration-thickness: 3px;
  text-underline-offset: 4px;
  text-decoration-style: solid;
}

.main-column {
  flex: 1;
  display: flex;
  flex-direction: column;
}
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
  margin-left: 3rem;
  color: #fff;
  text-decoration: none;
  position: relative;
  padding-bottom: 4px;
}
.main-nav a::after {
  content: '';
  position: absolute;
  left: 0;
  bottom: 0;
  height: 2px;
  width: 0;
  background-color: var(--color-accent);
  transition: width 0.2s ease;
  border-radius: 25px;
}
.main-nav a:hover::after {
  width: 100%;
}

.burger-btn {
  background: none;
  border: none;
  cursor: pointer;
  z-index: 31;
}
.burger-icon {
  width: 28px;
  height: 28px;
  filter: invert(1);
}
.close-icon {
  font-size: 3rem;
  color: #fff;
  line-height: 1;
}

.content-area {
  flex: 1;
  overflow-y: auto;
  background-color: #f5f5f5;
}

@media (min-width: 769px) {
  .overlay,
  .sidebar {
    display: none;
  }
  .burger-btn {
    display: none;
  }
  .main-header .logo-desktop {
    display: block;
  }
}
</style>
