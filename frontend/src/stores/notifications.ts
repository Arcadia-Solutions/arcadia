import type { NotificationCounts } from '@/services/api-schema'
import { defineStore } from 'pinia'

export const useNotificationsStore = defineStore('notifications', {
  state: (): NotificationCounts => ({
    announcements: 0,
    conversations: 0,
    forum_sub_category_threads: 0,
    forum_thread_posts: 0,
    title_group_comments: 0,
    torrent_request_comments: 0,
    staff_pm_messages: 0,
  }),
})
