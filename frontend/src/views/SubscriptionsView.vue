<template>
  <Tabs :value="currentTab" @update:value="tabChanged">
    <TabList>
      <Tab value="forum_thread_posts">{{ t('subscription.forum_thread_posts') }}</Tab>
      <Tab value="title_group_comments">{{ t('subscription.title_group_comments') }}</Tab>
      <Tab value="title_group_torrents">{{ t('subscription.title_group_torrents') }}</Tab>
      <Tab value="torrent_request_comments">{{ t('subscription.torrent_request_comments') }}</Tab>
    </TabList>
    <TabPanels>
      <TabPanel value="forum_thread_posts" v-if="currentTab === 'forum_thread_posts'">
        <ForumThreadPostsSubscriptionTable />
      </TabPanel>
      <TabPanel value="title_group_comments" v-if="currentTab === 'title_group_comments'">
        <TitleGroupSubscriptionTable :fetchFunction="getTitleGroupCommentsSubscriptions" :unsubscribeFunction="removeTitleGroupCommentsSubscription" />
      </TabPanel>
      <TabPanel value="title_group_torrents" v-if="currentTab === 'title_group_torrents'">
        <TitleGroupSubscriptionTable :fetchFunction="getTitleGroupTorrentsSubscriptions" :unsubscribeFunction="removeTitleGroupTorrentsSubscription" />
      </TabPanel>
      <TabPanel value="torrent_request_comments" v-if="currentTab === 'torrent_request_comments'">
        <TorrentRequestCommentsSubscriptionTable />
      </TabPanel>
    </TabPanels>
  </Tabs>
</template>

<script setup lang="ts">
import { Tab, TabList, TabPanel, TabPanels, Tabs } from 'primevue'
import ForumThreadPostsSubscriptionTable from '@/components/subscription/ForumThreadPostsSubscriptionTable.vue'
import TitleGroupSubscriptionTable from '@/components/subscription/TitleGroupSubscriptionTable.vue'
import TorrentRequestCommentsSubscriptionTable from '@/components/subscription/TorrentRequestCommentsSubscriptionTable.vue'
import { useI18n } from 'vue-i18n'
import { onMounted, ref, watch } from 'vue'
import { useRouter } from 'vue-router'
import {
  getTitleGroupCommentsSubscriptions,
  getTitleGroupTorrentsSubscriptions,
  removeTitleGroupCommentsSubscription,
  removeTitleGroupTorrentsSubscription,
} from '@/services/api-schema'

const { t } = useI18n()
const router = useRouter()

const currentTab = ref('forum_thread_posts')

const tabChanged = (tab: string | number) => {
  router.push({ query: { tab } })
}

onMounted(() => {
  if (router.currentRoute.value.query.tab) {
    currentTab.value = router.currentRoute.value.query.tab as string
  }
})

watch(
  () => router.currentRoute.value.query.tab,
  (newTab) => {
    if (newTab) {
      currentTab.value = newTab as string
    }
  },
)
</script>
