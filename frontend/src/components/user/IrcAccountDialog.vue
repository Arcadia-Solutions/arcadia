<template>
  <div v-if="!ircPassword">
    <p>{{ userStore.irc_password_hash ? t('user_settings.reset_irc_confirm') : t('user_settings.create_irc_confirm') }}</p>
    <div class="wrapper-center" style="margin-top: 10px">
      <Button :label="t('general.confirm')" size="small" :loading="loading" @click="submit" />
    </div>
  </div>
  <div v-else>
    <Message severity="warn" :closable="false">{{ t('user_settings.irc_password_warning') }}</Message>
    <div style="margin-top: 10px">
      <p>
        <b>{{ t('user_settings.irc_username_label') }}:</b> {{ userStore.username }}
      </p>
      <p>
        <b>{{ t('user_settings.irc_password_label') }}:</b> <code>{{ ircPassword }}</code>
      </p>
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { Button, Message } from 'primevue'
import { createIRCAccount, resetIRCPassword } from '@/services/api-schema'
import { useUserStore } from '@/stores/user'
import { showToast } from '@/main'

const { t } = useI18n()
const userStore = useUserStore()

const ircPassword = ref<string>()
const loading = ref(false)

const submit = () => {
  loading.value = true
  const request = userStore.irc_password_hash ? resetIRCPassword() : createIRCAccount()
  request
    .then((res) => {
      ircPassword.value = res.irc_password
      userStore.irc_password_hash = 'set'
    })
    .catch(() => {
      showToast('', t('user_settings.irc_error'), 'error', 3000)
    })
    .finally(() => {
      loading.value = false
    })
}
</script>
