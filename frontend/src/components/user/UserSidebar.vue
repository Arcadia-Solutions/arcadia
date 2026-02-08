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
      {{ publicArcadiaSettings.bonus_points_alias }}: {{ formatBp(user.bonus_points, publicArcadiaSettings.bonus_points_decimal_places) }}
      <br />
      {{ t('general.uploaded') }}: {{ bytesToReadable(user.uploaded) }}
      <br />
      {{ t('general.uploaded_real') }}: {{ bytesToReadable(user.real_uploaded) }}
      <br />
      {{ t('general.downloaded') }}: {{ bytesToReadable(user.downloaded) }}
      <br />
      {{ t('general.downloaded_real') }}: {{ bytesToReadable(user.real_downloaded) }}
      <br />
      {{ t('user.seeding_size') }}: {{ bytesToReadable(user.seeding_size) }}
      <br />
    </ContentContainer>
    <ContentContainer :container-title="t('community.community')" class="stats-container">
      {{ t('artist.title_groups') }}: {{ user.title_groups }}
      <br />
      {{ t('edition_group.edition_group', 2) }}: {{ user.edition_groups }}
      <br />
      {{ t('statistics.torrents') }}: {{ user.torrents }}
      <br />
      {{ t('community.forum_threads') }}: {{ user.forum_threads }}
      <br />
      {{ t('community.forum_posts') }}: {{ user.forum_posts }}
      <br />
      {{ t('community.collages_started') }}: {{ user.collages_started }}
      <br />
      {{ t('community.title_group_comments') }}: {{ user.title_group_comments }}
      <br />
      {{ t('community.request_comments') }}: {{ user.request_comments }}
      <br />
      {{ t('community.request_voted') }}: {{ user.requests_voted }}
      <br />
      {{ t('community.request_filled') }}: {{ user.requests_filled }}
      <br />
      {{ t('community.artist_comments') }}: {{ user.artist_comments }}
      <br />
      {{ t('community.invited') }}: {{ user.invited }}
    </ContentContainer>
  </div>
</template>

<script setup lang="ts">
import ContentContainer from '../ContentContainer.vue'
import { useI18n } from 'vue-i18n'
import { bytesToReadable, timeAgo, formatDate, formatBp } from '@/services/helpers'
import ImagePreview from '../ImagePreview.vue'
import type { PublicUser, User } from '@/services/api-schema'
import { usePublicArcadiaSettingsStore } from '@/stores/publicArcadiaSettings'

const { t } = useI18n()
const publicArcadiaSettings = usePublicArcadiaSettingsStore()

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
