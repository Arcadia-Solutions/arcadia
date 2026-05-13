<template>
  <Toast position="bottom-right" group="bottom-right">
    <template #message="slotProps">
      <div class="p-toast-detail notification">
        {{ slotProps.message.detail }}
        <br />
        <RouterLink :to="viewRoutes[slotProps.message.summary!]">{{ t('general.view') }}</RouterLink>
      </div>
    </template>
  </Toast>
</template>

<script setup lang="ts">
import { removeToastGroup, showToast } from '@/main'
import { useNotificationsStore } from '@/stores/notifications'
import { useUserStore } from '@/stores/user'
import { Toast } from 'primevue'
import { computed, nextTick, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { RouterLink } from 'vue-router'

const notificationsStore = useNotificationsStore()
const userStore = useUserStore()
const { t } = useI18n()

const viewRoutes = computed<Record<string, string>>(() => ({
  announcement: '/forum/sub-category/1',
  conversation: '/conversations',
  forum_sub_category_thread: '/notifications?tab=forum_sub_category_threads',
  forum_thread_post: '/notifications?tab=forum_thread_posts',
  title_group_comment: '/notifications?tab=title_group_comments',
  torrent_request_comment: '/notifications?tab=torrent_request_comments',
  staff_pm: userStore.permissions.includes('read_staff_pm') ? '/staff-dashboard?tab=staffPms' : '/staff-pms',
  torrent_deletion: '/notifications?tab=torrent_deletions',
}))

watch(
  [
    () => notificationsStore.announcements,
    () => notificationsStore.conversations,
    () => notificationsStore.forum_sub_category_threads,
    () => notificationsStore.forum_thread_posts,
    () => notificationsStore.title_group_comments,
    () => notificationsStore.torrent_request_comments,
    () => notificationsStore.staff_pm_messages,
    () => notificationsStore.torrent_deletions,
  ],
  async ([
    newAnnouncements,
    newConversations,
    newForumSubCategoryThreads,
    newForumThreadPosts,
    newTitleGroupComments,
    newTorrentRequestComments,
    newStaffPms,
    newTorrentDeletions,
  ]) => {
    removeToastGroup('bottom-right')
    await nextTick()

    if (newAnnouncements > 0) {
      showToast('announcement', t('user.announcements', [newAnnouncements]), 'info', undefined, false, 'bottom-right')
    }

    if (newConversations > 0) {
      showToast('conversation', t('user.conversations', [newConversations]), 'info', undefined, false, 'bottom-right')
    }

    if (newForumSubCategoryThreads > 0) {
      showToast('forum_sub_category_thread', t('user.forum_sub_category_threads', [newForumSubCategoryThreads]), 'info', undefined, false, 'bottom-right')
    }

    if (newForumThreadPosts > 0) {
      showToast('forum_thread_post', t('user.forum_thread_posts', [newForumThreadPosts]), 'info', undefined, false, 'bottom-right')
    }

    if (newTitleGroupComments > 0) {
      showToast('title_group_comment', t('user.title_group_comments', [newTitleGroupComments]), 'info', undefined, false, 'bottom-right')
    }

    if (newTorrentRequestComments > 0) {
      showToast('torrent_request_comment', t('user.torrent_request_comments', [newTorrentRequestComments]), 'info', undefined, false, 'bottom-right')
    }

    if (newStaffPms > 0) {
      showToast('staff_pm', t('user.staff_pm_messages', [newStaffPms]), 'info', undefined, false, 'bottom-right')
    }

    if (newTorrentDeletions > 0) {
      showToast('torrent_deletion', t('user.torrent_deletions', [newTorrentDeletions]), 'info', undefined, false, 'bottom-right')
    }
  },
  { immediate: true },
)
</script>
<style scoped>
.notification {
  margin-bottom: -3px;
}
</style>
