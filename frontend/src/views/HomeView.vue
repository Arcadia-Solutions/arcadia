<template>
  <div id="home-page">
    <div class="main">
      <LatestTorrents
        v-if="latestUploads"
        containerTitleLink="/torrents"
        :containerTitle="t('torrent.latest_uploads')"
        :titleGroups="latestUploads"
        showUploader
      />
      <KiwiIrcChat v-if="userStore.id && publicSettings.irc_enabled && publicSettings.irc_webchat_enabled" containerTitleLink="/wiki/article/3" />
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
      <ContentContainer :containerTitle="t('statistics.community_stats')" v-if="stats">
        <div>{{ t('statistics.enabled_users') }}: {{ stats.enabled_users }}</div>
        <div>
          {{ t('statistics.users_active_today') }}: {{ stats.users_active_today }} ({{ ((stats.users_active_today / stats.enabled_users) * 100).toFixed(2) }}%)
        </div>
        <div>
          {{ t('statistics.users_active_this_week') }}: {{ stats.users_active_this_week }} ({{
            ((stats.users_active_this_week / stats.enabled_users) * 100).toFixed(2)
          }}%)
        </div>
        <div>
          {{ t('statistics.users_active_this_month') }}: {{ stats.users_active_this_month }} ({{
            ((stats.users_active_this_month / stats.enabled_users) * 100).toFixed(2)
          }}%)
        </div>
        <div>
          {{ t('community.forum_threads') }}: {{ stats.forum_threads }}
          <RouterLink to="/stats?tab=forum" v-if="userStore.permissions.includes('view_stats_details')"><i class="pi pi-chart-bar" /></RouterLink>
        </div>
        <div>{{ t('community.forum_posts') }}: {{ stats.forum_posts }}</div>
        <div>{{ t('community.title_group_comments') }}: {{ stats.title_group_comments }}</div>
        <div>{{ t('community.request_comments') }}: {{ stats.torrent_request_comments }}</div>
      </ContentContainer>
      <ContentContainer :containerTitle="t('statistics.library_stats')" v-if="stats" style="margin-top: 10px">
        <div>
          {{ t('statistics.torrents') }}: {{ stats.torrents }}
          <RouterLink to="/stats?tab=torrents" v-if="userStore.permissions.includes('view_stats_details')"><i class="pi pi-chart-bar" /></RouterLink>
        </div>
        <div>
          {{ t('torrent.requests') }}: {{ stats.torrent_requests }} ({{ ((stats.torrent_requests_filled / stats.torrent_requests) * 100).toFixed(2) }}%
          {{ t('torrent_request.filled') }})
        </div>
        <div>{{ t('torrent.peers') }}: {{ stats.peers }}</div>
        <div>{{ t('torrent.seeders') }}: {{ stats.seeders }}</div>
        <div>{{ t('torrent.leecher', 2) }}: {{ stats.leechers }}</div>
        <div>{{ t('user.snatches', 2) }}: {{ stats.snatches }}</div>
        <div>{{ t('statistics.titles') }}: {{ stats.titles }}</div>
        <div>{{ t('statistics.artists') }}: {{ stats.artists }}</div>
        <div>{{ t('series.series', 2) }}: {{ stats.series }}</div>
        <div>{{ t('collage.collage', 2) }}: {{ stats.collages }}</div>
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
import KiwiIrcChat from '@/components/irc/KiwiIrcChat.vue'
import { useUserStore } from '@/stores/user'
import { usePublicArcadiaSettingsStore } from '@/stores/publicArcadiaSettings'
import {
  getHomeData,
  type ForumPostAndThreadName,
  type ForumSearchResult,
  type HomeStats,
  type TitleGroupCommentSearchResult,
  type TitleGroupLite,
} from '@/services/api-schema'

const userStore = useUserStore()
const publicSettings = usePublicArcadiaSettingsStore()

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
