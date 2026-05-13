<template>
  <div class="delete-torrent">
    <Form :initial-values="form" :resolver validate-on-submit :validate-on-value-update="false" validate-on-blur @submit="onSubmit" v-slot="$form">
      <div class="input">
        <FloatLabel>
          <Select
            name="deletion_reason"
            v-model="form.deletion_reason"
            :options="[
              { value: TorrentDeletionReason.Trumped, label: t('notification.deletion_reason_trumped') },
              { value: TorrentDeletionReason.Duplicate, label: t('notification.deletion_reason_duplicate') },
              { value: TorrentDeletionReason.Other, label: t('notification.deletion_reason_other') },
            ]"
            option-label="label"
            option-value="value"
            size="small"
            fluid
          />
          <label>{{ t('general.reason') }}</label>
        </FloatLabel>
        <Message v-if="$form.deletion_reason?.invalid" severity="error" size="small" variant="simple">
          {{ $form.deletion_reason.error?.message }}
        </Message>
      </div>
      <FloatLabel class="input">
        <Textarea name="extra_information" v-model="form.extra_information" rows="4" fluid />
        <label>{{ t('notification.extra_information') }}</label>
      </FloatLabel>
      <FloatLabel class="input">
        <InputNumber name="replacement_torrent_id" v-model="form.replacement_torrent_id" :use-grouping="false" size="small" fluid />
        <label>{{ t('notification.replacement_torrent_id') }}</label>
      </FloatLabel>
      <Button :label="t('torrent.delete_torrent')" size="small" type="submit" :loading />
    </Form>
  </div>
</template>

<script setup lang="ts">
import { deleteTorrent, TorrentDeletionReason, type TorrentToDelete } from '@/services/api-schema'
import { FloatLabel, InputNumber, Message, Select, Textarea } from 'primevue'
import { Form, type FormResolverOptions, type FormSubmitEvent } from '@primevue/forms'
import Button from 'primevue/button'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

type DeleteTorrentForm = {
  deletion_reason: TorrentDeletionReason | null
  extra_information: string
  replacement_torrent_id: number | null
}

const form = ref<DeleteTorrentForm>({
  deletion_reason: null,
  extra_information: '',
  replacement_torrent_id: null,
})
const loading = ref(false)

const props = defineProps<{
  torrentId: number
}>()

const emit = defineEmits<{
  deleted: []
}>()

const resolver = ({ values }: FormResolverOptions) => {
  const errors: Partial<Record<keyof DeleteTorrentForm, { message: string }[]>> = {}
  if (!values.deletion_reason) {
    errors.deletion_reason = [{ message: t('error.field_required') }]
  }
  return { errors }
}

const onSubmit = ({ valid }: FormSubmitEvent) => {
  if (!valid || !form.value.deletion_reason) return
  loading.value = true
  const payload: TorrentToDelete = {
    id: props.torrentId,
    deletion_reason: form.value.deletion_reason,
    extra_information: form.value.extra_information.trim() || null,
    replacement_torrent_id: form.value.replacement_torrent_id,
  }
  deleteTorrent(payload)
    .then(() => {
      emit('deleted')
    })
    .finally(() => {
      loading.value = false
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
.input {
  width: 25em;
  margin-bottom: 20px;
}
</style>
