<template>
  <div id="home-page">
    <div class="main">
      <LatestTorrents v-if="latestUploads" containerTitleLink="/torrents" :containerTitle="t('torrent.latest_uploads')" :titleGroups="latestUploads" />
      <LatestForumPosts v-if="latestForumPosts" :latestPosts="latestForumPosts" />
      <div class="announcements">
        <AnnouncementComponent v-for="announcement in recentAnnouncements" :key="announcement.id" :announcement class="announcement" />
      </div>
    </div>
    <div class="sidebar">
      <ContentContainer :containerTitle="t('statistics.statistics')" v-if="stats">
        <div v-for="(value, statName) in stats" :key="statName">
          {{ t(`statistics.${statName}`) }}: {{ value }}
          <span v-if="statName.includes('users_active')">({{ ((value / stats.enabled_users) * 100).toFixed(2) }}%)</span>
        </div>
      </ContentContainer>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getHome, type ForumPostAndThreadName, type HomeStats, type LatestForumPost } from '@/services/api/homeService'
import { onMounted } from 'vue'
import { ref } from 'vue'
import AnnouncementComponent from '@/components/forum/AnnouncementComponent.vue'
import ContentContainer from '@/components/ContentContainer.vue'
import { useI18n } from 'vue-i18n'
import type { TitleGroupLite } from '@/services/api/torrentService'
import LatestTorrents from '@/components/torrent/LatestTorrents.vue'
import LatestForumPosts from '@/components/forum/LatestForumPosts.vue'

const { t } = useI18n()

const recentAnnouncements = ref<ForumPostAndThreadName[]>()
const latestForumPosts = ref<LatestForumPost[]>()
const stats = ref<HomeStats>()
const latestUploads = ref<TitleGroupLite[]>()

const fetchHome = async () => {
  getHome().then((data) => {
    recentAnnouncements.value = data.recent_announcements
    latestForumPosts.value = data.latest_forum_posts
    stats.value = data.stats
    latestUploads.value = data.latest_uploads
  })
}

onMounted(() => {
  fetchHome()
})
</script>

<style scoped>
#home-page {
  display: flex;
  justify-content: space-between;
}
.main {
  width: 77%;
}
.sidebar {
  width: 22%;
}
.announcement {
  margin-top: 10px;
  margin-bottom: 10px;
}
</style>
