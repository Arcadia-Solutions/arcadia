import { fileURLToPath, URL } from 'node:url'
import { existsSync, readFileSync } from 'node:fs'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import { parse } from 'yaml'
// import vueDevTools from 'vite-plugin-vue-devtools'

// The whole project is configured by a single config.yml at the root of the repository.
// Its `frontend` section is inlined in the bundle at build time, which means the frontend
// has to be rebuilt after changing it.
const configurationPath = fileURLToPath(new URL('../config.yml', import.meta.url))
const configuration = parse(readFileSync(configurationPath, 'utf8'))

if (!configuration?.frontend) {
  throw new Error(`no 'frontend' section found in ${configurationPath}`)
}

const logo = existsSync(new URL('./src/assets/logo.svg', import.meta.url)) ? 'logo.svg' : 'logo.example.svg'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueJsx(),
    // vueDevTools(),
  ],
  server: {
    proxy: {
      '/api': {
        target: 'http://localhost:8080',
        changeOrigin: true,
      },
    },
  },
  define: {
    __ARCADIA_CONFIG__: JSON.stringify(configuration.frontend),
  },
  resolve: {
    alias: {
      '@/assets/logo.svg': fileURLToPath(new URL(`./src/assets/${logo}`, import.meta.url)),
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
})
