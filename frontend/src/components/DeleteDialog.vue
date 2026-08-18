<template>
  <div class="delete-dialog">
    <p>{{ message }}</p>
    <Button :label="label ?? t('general.delete')" severity="danger" size="small" :loading @click="handleDelete" />
  </div>
</template>

<script setup lang="ts">
import { showToast } from '@/main'
import Button from 'primevue/button'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps<{
  // text shown above the confirmation button
  message: string
  // the deletion request itself
  action: () => Promise<unknown>
  // toast shown when the deletion succeeds, none is shown if not given
  successMessage?: string
  // label of the confirmation button, defaults to "delete"
  label?: string
}>()

const loading = ref(false)

const emit = defineEmits<{
  deleted: []
}>()

const handleDelete = () => {
  loading.value = true
  props
    .action()
    .then(() => {
      if (props.successMessage) {
        showToast('', props.successMessage, 'success', 2000)
      }
      emit('deleted')
    })
    .finally(() => (loading.value = false))
}
</script>

<style scoped>
.delete-dialog {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20px;
  padding-top: 10px;
}
</style>
