<template>
  <div id="app-container" v-if="isAppReady">
    <Toast position="top-right" group="tr" />
    <div class="navbars-container" v-if="isProtectedRoute()">
      <TopBar />
      <MenuBar class="menu-bar" />
      <SearchBars class="search-bars" />
    </div>
    <div id="view-container">
      <router-view></router-view>
    </div>
    <NotificationToasts />
    <FooterBar />
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import MenuBar from './components/MenuBar.vue'
import TopBar from './components/TopBar.vue'
import SearchBars from './components/SearchBars.vue'
import NotificationToasts from './components/NotificationToasts.vue'
import { Toast } from 'primevue'
import { useUserStore } from './stores/user'
import { getMe, type Profile } from './services/api/userService'
import { ref } from 'vue'
import { useRoute } from 'vue-router'
import FooterBar from './components/FooterBar.vue'
import { useNotificationsStore } from './stores/notifications'

// enable dark mode by default
document.documentElement.classList.add('dark-theme')

const isAppReady = ref(false)
const route = useRoute()
const router = useRouter()
const siteName = import.meta.env.VITE_SITE_NAME

const isProtectedRoute = (path?: string) => {
  if (path === undefined) {
    path = route.path
  }
  return ['/login', '/register', '/apply'].indexOf(path) < 0
}

router.beforeEach(async (to, from, next) => {
  if (from.path === '/login' && isProtectedRoute(to.path)) {
    await getAppReady(true)
  }
  if (to.meta.dynamicDocumentTitle) {
    /*
      The View for this route handles it's own
      document title because it's dynamic.
    */
    return next()
  }

  /*
    Favour a custom document title if is defined, otherwise,
    fall back to use the route name at least.
  */
  document.title = `${to.meta.documentTitle || to.name} - ${siteName}`

  return next()
})

const getAppReady = async (forceGetUser: boolean = false) => {
  const token = localStorage.getItem('token')

  let profile: null | Profile = null
  if (isProtectedRoute() || forceGetUser) {
    if (token) {
      try {
        // refresh user on page reload or fetch user after registration
        profile = await getMe()
        localStorage.setItem('user', JSON.stringify(profile.user))
        const userStore = useUserStore()
        userStore.setUser(profile.user)
      } catch {
        // Token is invalid, redirect to login
        localStorage.removeItem('token')
        localStorage.removeItem('user')
        router.push('/login')
      }
    } else {
      router.push('/login')
    }
  }
  isAppReady.value = true
  // removeToastGroup('br')
  if (profile) {
    useNotificationsStore().unread_conversations_amount = profile.unread_conversations_amount
  }
}

router.isReady().then(async () => {
  getAppReady()
})
</script>

<style>
.navbars-container {
  width: 100%;
}
.menu-bar {
  margin-top: 10px;
}
.search-bars {
  margin-top: 10px;
  margin-bottom: 20px;
}
#footer {
  margin-top: auto;
  width: 100%;
}
</style>
