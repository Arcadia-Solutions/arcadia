<template>
  <div id="user-sidebar">
    <ImagePreview :imageLink="user.avatar ?? '/default_user_avatar.png'" :alt="user.username + '\'s avatar'" />
    <ContentContainer :container-title="t('community.statistics')" class="stats-container">
      {{ t('user.joined_at') }}:
      <span v-tooltip.top="formatDate(user.created_at)">{{ timeAgo(user.created_at) }}</span>
      <br />
      {{ t('user.last_seen') }}:
      <span v-tooltip.top="formatDate(user.last_seen)">{{ timeAgo(user.last_seen) }}</span>
      <br />
      {{ t('user.class') }}: {{ user.class_name }}
      <br />
      <template v-if="shouldStatBeDisplayed('bonus_points')">
        {{ publicArcadiaSettings.bonus_points_alias }}: {{ formatBp(user.bonus_points, publicArcadiaSettings.bonus_points_decimal_places) }}
        <br />
      </template>
      <template v-if="shouldStatBeDisplayed('uploaded')">
        {{ t('general.uploaded') }}: {{ bytesToReadable(user.uploaded) }}
        <br />
      </template>
      <template v-if="shouldStatBeDisplayed('real_uploaded')">
        {{ t('general.uploaded_real') }}: {{ bytesToReadable(user.real_uploaded) }}
        <br />
      </template>
      <template v-if="shouldStatBeDisplayed('downloaded')">
        {{ t('general.downloaded') }}: {{ bytesToReadable(user.downloaded) }}
        <br />
      </template>
      <template v-if="shouldStatBeDisplayed('real_downloaded')">
        {{ t('general.downloaded_real') }}: {{ bytesToReadable(user.real_downloaded) }}
        <br />
      </template>
      <template v-if="shouldStatBeDisplayed('ratio')">
        {{ t('general.ratio') }}: {{ user.downloaded > 0 ? (user.uploaded / user.downloaded).toFixed(2) : '∞' }}
        <br />
      </template>
      <template v-if="shouldStatBeDisplayed('seeding')">
        {{ t('torrent.seeding') }}: {{ user.seeding }}
        <br />
      </template>
      <template v-if="shouldStatBeDisplayed('leeching')">
        {{ t('torrent.leeching') }}: {{ user.leeching }}
        <br />
      </template>
      <template v-if="shouldStatBeDisplayed('snatched')">
        {{ t('torrent.snatched') }}: {{ user.snatched }}
        <br />
      </template>
      <template v-if="shouldStatBeDisplayed('seeding_size')">
        {{ t('user.seeding_size') }}: {{ bytesToReadable(user.seeding_size) }}
        <br />
      </template>
      <template v-if="shouldStatBeDisplayed('average_seeding_time')">
        {{ t('user.average_seeding_time') }}: {{ secondsToReadable(user.average_seeding_time) }}
        <br />
      </template>
      <template v-if="shouldStatBeDisplayed('freeleech_tokens') && 'freeleech_tokens' in user">
        {{ t('user.freeleech_tokens') }}: {{ user.freeleech_tokens }}
        <br />
      </template>
      <template v-if="shouldStatBeDisplayed('current_streak') && 'current_streak' in user">
        {{ t('user.current_streak') }}: {{ user.current_streak }}
        <br />
      </template>
      <template v-if="shouldStatBeDisplayed('highest_streak') && 'highest_streak' in user">
        {{ t('user.highest_streak') }}: {{ user.highest_streak }}
        <br />
      </template>
    </ContentContainer>
    <ContentContainer :container-title="t('community.community')" class="stats-container">
      <template v-if="shouldStatBeDisplayed('title_groups')">
        {{ t('artist.title_groups') }}: {{ user.title_groups }}
        <br />
      </template>
      <template v-if="shouldStatBeDisplayed('edition_groups')">
        {{ t('edition_group.edition_group', 2) }}: {{ user.edition_groups }}
        <br />
      </template>
      <template v-if="shouldStatBeDisplayed('torrents')">
        {{ t('statistics.torrents') }}: {{ user.torrents }}
        <br />
      </template>
      <template v-if="shouldStatBeDisplayed('forum_threads')">
        {{ t('community.forum_threads') }}: {{ user.forum_threads }}
        <br />
      </template>
      <template v-if="shouldStatBeDisplayed('forum_posts')">
        {{ t('community.forum_posts') }}: {{ user.forum_posts }}
        <br />
      </template>
      <template v-if="shouldStatBeDisplayed('collages_started')">
        {{ t('community.collages_started') }}: {{ user.collages_started }}
        <br />
      </template>
      <template v-if="shouldStatBeDisplayed('title_group_comments')">
        {{ t('community.title_group_comments') }}: {{ user.title_group_comments }}
        <br />
      </template>
      <template v-if="shouldStatBeDisplayed('request_comments')">
        {{ t('community.request_comments') }}: {{ user.request_comments }}
        <br />
      </template>
      <template v-if="shouldStatBeDisplayed('requests_voted')">
        {{ t('community.request_voted') }}: {{ user.requests_voted }}
        <br />
      </template>
      <template v-if="shouldStatBeDisplayed('requests_filled')">
        {{ t('community.request_filled') }}: {{ user.requests_filled }}
        <br />
      </template>
      <template v-if="shouldStatBeDisplayed('artist_comments')">
        {{ t('community.artist_comments') }}: {{ user.artist_comments }}
        <br />
      </template>
      <template v-if="shouldStatBeDisplayed('invited')">
        {{ t('community.invited') }}: {{ user.invited }}
        <br />
      </template>
      <template v-if="shouldStatBeDisplayed('invitations')">
        {{ t('user.invitations') }}: {{ user.invitations }}
        <br />
      </template>
    </ContentContainer>
  </div>
</template>

<script setup lang="ts">
import ContentContainer from '../ContentContainer.vue'
import { useI18n } from 'vue-i18n'
import { bytesToReadable, timeAgo, formatDate, formatBp, secondsToReadable } from '@/services/helpers'
import ImagePreview from '../ImagePreview.vue'
import type { PublicUser, User, DisplayableUserStats } from '@/services/api-schema'
import { usePublicArcadiaSettingsStore } from '@/stores/publicArcadiaSettings'

const { t } = useI18n()
const publicArcadiaSettings = usePublicArcadiaSettingsStore()

const shouldStatBeDisplayed = (stat: DisplayableUserStats) => publicArcadiaSettings.displayable_user_stats.includes(stat)

defineProps<{
  user: User | PublicUser
}>()
</script>

<style scoped>
.stats-container {
  margin-top: 10px;
}
</style>
<style>
#user-sidebar {
  .p-image-preview {
    width: 100% !important;
    border-radius: 7px;
    img {
      width: 100% !important;
      border-radius: 7px;
    }
  }
}
</style>
