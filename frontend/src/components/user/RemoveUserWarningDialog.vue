<template>
  <div class="remove-warning-dialog">
    <p>{{ banned ? t('user.confirm_remove_ban') : t('user.confirm_remove_warning') }}</p>
    <Button :label="banned ? t('user.remove_ban') : t('user.remove_warning')" severity="danger" size="small" :loading @click="handleRemove" />
  </div>
</template>

<script setup lang="ts">
import { showToast } from '@/main'
import api from '@/services/api/api'
import Button from 'primevue/button'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps<{
  userId: number
  banned: boolean
}>()

const loading = ref(false)

const emit = defineEmits<{
  removed: []
}>()

const handleRemove = () => {
  loading.value = true
  api
    .delete(`/api/users/${props.userId}/warnings`)
    .then(() => {
      showToast('', props.banned ? t('user.ban_removed_success') : t('user.warning_removed_success'), 'success', 2000)
      emit('removed')
    })
    .finally(() => {
      loading.value = false
    })
}
</script>

<style scoped>
.remove-warning-dialog {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20px;
  padding-top: 10px;
}
</style>
