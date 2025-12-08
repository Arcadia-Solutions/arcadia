<template>
  <ContentContainer id="torrent-request-details">
    <table>
      <tbody>
        <tr v-for="requirement in filteredRequirements" :key="requirement">
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
      <TorrentRequestVoteInputs showVoteBtn @vote="vote" :loading="newVoteLoading" />
    </div>
  </ContentContainer>
</template>

<script lang="ts" setup>
import ContentContainer from '../ContentContainer.vue'
import { useI18n } from 'vue-i18n'
import { ref } from 'vue'
import { bytesToReadable, isAttributeUsed } from '@/services/helpers'
import { useUserStore } from '@/stores/user'
import { showToast } from '@/main'
import { computed } from 'vue'
import TorrentRequestVoteInputs from './TorrentRequestVoteInputs.vue'
import {
  createTorrentRequestVote,
  type ContentType,
  type TorrentRequest,
  type TorrentRequestVote,
  type TorrentRequestVoteHierarchy,
  type UserCreatedTorrentRequestVote,
} from '@/services/api-schema'

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
const filteredRequirements = computed(() => {
  return requirements.value.filter((requirement) => isAttributeUsed(requirement, props.contentType))
})

const props = defineProps<{
  torrentRequest: TorrentRequest
  votes: TorrentRequestVote[]
  contentType: ContentType
}>()

const emit = defineEmits<{
  voted: [TorrentRequestVoteHierarchy]
}>()

const userStore = useUserStore()
const newVoteLoading = ref(false)

const vote = async (newVote: UserCreatedTorrentRequestVote) => {
  newVoteLoading.value = true

  createTorrentRequestVote({ ...newVote, torrent_request_id: props.torrentRequest.id })
    .then((castedVote) => {
      emit('voted', { ...castedVote, created_by: userStore })
      userStore.uploaded -= castedVote.bounty_upload
      userStore.bonus_points -= castedVote.bounty_bonus_points
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
}
</style>
