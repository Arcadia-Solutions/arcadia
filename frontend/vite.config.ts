import { fileURLToPath, URL } from 'node:url'
import { existsSync, readFileSync } from 'node:fs'
import { resolve } from 'node:path'

import { defineConfig, type Plugin } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import { parse } from 'yaml'
import sirv from 'sirv'
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

const kiwiDevServer = (): Plugin => ({
  name: 'kiwiirc-dev-server',
  configureServer(server) {
    const kiwiDist = resolve(__dirname, '../kiwiirc/dist')
    if (!existsSync(kiwiDist)) {
      server.middlewares.use('/kiwiirc', (_req, res) => {
        res.setHeader('Content-Type', 'text/html; charset=utf-8')
        res.end(
          '<p style="font-family:system-ui,sans-serif;padding:40px;text-align:center;color:#888;background:#1a1a1a;">' +
            '<strong style="color:#eee;">KiwiIRC Dev Mode:</strong> <code>kiwiirc/dist</code> not found.<br>' +
            'Run <code>npm run kiwi:setup</code> to build & extract KiwiIRC locally.</p>',
        )
      })
      return
    }

    // Live reload workspace config and plugin during development
    const kiwiConfig = existsSync(resolve(__dirname, '../kiwiirc/config.json'))
      ? 'config.json'
      : 'config.json.example'

    server.middlewares.use('/kiwiirc', (req, res, next) => {
      if (req.url === '/static/config.json') {
        res.setHeader('Content-Type', 'application/json')
        return res.end(readFileSync(resolve(__dirname, `../kiwiirc/${kiwiConfig}`)))
      }
      if (req.url === '/static/plugins/arcadia-plugin.js') {
        res.setHeader('Content-Type', 'application/javascript')
        return res.end(readFileSync(resolve(__dirname, '../kiwiirc/arcadia-plugin.js')))
      }
      next()
    })

    server.middlewares.use('/kiwiirc', sirv(kiwiDist, { dev: true, single: true }))
  },
})

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueJsx(),
    kiwiDevServer(),
    // vueDevTools(),
  ],
  server: {
    proxy: {
      '/api': {
        target: 'http://localhost:8080',
        changeOrigin: true,
      },
      '/webirc/websocket': {
        target: 'ws://localhost:8097',
        ws: true,
        rewrite: (path) => path.replace(/^\/webirc\/websocket/, ''),
        configure: (proxy) => {
          proxy.on('error', (_err, _req, socket) => {
            // Silently close socket if Ergo is offline to prevent crashing Vite
            if (socket && 'destroyed' in socket && !socket.destroyed) {
              socket.destroy()
            }
          })
        },
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
