<template>
  <div id="home-page">
    <div class="main">
      <LatestTorrents v-if="latestUploads" containerTitleLink="/torrents" :containerTitle="t('torrent.latest_uploads')" :titleGroups="latestUploads" />
      <Tabs value="0" size="small" style="margin: 10px 0">
        <TabList>
          <Tab value="0">{{ t('forum.latest_forum_post', 2) }}</Tab>
          <Tab value="1">{{ t('title_group.latest_title_group_comment', 2) }}</Tab>
        </TabList>
        <TabPanels>
          <TabPanel value="0">
            <ForumSearchResults :search-results="latestPostsInThreads" />
          </TabPanel>
          <TabPanel value="1">
            <TitleGroupCommentSearchResults :search-results="latestTitleGroupComments" />
          </TabPanel>
        </TabPanels>
      </Tabs>
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
import { onMounted, ref } from 'vue'
import AnnouncementComponent from '@/components/forum/AnnouncementComponent.vue'
import ContentContainer from '@/components/ContentContainer.vue'
import { useI18n } from 'vue-i18n'
import { Tab, TabList, TabPanel, TabPanels, Tabs } from 'primevue'
import LatestTorrents from '@/components/torrent/LatestTorrents.vue'
import ForumSearchResults from '@/components/forum/ForumSearchResults.vue'
import TitleGroupCommentSearchResults from '@/components/title_group/TitleGroupCommentSearchResults.vue'
import {
  getHomeData,
  type ForumPostAndThreadName,
  type ForumSearchResult,
  type HomeStats,
  type TitleGroupCommentSearchResult,
  type TitleGroupLite,
} from '@/services/api-schema'

const { t } = useI18n()

const recentAnnouncements = ref<ForumPostAndThreadName[]>()
const stats = ref<HomeStats>()
const latestUploads = ref<TitleGroupLite[]>()
const latestPostsInThreads = ref<ForumSearchResult[]>([])
const latestTitleGroupComments = ref<TitleGroupCommentSearchResult[]>([])

const fetchHome = async () => {
  getHomeData().then((data) => {
    recentAnnouncements.value = data.recent_announcements
    stats.value = data.stats
    latestUploads.value = data.latest_uploads
    latestPostsInThreads.value = data.latest_posts_in_threads
    latestTitleGroupComments.value = data.latest_title_group_comments
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
