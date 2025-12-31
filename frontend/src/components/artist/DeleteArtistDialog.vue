<template>
  <div class="delete-dialog">
    <p>{{ t('artist.confirm_delete_artist') }}</p>
    <Button :label="t('general.delete')" severity="danger" size="small" :loading="deleting" @click="handleDelete" />
  </div>
</template>

<script setup lang="ts">
import { showToast } from '@/main'
import { deleteArtist } from '@/services/api-schema'
import Button from 'primevue/button'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps<{
  artistId: number
}>()

const deleting = ref(false)

const emit = defineEmits<{
  deleted: []
}>()

const handleDelete = async () => {
  deleting.value = true
  await deleteArtist(props.artistId)
  showToast('', t('artist.artist_deleted_success'), 'success', 2000)
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
