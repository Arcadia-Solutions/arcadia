<template>
  <div id="user-sidebar">
    <ImagePreview :imageLink="user.avatar ?? '/default_user_avatar.png'" :alt="user.username + '\'s avatar'" />
    <ContentContainer :container-title="t('community.statistics')" class="stats-container">
      <template v-for="row in statistics" :key="row.label">
        {{ row.label }}:
        <span v-if="row.tooltip" v-tooltip.top="row.tooltip">{{ row.text }}</span>
        <template v-else>{{ row.text }}</template>
        <br />
      </template>
    </ContentContainer>
    <ContentContainer :container-title="t('community.community')" class="stats-container">
      <template v-for="row in communityStatistics" :key="row.label">
        {{ row.label }}: {{ row.text }}
        <br />
      </template>
    </ContentContainer>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import ContentContainer from '../ContentContainer.vue'
import { useI18n } from 'vue-i18n'
import { bytesToReadable, timeAgo, formatDate, formatBp, formatNumber, secondsToReadable } from '@/services/helpers'
import ImagePreview from '../ImagePreview.vue'
import { DisplayableUserStats, type PublicUser, type User } from '@/services/api-schema'
import { useUserStatLabel } from '@/composables/useUserStatLabel'
import { usePublicArcadiaSettingsStore } from '@/stores/publicArcadiaSettings'

const { t } = useI18n()
const publicArcadiaSettings = usePublicArcadiaSettingsStore()
const userStatLabel = useUserStatLabel()

const props = defineProps<{
  user: User | PublicUser
}>()

// a displayed line of the sidebar. a statistic hidden site wide, or hidden by the user with their
// paranoia settings, has no line at all
interface UserStatisticRow {
  label: string
  text: string
  tooltip?: string
}

const shouldStatBeDisplayed = (stat: DisplayableUserStats) => publicArcadiaSettings.displayable_user_stats.includes(stat)

const amountRow = (stat: DisplayableUserStats, amount: number | null | undefined, format: (amount: number) => string = formatNumber) =>
  shouldStatBeDisplayed(stat) && amount != null ? { label: userStatLabel(stat), text: format(amount) } : null

const dateRow = (stat: DisplayableUserStats, date: string | null | undefined) =>
  shouldStatBeDisplayed(stat) && date != null ? { label: userStatLabel(stat), text: timeAgo(date), tooltip: formatDate(date) } : null

const ratioRow = () => {
  const { uploaded, downloaded } = props.user
  if (!shouldStatBeDisplayed(DisplayableUserStats.Ratio) || uploaded == null || downloaded == null) {
    return null
  }
  return { label: userStatLabel(DisplayableUserStats.Ratio), text: downloaded > 0 ? (uploaded / downloaded).toFixed(2) : '∞' }
}

const displayedRows = (rows: (UserStatisticRow | null)[]) => rows.filter((row) => row !== null)

const statistics = computed(() => {
  const user = props.user
  return displayedRows([
    dateRow(DisplayableUserStats.JoinedAt, user.created_at),
    dateRow(DisplayableUserStats.LastSeen, user.last_seen),
    { label: t('user.class'), text: user.class_name },
    amountRow(DisplayableUserStats.BonusPoints, user.bonus_points, (amount) => formatBp(amount, publicArcadiaSettings.bonus_points_decimal_places)),
    amountRow(DisplayableUserStats.Uploaded, user.uploaded, bytesToReadable),
    amountRow(DisplayableUserStats.RealUploaded, user.real_uploaded, bytesToReadable),
    amountRow(DisplayableUserStats.Downloaded, user.downloaded, bytesToReadable),
    amountRow(DisplayableUserStats.RealDownloaded, user.real_downloaded, bytesToReadable),
    ratioRow(),
    amountRow(DisplayableUserStats.Seeding, user.seeding),
    amountRow(DisplayableUserStats.Leeching, user.leeching),
    amountRow(DisplayableUserStats.Snatched, user.snatched),
    amountRow(DisplayableUserStats.SeedingSize, user.seeding_size, bytesToReadable),
    amountRow(DisplayableUserStats.AverageSeedingTime, user.average_seeding_time, secondsToReadable),
    // those statistics only exist on the profile of the logged in user
    amountRow(DisplayableUserStats.FreeleechTokens, 'freeleech_tokens' in user ? user.freeleech_tokens : null),
    amountRow(DisplayableUserStats.CurrentStreak, 'current_streak' in user ? user.current_streak : null),
    amountRow(DisplayableUserStats.HighestStreak, 'highest_streak' in user ? user.highest_streak : null),
  ])
})

const communityStatistics = computed(() => {
  const user = props.user
  return displayedRows([
    amountRow(DisplayableUserStats.TitleGroups, user.title_groups),
    amountRow(DisplayableUserStats.EditionGroups, user.edition_groups),
    amountRow(DisplayableUserStats.Torrents, user.torrents),
    amountRow(DisplayableUserStats.ForumThreads, user.forum_threads),
    amountRow(DisplayableUserStats.ForumPosts, user.forum_posts),
    amountRow(DisplayableUserStats.CollagesStarted, user.collages_started),
    amountRow(DisplayableUserStats.TitleGroupComments, user.title_group_comments),
    amountRow(DisplayableUserStats.RequestComments, user.request_comments),
    amountRow(DisplayableUserStats.RequestsVoted, user.requests_voted),
    amountRow(DisplayableUserStats.RequestsFilled, user.requests_filled),
    amountRow(DisplayableUserStats.ArtistComments, user.artist_comments),
    amountRow(DisplayableUserStats.Invited, user.invited),
    amountRow(DisplayableUserStats.Invitations, user.invitations),
  ])
})
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
