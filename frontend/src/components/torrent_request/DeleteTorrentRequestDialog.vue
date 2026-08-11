<template>
  <div class="delete-torrent-request">
    <Message severity="warn" size="small" variant="simple" class="warning">
      {{ deletingWithPermission ? t('torrent_request.deletion_warning_staff') : t('torrent_request.deletion_warning_author') }}
    </Message>
    <template v-if="deletingWithPermission">
      <div class="refund">
        <Checkbox v-model="refundBounty" binary inputId="refund_bounty" />
        <label for="refund_bounty">{{ t('torrent_request.refund_bounty') }}</label>
      </div>
      <FloatLabel class="input">
        <Textarea v-model="message" rows="4" fluid />
        <label>{{ t('torrent_request.deletion_message') }}</label>
      </FloatLabel>
    </template>
    <Button :label="t('torrent_request.delete_request')" severity="danger" size="small" :loading @click="deleteRequest" />
  </div>
</template>

<script setup lang="ts">
import { deleteTorrentRequest } from '@/services/api-schema'
import { Checkbox, FloatLabel, Message, Textarea } from 'primevue'
import Button from 'primevue/button'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps<{
  torrentRequestId: number
  // the voters can only be given a message and their bounty back when deleting with the delete_torrent_request permission
  deletingWithPermission: boolean
}>()

const emit = defineEmits<{
  deleted: []
}>()

const refundBounty = ref(true)
const message = ref('')
const loading = ref(false)

const deleteRequest = () => {
  loading.value = true
  deleteTorrentRequest({
    id: props.torrentRequestId,
    refund_bounty: props.deletingWithPermission && refundBounty.value,
    message: message.value.trim() || null,
  })
    .then(() => {
      emit('deleted')
    })
    .finally(() => {
      loading.value = false
    })
}
</script>

<style scoped>
.delete-torrent-request {
  padding-top: 20px;
  width: 30em !important;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}
.warning {
  margin-bottom: 20px;
  text-align: center;
}
.refund {
  margin-bottom: 20px;
  display: flex;
  align-items: center;
  gap: 8px;
}
.input {
  width: 25em;
  margin-bottom: 20px;
}
</style>
