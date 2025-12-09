<template>
  <div class="warn-user">
    <FloatLabel>
      <Textarea class="reason" name="reason" v-model="warning.reason" rows="5" />
      <label for="reason">{{ t('general.reason') }}</label>
    </FloatLabel>
    <div class="ban">
      <Checkbox v-model="warning.ban" inputId="ban" name="ban" binary />
      <label for="ban"> {{ t('user.ban') }} </label>
    </div>
    <Button label="Send warning" size="small" :loading @click="sendWarning()" />
  </div>
</template>

<script setup lang="ts">
import { showToast } from '@/main'
import { warnUser, type UserCreatedUserWarning, type UserWarning } from '@/services/api-schema'
import { Textarea, FloatLabel, Checkbox } from 'primevue'
import Button from 'primevue/button'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute } from 'vue-router'

const { t } = useI18n()
const route = useRoute()

const warning = ref<UserCreatedUserWarning>({
  reason: '',
  ban: false,
  expires_at: null,
  user_id: parseInt(route.params.id as string),
})
const loading = ref(false)

const emit = defineEmits<{
  warned: [warning: UserWarning]
}>()

const sendWarning = () => {
  loading.value = true
  warnUser(warning.value).then((data: UserWarning) => {
    loading.value = false
    showToast('Success', t('user.user_warned_success'), 'success', 4000)
    emit('warned', data)
  })
}
</script>

<style scoped>
.warn-user {
  padding-top: 20px;
  width: 30em !important;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}
.reason {
  width: 25em;
  margin-bottom: 20px;
}
.ban {
  margin-bottom: 15px;
  display: flex;
  align-items: center;
  label {
    margin-left: 5px;
  }
}
</style>
