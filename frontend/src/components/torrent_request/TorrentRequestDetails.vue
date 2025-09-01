<template>
  <ContentContainer id="torrent-request-details">
    <table>
      <tbody>
        <tr v-for="requirement in requirements" :key="requirement">
          <td class="name">{{ t(`torrent_request.acceptable_${requirement}`) }}</td>
          <td class="value">
            {{
              torrentRequest[requirement] && typeof torrentRequest[requirement] === 'object' && (torrentRequest[requirement] as string[] | string).length > 0
                ? (torrentRequest[requirement] as string[]).join(', ')
                : t('general.any')
            }}
          </td>
        </tr>
      </tbody>
      <tbody class="extra-info">
        <tr>
          <td class="name">{{ t('torrent_request.bounty') }}</td>
          <td class="value">
            {{ bytesToReadable(votes.reduce((sum, vote) => sum + vote.bounty_upload, 0)) }} +
            {{ votes.reduce((sum, vote) => sum + vote.bounty_bonus_points, 0) }} bp
            <span class="votes-amount">({{ votes.length }} {{ t('torrent_request.vote', votes.length) }})</span>
          </td>
        </tr>
      </tbody>
    </table>
    <div class="new-vote">
      <span class="bold">{{ t('torrent_request.new_vote') }}</span>
      <div class="inputs">
        <FloatLabel>
          <InputNumber v-model="bountyUploadUnited" />
          <label for="name">{{ t('user.upload') }}</label>
        </FloatLabel>
        <Select v-model="bountyUploadUnit" :options="selectableUploadUnits" size="small" class="select-unit" />
        <FloatLabel>
          <InputNumber v-model="newVote.bounty_bonus_points" />
          <label for="name">{{ t('user.bonus_points') }}</label>
        </FloatLabel>
        <Button size="small" :loading="newVoteLoading" :label="t('torrent_request.vote')" @click="vote" />
      </div>
    </div>
  </ContentContainer>
</template>

<script lang="ts" setup>
import type { TorrentRequest } from '@/services/api/torrentRequestService'
import ContentContainer from '../ContentContainer.vue'
import { FloatLabel } from 'primevue'
import { InputNumber, Select, Button } from 'primevue'
import { useI18n } from 'vue-i18n'
import { ref } from 'vue'
import {
  newTorrentRequestVote,
  type TorrentRequestVote,
  type TorrentRequestVoteHierarchy,
  type UserCreatedTorrentRequestVote,
} from '@/services/api/torrentRequestVoteService'
import { bytesToReadable } from '@/services/helpers'
import { useUserStore } from '@/stores/user'
import { showToast } from '@/main'

const { t } = useI18n()
const requirements = ref<(keyof TorrentRequest)[]>([
  'audio_bitrate_sampling',
  'audio_channels',
  'audio_codec',
  'container',
  'edition_name',
  'features',
  'source',
  'video_codec',
  'video_resolution',
])
const selectableUploadUnits = ref(['MiB', 'GiB', 'TiB'])

const props = defineProps<{
  torrentRequest: TorrentRequest
  votes: TorrentRequestVote[]
}>()

const emit = defineEmits<{
  voted: [TorrentRequestVoteHierarchy]
}>()

const userStore = useUserStore()

const newVote = ref<UserCreatedTorrentRequestVote>({
  bounty_bonus_points: 0,
  bounty_upload: 0,
  torrent_request_id: 0,
})
const bountyUploadUnit = ref('MiB')
const bountyUploadUnited = ref(0)
const newVoteLoading = ref(false)

const vote = async () => {
  newVoteLoading.value = true
  const unitsInBytes: Record<string, number> = {
    MiB: 1024 ** 2,
    GiB: 1024 ** 3,
    TiB: 1024 ** 4,
  }
  newVote.value.bounty_upload = bountyUploadUnited.value * unitsInBytes[bountyUploadUnit.value]
  newTorrentRequestVote({ ...newVote.value, torrent_request_id: props.torrentRequest.id })
    .then((castedVote) => {
      emit('voted', { ...castedVote, created_by: userStore })
      showToast('', t('torrent_request.vote_successful'), 'success', 3000, true, 'tr')
    })
    .finally(() => (newVoteLoading.value = false))
}
</script>
<style scoped>
table {
  .name {
    vertical-align: middle;
    text-align: right;
    padding-right: 5px;
    font-weight: bold;
  }
  .value {
    text-align: left;
  }
}
.votes-amount {
  margin-left: 4px;
  font-size: 0.9em;
}
.new-vote {
  margin-top: 40px;
  display: flex;
  align-items: center;
  .inputs {
    display: flex;
    align-items: end;
    .p-floatlabel {
      margin-left: 10px;
    }
  }
  .p-button {
    margin-left: 15px;
  }
}
</style>
<style>
.new-vote {
  .p-inputnumber-input {
    width: 6em;
  }
}
.select-unit {
  width: 5em;
}
</style>
