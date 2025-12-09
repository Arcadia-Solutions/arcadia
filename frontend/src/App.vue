<template>
  <div id="app-container" v-if="isAppReady">
    <Toast position="top-right" group="tr" />
    <div class="navbars-container" v-if="isRouteProtected(route.path)">
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
import { ref } from 'vue'
import { useRoute } from 'vue-router'
import FooterBar from './components/FooterBar.vue'
import { useNotificationsStore } from './stores/notifications'
import { isRouteProtected } from './services/helpers'
import { getMe } from './services/api-schema'

// enable dark mode by default
document.documentElement.classList.add('dark-theme')

const isAppReady = ref(false)
const route = useRoute()
const router = useRouter()
const siteName = import.meta.env.VITE_SITE_NAME

router.beforeEach(async (to, from, next) => {
  if (from.path === '/login' && isRouteProtected(to.path)) {
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

router.afterEach(async (to) => {
  if (!isRouteProtected(to.path)) {
    isAppReady.value = true
  }
})

const getAppReady = async (forceGetUser: boolean = false) => {
  const token = localStorage.getItem('token')

  if (isRouteProtected(route.path) || forceGetUser) {
    if (token) {
      try {
        // refresh user on page reload or fetch user after registration
        const profile = await getMe()
        localStorage.setItem('user', JSON.stringify(profile.user))
        const userStore = useUserStore()
        userStore.setUser(profile.user)
        useNotificationsStore().unread_conversations_amount = profile.unread_conversations_amount
        useNotificationsStore().unread_notifications_amount_forum_thread_posts = profile.unread_notifications_amount_forum_thread_posts
        // load custom css sheet
        const head = document.getElementsByTagName('head')[0]
        const style = document.createElement('link')
        style.href = `${import.meta.env.VITE_API_BASE_URL}/css/${profile.user.css_sheet_name}.css`
        style.type = 'text/css'
        style.rel = 'stylesheet'
        head.append(style)

        isAppReady.value = true
      } catch {
        // token is invalid, redirect to login
        localStorage.removeItem('token')
        localStorage.removeItem('user')
        router.push('/login')
      }
    } else {
      // no token is present
      if (import.meta.env.VITE_ENABLE_CUSTOM_FRONT_PAGE === 'true') {
        window.location.href = '/home/index.html'
      } else {
        router.push('/login')
      }
    }
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
