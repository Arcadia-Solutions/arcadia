import { defineStore } from 'pinia'

export const useNotificationsStore = defineStore('notifications', {
  state: () => {
    return {
      unread_conversations_amount: 0,
      unread_notifications_amount_forum_thread_posts: 0,
      unread_staff_pms_amount: 0,
    }
  },
})
