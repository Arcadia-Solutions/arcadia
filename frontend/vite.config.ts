import { fileURLToPath, URL } from 'node:url'
import { readFileSync } from 'node:fs'

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

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueJsx(),
    // vueDevTools(),
  ],
  define: {
    __ARCADIA_CONFIG__: JSON.stringify(configuration.frontend),
  },
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
})
