<template>
  <div class="delete-dialog">
    <p>{{ t('forum.confirm_delete_post') }}</p>
    <Button :label="t('general.delete')" severity="danger" size="small" :loading="deleting" @click="deletePost" />
  </div>
</template>

<script setup lang="ts">
import { showToast } from '@/main'
import { deleteForumPost } from '@/services/api-schema'
import Button from 'primevue/button'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps<{
  postId: number
}>()

const deleting = ref(false)

const emit = defineEmits<{
  deleted: []
}>()

const deletePost = async () => {
  deleting.value = true
  await deleteForumPost(props.postId)
  showToast('', t('forum.post_deleted_success'), 'success', 2000)
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
