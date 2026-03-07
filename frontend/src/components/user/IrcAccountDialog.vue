<template>
  <div v-if="!done">
    <p>{{ userStore.irc_password ? t('user_settings.reset_irc_confirm') : t('user_settings.create_irc_confirm') }}</p>
    <div class="wrapper-center" style="margin-top: 10px">
      <Button :label="t('general.confirm')" size="small" :loading="loading" @click="submit" />
    </div>
  </div>
  <div v-else>
    <p>{{ t('user_settings.irc_password_updated') }}</p>
  </div>
</template>
<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { Button } from 'primevue'
import { createIRCAccount, resetIRCPassword } from '@/services/api-schema'
import { useUserStore } from '@/stores/user'
import { showToast } from '@/main'

const { t } = useI18n()
const userStore = useUserStore()

const done = ref(false)
const loading = ref(false)

const submit = () => {
  loading.value = true
  const request = userStore.irc_password ? resetIRCPassword() : createIRCAccount()
  request
    .then((res) => {
      userStore.irc_password = res.irc_password
      done.value = true
    })
    .catch(() => {
      showToast('', t('user_settings.irc_error'), 'error', 3000)
    })
    .finally(() => {
      loading.value = false
    })
}
</script>
