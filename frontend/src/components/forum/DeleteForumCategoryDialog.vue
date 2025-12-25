<template>
  <div class="delete-dialog">
    <p>{{ t('forum.confirm_delete_category') }}</p>
    <Button :label="t('general.delete')" severity="danger" size="small" :loading="deleting" @click="deleteCategory" />
  </div>
</template>

<script setup lang="ts">
import { showToast } from '@/main'
import { deleteForumCategory } from '@/services/api-schema'
import Button from 'primevue/button'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps<{
  categoryId: number
}>()

const deleting = ref(false)

const emit = defineEmits<{
  deleted: []
}>()

const deleteCategory = async () => {
  deleting.value = true
  await deleteForumCategory(props.categoryId)
  showToast('', t('forum.category_deleted_success'), 'success', 2000)
  deleting.value = false
  emit('deleted')
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
