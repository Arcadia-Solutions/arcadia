<template>
  <div class="delete-dialog">
    <p>{{ t('series.confirm_delete_series') }}</p>
    <Button :label="t('general.delete')" severity="danger" size="small" :loading="deleting" @click="handleDelete" />
  </div>
</template>

<script setup lang="ts">
import { showToast } from '@/main'
import { deleteSeries } from '@/services/api-schema'
import Button from 'primevue/button'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps<{
  seriesId: number
}>()

const deleting = ref(false)

const emit = defineEmits<{
  deleted: []
}>()

const handleDelete = () => {
  deleting.value = true
  deleteSeries(props.seriesId)
    .then(() => {
      showToast('', t('series.series_deleted_success'), 'success', 2000)
      emit('deleted')
    })
    .finally(() => (deleting.value = false))
}
</script>

<style scoped>
.delete-dialog {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20px;
}
</style>
