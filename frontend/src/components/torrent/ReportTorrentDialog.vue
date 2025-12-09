<template>
  <div class="report-torrent">
    <FloatLabel>
      <Textarea class="description" name="description" v-model="report.description" rows="5" />
      <label for="description">{{ t('general.description') }}</label>
    </FloatLabel>
    <Button label="Send report" size="small" :loading @click="sendReport()" />
  </div>
</template>

<script setup lang="ts">
import { createTorrentReport, type TorrentReport, type UserCreatedTorrentReport } from '@/services/api-schema'
import { Textarea, FloatLabel } from 'primevue'
import Button from 'primevue/button'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const report = ref<UserCreatedTorrentReport>({ description: '', reported_torrent_id: 0 })
const loading = ref(false)

const props = defineProps<{
  torrentId: number
}>()

const emit = defineEmits<{
  reported: [torrentReport: TorrentReport]
}>()

const sendReport = () => {
  loading.value = true
  report.value.reported_torrent_id = props.torrentId
  createTorrentReport(report.value)
    .then((data: TorrentReport) => {
      emit('reported', data)
    })
    .finally(() => {
      loading.value = false
    })
}
</script>

<style scoped>
.report-torrent {
  padding-top: 20px;
  width: 30em !important;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}
.description {
  width: 25em;
  margin-bottom: 20px;
}
</style>
