<template>
  <div class="password-reset-link">
    <p>{{ t('user.password_reset_link_explanation') }}</p>
    <template v-if="generatedToken">
      <div class="link">
        <InputText :modelValue="generatedToken.reset_url" size="small" readonly fluid />
        <Button :label="t('general.copy')" size="small" icon="pi pi-copy" @click="copyResetUrl" />
      </div>
      <p>{{ t('user.password_reset_link_expires', [timeAgo(generatedToken.expires_at)]) }}</p>
    </template>
    <template v-else>
      <p>{{ t('user.password_reset_link_revokes_previous') }}</p>
      <div class="wrapper-center">
        <Button :label="t('user.generate_password_reset_link')" size="small" :loading @click="generateResetLink" />
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { showToast } from '@/main'
import { createPasswordResetToken, type GeneratedPasswordResetToken } from '@/services/api-schema'
import { timeAgo } from '@/services/helpers'
import { Button, InputText } from 'primevue'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps<{
  userId: number
}>()

const generatedToken = ref<GeneratedPasswordResetToken>()
const loading = ref(false)

const copyResetUrl = () => {
  if (generatedToken.value) {
    navigator.clipboard.writeText(generatedToken.value.reset_url).then(() => {
      showToast('', t('user.password_reset_link_copied'), 'success', 2000)
    })
  }
}

// the link is only generated once the staff member confirms, as generating it gives
// its bearer the ability to take over the account until it expires
const generateResetLink = () => {
  loading.value = true
  createPasswordResetToken(props.userId)
    .then((token) => {
      generatedToken.value = token
    })
    .finally(() => {
      loading.value = false
    })
}
</script>

<style scoped>
.password-reset-link {
  width: 50vw;
}
.link {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 10px;
}
</style>
