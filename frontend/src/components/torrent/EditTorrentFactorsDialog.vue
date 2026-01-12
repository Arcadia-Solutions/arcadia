<template>
  <div class="edit-factors">
    <FloatLabel>
      <InputNumber v-model="uploadFactor" :min="0" inputId="upload-factor" />
      <label for="upload-factor">{{ t('torrent.upload_factor') }}</label>
    </FloatLabel>
    <FloatLabel>
      <InputNumber v-model="downloadFactor" :min="0" inputId="download-factor" />
      <label for="download-factor">{{ t('torrent.download_factor') }}</label>
    </FloatLabel>
    <Button :label="t('general.confirm')" size="small" :loading="loading" @click="sendEdits()" />
  </div>
</template>

<script setup lang="ts">
import { editTorrentUploadDownloadFactors } from '@/services/api-schema'
import FloatLabel from 'primevue/floatlabel'
import InputNumber from 'primevue/inputnumber'
import Button from 'primevue/button'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps<{
  torrentId: number
  initialUploadFactor: number
  initialDownloadFactor: number
}>()

const uploadFactor = ref(props.initialUploadFactor)
const downloadFactor = ref(props.initialDownloadFactor)
const loading = ref(false)

const emit = defineEmits<{
  done: [uploadFactor: number, downloadFactor: number]
}>()

const sendEdits = () => {
  loading.value = true
  editTorrentUploadDownloadFactors({
    torrent_id: props.torrentId,
    upload_factor: uploadFactor.value,
    download_factor: downloadFactor.value,
  })
    .then(() => {
      emit('done', uploadFactor.value, downloadFactor.value)
    })
    .finally(() => {
      loading.value = false
    })
}
</script>

<style scoped>
.edit-factors {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  min-width: 250px;
}
</style>
