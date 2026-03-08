<template>
  <ContentContainer :containerTitle="t('irc.webchat')" :containerTitleLink style="margin-top: 10px">
    <template #top-right>
      <i
        :class="userStore.irc_site_embed_enabled ? 'pi pi-eye-slash' : 'pi pi-eye'"
        style="cursor: pointer"
        v-tooltip.top="userStore.irc_site_embed_enabled ? t('user_settings.hide_irc_on_homepage') : t('user_settings.show_irc_on_homepage')"
        @click="toggleIrcEmbed"
      />
    </template>
    <template v-if="userStore.irc_site_embed_enabled">
      <div v-if="loading" class="loading">Loading IRC...</div>
      <Message v-else-if="error" severity="error">{{ error }}</Message>
      <iframe v-else-if="iframeSrc" :src="iframeSrc" :name="iframeWindowName" class="kiwi-iframe" />
    </template>
  </ContentContainer>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import Message from 'primevue/message'
import ContentContainer from '@/components/ContentContainer.vue'
import { useUserStore } from '@/stores/user'
import { usePublicArcadiaSettingsStore } from '@/stores/publicArcadiaSettings'
import { createIRCAccount, getUserSettings, updateUserSettings } from '@/services/api-schema'

defineProps<{
  containerTitleLink?: string
}>()

const { t } = useI18n()
const userStore = useUserStore()
const publicSettings = usePublicArcadiaSettingsStore()

const iframeSrc = ref<string | null>(null)
const iframeWindowName = ref('')
const loading = ref(true)
const error = ref<string | null>(null)

const toggleIrcEmbed = () => {
  const newValue = !userStore.irc_site_embed_enabled
  getUserSettings().then((settings) => {
    settings.irc_site_embed_enabled = newValue
    updateUserSettings(settings).then(() => {
      userStore.irc_site_embed_enabled = newValue
    })
  })
}

const buildConnectionConfig = () => {
  const websocketUrl = new URL(import.meta.env.VITE_IRC_WEBSOCKET_URL || '/webirc/websocket', window.location.origin)
  const channels = publicSettings.irc_webchat_default_channels.join(',')
  const password = `${userStore.username}:${userStore.irc_password}`
  const tls = websocketUrl.protocol === 'https:' || websocketUrl.protocol === 'wss:'
  const defaultPort = tls ? '443' : '80'
  const port = websocketUrl.port || defaultPort
  const path = websocketUrl.pathname === '/' ? '' : websocketUrl.pathname
  return { server: websocketUrl.hostname, port: parseInt(port, 10), tls, path, nick: userStore.username, password, channel: channels }
}

const connect = () => {
  iframeWindowName.value = JSON.stringify(buildConnectionConfig())
  iframeSrc.value = import.meta.env.VITE_IRC_WEBCHAT_URL || '/kiwiirc/'
  loading.value = false
}

onMounted(() => {
  if (userStore.irc_password) {
    connect()
  } else {
    createIRCAccount()
      .then((res) => {
        userStore.irc_password = res.irc_password
        connect()
      })
      .catch(() => {
        error.value = t('irc.account_creation_failed')
      })
      .finally(() => {
        loading.value = false
      })
  }
})
</script>

<style scoped>
.kiwi-iframe {
  width: 100%;
  height: 500px;
  border: none;
}
.loading {
  text-align: center;
  padding: 20px;
}
</style>
