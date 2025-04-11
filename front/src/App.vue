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
  <div class="wrapper">
    <header>
      <img alt="WG logo" class="logo" src="@/assets/logo.svg" />
      <nav>
        <RouterLink to="/">Chat<i class="fa-solid fa-comments iconeFontAwesome"></i></RouterLink>
        <RouterLink to="/contacts"
          >Contacts<i class="fa-solid fa-address-book iconeFontAwesome"></i
        ></RouterLink>
        <RouterLink to="/settings"
          >Paramètres<img class="gear" src="./assets/gear.webp"
        /></RouterLink>
        <RouterLink to="/share">
          Partager<img class="shareIcon" src="./assets/partager.png" />
        </RouterLink>
      </nav>
      <button v-if="isMobile" class="burger-btn" @click="toggleMobileMenu">
        <img src="./assets/menu.png" alt="menu" class="burger-icon" />
      </button>
    </header>

    <div v-if="isMobile && showMobileMenu" class="mobile-menu">
      <button class="close-menu" @click="toggleMobileMenu">×</button>
      <RouterLink @click="toggleMobileMenu" to="/"
        >Chat<i class="fa-solid fa-comments" style="margin-left: 8px"></i
      ></RouterLink>
      <RouterLink @click="toggleMobileMenu" to="/contacts"
        >Contacts<i class="fa-solid fa-address-book" style="margin-left: 8px"></i
      ></RouterLink>
      <RouterLink @click="toggleMobileMenu" to="/chatlist"
        >Chatlist<i class="fa-solid fa-comments" style="margin-left: 8px"></i
      ></RouterLink>
      <RouterLink @click="toggleMobileMenu" to="/settings"
        >Paramètres<img class="phoneGear" src="./assets/gear.webp"
      /></RouterLink>
      <RouterLink @click="toggleMobileMenu" to="/share">
        Partager<img class="phoneShare" src="./assets/partager.png" />
      </RouterLink>
    </div>

    <RouterView />
  </div>
</template>

<style scoped>
.gear {
  height: 18px;
  margin-bottom: -4px;
  margin-left: 5px;
  filter: invert(1);
}

.iconeFontAwesome {
  margin-left: 8px;
}

.logo {
  display: block;
  height: 125px;
  margin: 0.5rem;
  width: 125px;
}

.mobile-menu a.router-link-exact-active .phoneGear {
  content: url('./assets/gear_edited.png');
  filter: invert(0);
}

.mobile-menu a.router-link-exact-active .phoneShare {
  content: url('./assets/partager_edited.png');
  filter: invert(0);
}

.phoneGear {
  height: 24px;
  margin-bottom: -4px;
  margin-left: 5px;
  filter: invert(1);
}

.phoneShare {
  height: 24px;
  margin-bottom: -4px;
  margin-left: 5px;
  filter: invert(1);
}

.shareIcon {
  height: 18px;
  margin-bottom: -4px;
  margin-left: 5px;
  filter: invert(1);
}

.wrapper {
  display: flex;
  flex-direction: column;
  justify-content: space-evenly;
  align-items: center;
  height: 100vh;
  width: 100%;
}

header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 1rem;
  color: #eaeaea;
  height: 8vh;
}

nav {
  display: flex;
  font-size: 1rem;
}

nav a {
  display: inline-block;
  padding: 0 1rem;
  border-left: 1px solid var(--color-border);
  color: #ffffff;
}

nav a:first-of-type {
  border: 0;
}

nav a.router-link-exact-active {
  color: #7d2320;
}

nav a.router-link-exact-active .gear {
  content: url('./assets/gear_edited.png');
  filter: invert(0);
}

nav a.router-link-exact-active .shareIcon {
  content: url('./assets/partager_edited.png');
  filter: invert(0);
}

nav a.router-link-exact-active:hover {
  background-color: transparent;
}

@media (min-width: 1024px) {
  .burger-btn {
    display: none;
    background: none;
    border: none;
    font-size: 1.5rem;
    color: #eaeaea;
  }

  .logo {
    margin: 0 2rem 0 0;
  }

  header {
    display: flex;
    place-items: center;
    justify-content: space-between;
    align-items: center;
  }

  header .wrapper {
    display: flex;
    flex-wrap: wrap;
  }

  nav {
    text-align: left;
    font-size: 1rem;
    padding: 1rem 0;
    margin-top: 1rem;
  }
}

@media (max-width: 768px) {
  .burger-btn {
    border: none;
    background-color: transparent;
    display: block;
    filter: invert(1);
  }

  .burger-icon {
    width: 30px;
    height: 30px;
    filter: invert(0);
  }

  .close-menu {
    position: absolute;
    top: 20px;
    right: 20px;
    font-size: 2rem;
    color: #fff;
    background-color: transparent;
    border: none;
    cursor: pointer;
  }

  .logo {
    width: 20%;
  }

  .mobile-menu {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.92);
    color: #ffffff;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }

  .mobile-menu a {
    margin-bottom: 1rem;
    font-size: 1.5rem;
    color: #ffffff;
  }

  .mobile-menu a:hover {
    text-decoration: underline;
  }
  .mobile-menu a.router-link-exact-active {
    color: #7d2320;
  }

  header {
    width: 100%;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    margin: 0;
  }

  nav {
    display: none;
    margin: 0;
  }
}
</style>
