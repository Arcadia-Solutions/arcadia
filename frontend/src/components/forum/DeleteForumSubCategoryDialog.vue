<template>
  <div class="delete-dialog">
    <p>{{ t('forum.confirm_delete_subcategory') }}</p>
    <Button :label="t('general.delete')" severity="danger" size="small" :loading="deleting" @click="deleteSubCategory" />
  </div>
</template>

<script setup lang="ts">
import { showToast } from '@/main'
import { deleteForumSubCategory } from '@/services/api-schema'
import Button from 'primevue/button'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps<{
  subCategoryId: number
}>()

const deleting = ref(false)

const emit = defineEmits<{
  deleted: []
}>()

const deleteSubCategory = async () => {
  deleting.value = true
  await deleteForumSubCategory(props.subCategoryId)
  showToast('', t('forum.subcategory_deleted_success'), 'success', 2000)
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
