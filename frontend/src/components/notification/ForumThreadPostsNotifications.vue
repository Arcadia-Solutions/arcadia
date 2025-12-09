<template>
  <DataTable v-if="notifications.length > 0" :value="notifications" size="small">
    <Column :header="t('forum.thread_name')">
      <template #body="slotProps">
        <ForumThreadName
          :threadName="slotProps.data.forum_thread_name"
          :threadId="slotProps.data.forum_thread_id"
          :postId="slotProps.data.forum_post_id"
          @click="slotProps.data.is_read ? null : (notificationsStore.unread_notifications_amount_forum_thread_posts -= 1)"
        />
      </template>
    </Column>
    <Column :header="t('notification.notified_at')">
      <template #body="slotProps">
        {{ timeAgo(slotProps.data.created_at) }}
      </template>
    </Column>
  </DataTable>
  <div v-else class="wrapper-center">
    {{ t('notification.no_notification') }}
  </div>
</template>

<script setup lang="ts">
import { Column, DataTable } from 'primevue'
import { ref } from 'vue'
import { onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import ForumThreadName from '../forum/ForumThreadName.vue'
import { timeAgo } from '@/services/helpers'
import { useNotificationsStore } from '@/stores/notifications'
import { getNotificationsForForumThreadPosts, type NotificationForumThreadPost } from '@/services/api-schema'

const notificationsStore = useNotificationsStore()
const { t } = useI18n()

const includeRead = ref(false)
const notifications = ref<NotificationForumThreadPost[]>([])

const fetchNotifications = async () => {
  getNotificationsForForumThreadPosts(includeRead.value).then((n) => {
    notifications.value = n
  })
}

onMounted(async () => {
  await fetchNotifications()
})
</script>
