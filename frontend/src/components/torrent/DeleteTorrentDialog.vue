<template>
  <div class="delete-torrent">
    <FloatLabel>
      <Textarea class="reason" name="reason" v-model="torrentToDelete.reason" rows="5" />
      <label for="reason">{{ t('general.reason') }}</label>
    </FloatLabel>
    <Button :label="t('torrent.delete_torrent')" size="small" :loading @click="sendDeletion()" />
  </div>
</template>

<script setup lang="ts">
import { deleteTorrent, type TorrentToDelete } from '@/services/api-schema'
import { Textarea, FloatLabel } from 'primevue'
import Button from 'primevue/button'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const torrentToDelete = ref<TorrentToDelete>({ reason: '', id: 0 })
const loading = ref(false)

const props = defineProps<{
  torrentId: number
}>()

const emit = defineEmits<{
  deleted: []
}>()

const sendDeletion = () => {
  loading.value = true
  torrentToDelete.value.id = props.torrentId
  deleteTorrent(torrentToDelete.value).then(() => {
    loading.value = false
    emit('deleted')
  })
}
</script>

<style scoped>
.delete-torrent {
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
</style>
