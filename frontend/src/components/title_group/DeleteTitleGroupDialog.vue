<template>
  <div class="delete-dialog">
    <p>{{ t('title_group.confirm_delete_title_group') }}</p>
    <Button :label="t('general.delete')" severity="danger" size="small" :loading="deleting" @click="handleDelete" />
  </div>
</template>

<script setup lang="ts">
import { showToast } from '@/main'
import { deleteTitleGroup } from '@/services/api-schema'
import Button from 'primevue/button'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps<{
  titleGroupId: number
}>()

const deleting = ref(false)

const emit = defineEmits<{
  deleted: []
}>()

const handleDelete = () => {
  deleting.value = true
  deleteTitleGroup(props.titleGroupId)
    .then(() => {
      showToast('', t('title_group.title_group_deleted_success'), 'success', 2000)
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
