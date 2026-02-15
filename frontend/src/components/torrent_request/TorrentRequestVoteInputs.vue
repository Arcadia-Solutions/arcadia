<template>
  <div class="new-torrent-request-vote">
    <div class="inputs">
      <template v-if="publicArcadiaSettings.torrent_request_vote_currencies.includes(TorrentRequestVoteCurrency.Upload)">
        <FloatLabel>
          <InputNumber v-model="bountyUploadUnited" />
          <label for="name">{{ t('user.upload') }}</label>
        </FloatLabel>
        <Select v-model="bountyUploadUnit" :options="selectableUploadUnits" size="small" class="select-unit" style="width: 6em" />
      </template>
      <FloatLabel v-if="publicArcadiaSettings.torrent_request_vote_currencies.includes(TorrentRequestVoteCurrency.BonusPoints)">
        <InputNumber v-model="displayBountyBonusPoints" style="width: 12em" />
        <label for="name">{{ publicArcadiaSettings.bonus_points_alias }}</label>
      </FloatLabel>
      <Button v-if="showVoteBtn" size="small" :loading="loading" :label="t('torrent_request.vote')" @click="vote" />
    </div>
    <Message v-if="showError" severity="error" size="small" variant="simple">{{ t('torrent_request.vote_bounty_required') }}</Message>
  </div>
</template>
<script setup lang="ts">
import { TorrentRequestVoteCurrency, type UserCreatedTorrentRequestVote } from '@/services/api-schema'
import { FloatLabel } from 'primevue'
import { InputNumber, Select, Button, Message } from 'primevue'
import { ref, computed } from 'vue'
import { rawToDisplayBp, displayToRawBp } from '@/services/helpers'
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

const displayBountyBonusPoints = computed({
  get: () => rawToDisplayBp(newVote.value.bounty_bonus_points, publicArcadiaSettings.bonus_points_decimal_places),
  set: (value: number) => {
    newVote.value.bounty_bonus_points = displayToRawBp(value, publicArcadiaSettings.bonus_points_decimal_places)
  },
})

const selectableUploadUnits = ref(['MiB', 'GiB', 'TiB'])
const bountyUploadUnit = ref('MiB')
const bountyUploadUnited = ref(0)
const showError = ref(false)

const vote = (): UserCreatedTorrentRequestVote | undefined => {
  const unitsInBytes: Record<string, number> = {
    MiB: 1024 ** 2,
    GiB: 1024 ** 3,
    TiB: 1024 ** 4,
  }
  newVote.value.bounty_upload = bountyUploadUnited.value * unitsInBytes[bountyUploadUnit.value]
  if (newVote.value.bounty_upload <= 0 && newVote.value.bounty_bonus_points <= 0) {
    showError.value = true
    return
  }
  showError.value = false
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
  flex-direction: column;
  align-items: center;
  .p-floatlabel {
    margin-left: 10px;
  }
}
.inputs {
  display: flex;
  align-items: end;
}
.p-button {
  margin-left: 10px;
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
