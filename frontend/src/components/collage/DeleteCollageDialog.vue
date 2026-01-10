<template>
  <div class="delete-dialog">
    <p>{{ t('collage.confirm_delete_collage') }}</p>
    <Button :label="t('general.delete')" severity="danger" size="small" :loading="deleting" @click="handleDelete" />
  </div>
</template>

<script setup lang="ts">
import { showToast } from '@/main'
import { deleteCollage } from '@/services/api-schema'
import Button from 'primevue/button'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps<{
  collageId: number
}>()

const deleting = ref(false)

const emit = defineEmits<{
  deleted: []
}>()

const handleDelete = async () => {
  deleting.value = true
  deleteCollage(props.collageId).then(() => {
    showToast('', t('collage.collage_deleted_success'), 'success', 2000)
    emit('deleted')
  })
  deleting.value = false
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
