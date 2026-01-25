<template>
  <div class="new-torrent-request-vote">
    <FloatLabel>
      <InputNumber v-model="bountyUploadUnited" />
      <label for="name">{{ t('user.upload') }}</label>
    </FloatLabel>
    <Select v-model="bountyUploadUnit" :options="selectableUploadUnits" size="small" class="select-unit" />
    <FloatLabel>
      <InputNumber v-model="newVote.bounty_bonus_points" />
      <label for="name">{{ publicArcadiaSettings.bonus_points_alias }}</label>
    </FloatLabel>
    <Button v-if="showVoteBtn" size="small" :loading="loading" :label="t('torrent_request.vote')" @click="vote" />
  </div>
</template>
<script setup lang="ts">
import type { UserCreatedTorrentRequestVote } from '@/services/api-schema'
import { FloatLabel } from 'primevue'
import { InputNumber, Select, Button } from 'primevue'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { usePublicArcadiaSettingsStore } from '@/stores/publicArcadiaSettings'

defineProps<{
  loading?: boolean
  showVoteBtn?: boolean
}>()
const emit = defineEmits<{
  vote: [UserCreatedTorrentRequestVote]
}>()
const { t } = useI18n()
const publicArcadiaSettings = usePublicArcadiaSettingsStore()
const newVote = ref<UserCreatedTorrentRequestVote>({
  bounty_bonus_points: 0,
  bounty_upload: 0,
  torrent_request_id: 0,
})

const selectableUploadUnits = ref(['MiB', 'GiB', 'TiB'])
const bountyUploadUnit = ref('MiB')
const bountyUploadUnited = ref(0)

const vote = (): UserCreatedTorrentRequestVote => {
  // TODO: add form validation
  const unitsInBytes: Record<string, number> = {
    MiB: 1024 ** 2,
    GiB: 1024 ** 3,
    TiB: 1024 ** 4,
  }
  newVote.value.bounty_upload = bountyUploadUnited.value * unitsInBytes[bountyUploadUnit.value]
  emit('vote', newVote.value)
  return newVote.value
}

defineExpose({
  vote,
})
</script>
<style scoped>
.new-torrent-request-vote {
  display: flex;
  align-items: end;
  .p-floatlabel {
    margin-left: 10px;
  }
}
.p-button {
  margin-left: 15px;
}
</style>
<style>
.new-torrent-request-vote {
  .p-inputnumber-input {
    width: 6em;
  }
  .select-unit {
    width: 5em;
  }
}
</style>
