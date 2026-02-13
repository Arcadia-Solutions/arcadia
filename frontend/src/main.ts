import { createApp } from 'vue'
import App from './App.vue'
import PrimeVue from 'primevue/config'
import 'primeicons/primeicons.css'
import 'primeflex/primeflex.css'
import Aura from '@primeuix/themes/aura'
import { createPinia } from 'pinia'
import router from './router'
import 'primeicons/primeicons.css'
import './assets/main.css'
import Tooltip from 'primevue/tooltip'
import ToastService from 'primevue/toastservice'
import { definePreset } from '@primeuix/themes'
import { createI18n } from 'vue-i18n'
import en from './i18n/en.json'
import fr from './i18n/fr.json'

const app = createApp(App)

const CustomThemePreset = definePreset(Aura, {
  semantic: {
    primary: {
      200: '#BD3333',
      300: '#5fe1ab', //primary
      400: '#876ED0', //secondary
      500: '#BD3333',
      600: '#BD3333',
      700: '#BD3333',
      800: '#BD3333',
      900: '#BD3333',
    },
  },
})
const i18n = createI18n({
  locale: 'en',
  fallbackLocale: 'en', // Fallback if a translation is missing
  messages: {
    en,
    fr,
  },
})

app.use(createPinia())
app.use(PrimeVue, {
  theme: {
    preset: CustomThemePreset,
    options: {
      darkModeSelector: '.dark-theme',
    },
  },
})
app.use(router)
app.use(i18n)
app.use(ToastService)
app.directive('tooltip', Tooltip)

export function showToast(title: string, detail: string, severity: string, life?: number, closable: boolean = true, group: string = 'tr'): void {
  app.config.globalProperties.$toast.add({
    severity: severity,
    summary: title,
    detail: detail,
    life: life,
    closable: closable,
    group: group,
  })
}

export function removeToastGroup(group: string): void {
  app.config.globalProperties.$toast.removeGroup(group)
}

export { i18n }

app.mount('#app')
