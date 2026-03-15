import { useNotificationsStore } from '@/stores/notifications'
import { type NotificationCounts, getNotificationCounts } from '@/services/api-schema'
import { i18n } from '@/main'

let eventSource: EventSource | null = null
let reconnectDelay = 5000

const eventTypeToCountKey: Record<string, keyof NotificationCounts> = {
  forum_sub_category_thread: 'forum_sub_category_threads',
  forum_thread_post: 'forum_thread_posts',
  title_group_comment: 'title_group_comments',
  torrent_request_comment: 'torrent_request_comments',
  staff_pm_message: 'staff_pm_messages',
  conversation: 'conversations',
}

function sendBrowserNotification(body: string) {
  if (Notification.permission !== 'granted') return
  const siteName = import.meta.env.VITE_SITE_NAME
  new Notification(siteName, { body, tag: body, icon: '/favicon.ico' })
}

function refreshNotificationCounts() {
  getNotificationCounts().then((counts) => {
    const notificationsStore = useNotificationsStore()

    for (const key of Object.keys(counts) as (keyof NotificationCounts)[]) {
      notificationsStore[key] = counts[key]
    }
  })
}

export function connectNotificationStream() {
  disconnectNotificationStream()

  const token = localStorage.getItem('token')
  if (!token) return

  if ('Notification' in window && Notification.permission === 'default') {
    Notification.requestPermission()
  }

  refreshNotificationCounts()

  const baseUrl = import.meta.env.VITE_API_BASE_URL
  eventSource = new EventSource(`${baseUrl}/api/notifications/stream?token=${encodeURIComponent(token)}`)

  eventSource.onmessage = (event) => {
    const eventType = event.data as string

    if (eventType === 'refresh') {
      refreshNotificationCounts()
      return
    }

    const countKey = eventTypeToCountKey[eventType]
    if (!countKey) return

    const notificationsStore = useNotificationsStore()
    const newCount = notificationsStore[countKey] + 1
    notificationsStore[countKey] = newCount

    const t = i18n.global.t
    sendBrowserNotification(t(`user.${countKey}`, [newCount]))
  }

  eventSource.onopen = () => {
    reconnectDelay = 5000
  }

  eventSource.onerror = () => {
    disconnectNotificationStream()
    setTimeout(connectNotificationStream, reconnectDelay)
    reconnectDelay = Math.min(reconnectDelay * 2, 60000)
  }
}

export function disconnectNotificationStream() {
  if (eventSource) {
    eventSource.close()
    eventSource = null
  }
}
