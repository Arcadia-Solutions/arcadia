import { defineStore } from 'pinia'

export const useNotificationsStore = defineStore('notifications', {
  state: () => {
    return {
      unread_announcements_amount: 0,
      unread_conversations_amount: 0,
      unread_notifications_amount_forum_thread_posts: 0,
      unread_notifications_amount_title_group_comments: 0,
      unread_staff_pms_amount: 0,
    }
  },
})
