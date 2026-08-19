<template>
  <Tabs :value="currentTab" size="small">
    <TabList>
      <Tab v-for="(tab, index) in tabs" :key="tab" :value="index">
        {{ t(`notification.${tab}`) }}
        <Badge v-if="unreadCounts[tab] > 0" :value="unreadCounts[tab]" severity="danger" style="margin-left: 6px" />
      </Tab>
    </TabList>
    <TabPanels v-if="isPageReady">
      <TabPanel :value="0"> <ForumSubCategoryThreadsNotifications :notifications="notifications.forum_sub_category_threads" /> </TabPanel>
      <TabPanel :value="1"> <ForumThreadPostsNotifications :notifications="notifications.forum_thread_posts" /> </TabPanel>
      <TabPanel :value="2"> <TitleGroupCommentsNotifications :notifications="notifications.title_group_comments" /> </TabPanel>
      <TabPanel :value="3"> <TitleGroupTorrentsNotifications :notifications="notifications.title_group_torrents" /> </TabPanel>
      <TabPanel :value="4"> <ArtistTitleGroupsNotifications :notifications="notifications.artist_title_groups" /> </TabPanel>
      <TabPanel :value="5"> <TorrentRequestCommentsNotifications :notifications="notifications.torrent_request_comments" /> </TabPanel>
      <TabPanel :value="6"> <TorrentDeletionsNotifications :notifications="notifications.torrent_deletions" /> </TabPanel>
    </TabPanels>
  </Tabs>
</template>

<script setup lang="ts">
import { Badge, Tab, TabList, TabPanel, TabPanels, Tabs } from 'primevue'
import ForumSubCategoryThreadsNotifications from '@/components/notification/ForumSubCategoryThreadsNotifications.vue'
import ForumThreadPostsNotifications from '@/components/notification/ForumThreadPostsNotifications.vue'
import TitleGroupCommentsNotifications from '@/components/notification/TitleGroupCommentsNotifications.vue'
import TitleGroupTorrentsNotifications from '@/components/notification/TitleGroupTorrentsNotifications.vue'
import ArtistTitleGroupsNotifications from '@/components/notification/ArtistTitleGroupsNotifications.vue'
import TorrentRequestCommentsNotifications from '@/components/notification/TorrentRequestCommentsNotifications.vue'
import TorrentDeletionsNotifications from '@/components/notification/TorrentDeletionsNotifications.vue'
import { useI18n } from 'vue-i18n'
import { onMounted, computed, ref } from 'vue'
import { useRoute } from 'vue-router'
import { getNotifications, type Notifications } from '@/services/api-schema'

const { t } = useI18n()
const route = useRoute()

const tabs = [
  'forum_sub_category_threads',
  'forum_thread_posts',
  'title_group_comments',
  'title_group_torrents',
  'artist_title_groups',
  'torrent_request_comments',
  'torrent_deletions',
] as const
const isPageReady = ref(false)
const currentTab = ref(0)

const notifications = ref<Notifications>({
  forum_sub_category_threads: [],
  forum_thread_posts: [],
  title_group_comments: [],
  title_group_torrents: [],
  artist_title_groups: [],
  torrent_request_comments: [],
  staff_pm_messages: [],
  torrent_deletions: [],
})

const unreadCounts = computed(() => ({
  forum_sub_category_threads: notifications.value.forum_sub_category_threads.filter((n) => !n.read_status).length,
  forum_thread_posts: notifications.value.forum_thread_posts.filter((n) => !n.read_status).length,
  title_group_comments: notifications.value.title_group_comments.filter((n) => !n.read_status).length,
  title_group_torrents: notifications.value.title_group_torrents.filter((n) => !n.read_status).length,
  artist_title_groups: notifications.value.artist_title_groups.filter((n) => !n.read_status).length,
  torrent_request_comments: notifications.value.torrent_request_comments.filter((n) => !n.read_status).length,
  torrent_deletions: notifications.value.torrent_deletions.filter((n) => !n.read_status).length,
}))

onMounted(() => {
  if (route.query.tab) {
    currentTab.value = tabs.indexOf(route.query.tab as (typeof tabs)[number])
  }

  getNotifications(false).then((data) => {
    notifications.value = data
    isPageReady.value = true
  })
})
</script>
