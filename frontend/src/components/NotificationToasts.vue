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
  forum_thread_post: '/notifications?tab=forum_thread_posts',
  title_group_comment: '/notifications?tab=title_group_comments',
  staff_pm: userStore.permissions.includes('read_staff_pm') ? '/staff-dashboard?tab=staffPms' : '/staff-pms',
}))

watch(
  [
    () => notificationsStore.unread_announcements_amount,
    () => notificationsStore.unread_conversations_amount,
    () => notificationsStore.unread_notifications_amount_forum_thread_posts,
    () => notificationsStore.unread_notifications_amount_title_group_comments,
    () => notificationsStore.unread_staff_pms_amount,
  ],
  async ([newAnnouncements, newConversations, newForumThreadPosts, newTitleGroupComments, newStaffPms]) => {
    removeToastGroup('bottom-right')
    await nextTick()

    if (newAnnouncements > 0) {
      showToast('announcement', t('user.unread_announcements', [newAnnouncements]), 'info', undefined, false, 'bottom-right')
    }

    if (newConversations > 0) {
      showToast('conversation', t('user.unread_messages_in_conversation', [newConversations]), 'info', undefined, false, 'bottom-right')
    }

    if (newForumThreadPosts > 0) {
      showToast('forum_thread_post', t('user.unread_notifications_forum_thread_posts', [newForumThreadPosts]), 'info', undefined, false, 'bottom-right')
    }

    if (newTitleGroupComments > 0) {
      showToast('title_group_comment', t('user.unread_notifications_title_group_comments', [newTitleGroupComments]), 'info', undefined, false, 'bottom-right')
    }

    if (newStaffPms > 0) {
      showToast('staff_pm', t('user.unread_notifications_staff_pms', [newStaffPms]), 'info', undefined, false, 'bottom-right')
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
