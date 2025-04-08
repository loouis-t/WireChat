import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueDevTools(),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    },
  },
  build: {
    // Tell Vite (and esbuild) to target modern JavaScript environments.
    target: 'esnext'
  },
  optimizeDeps: {
    esbuildOptions: {
      // Ensure dependencies are also built with a modern target.
      target: 'esnext'
    }
  }
})
