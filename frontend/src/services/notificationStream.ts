import { useNotificationsStore } from '@/stores/notifications'
import { type NotificationCounts, getNotificationCounts } from '@/services/api-schema'
import { i18n } from '@/main'

let eventSource: EventSource | null = null
let reconnectDelay = 5000
let reconnectTimeout: ReturnType<typeof setTimeout> | null = null
let channel: BroadcastChannel | null = null
let releaseLock: (() => void) | null = null
let lockAbort: AbortController | null = null

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

    channel?.postMessage({ type: 'counts', counts })
  })
}

function handleNotificationEvent(eventType: string) {
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

function startEventSource() {
  closeEventSource()
  if (reconnectTimeout) {
    clearTimeout(reconnectTimeout)
    reconnectTimeout = null
  }

  const token = localStorage.getItem('token')
  if (!token) return

  const baseUrl = import.meta.env.VITE_API_BASE_URL
  eventSource = new EventSource(`${baseUrl}/api/notifications/stream?token=${encodeURIComponent(token)}`)

  eventSource.onmessage = (event) => {
    const eventType = event.data as string
    handleNotificationEvent(eventType)
    channel?.postMessage(eventType)
  }

  eventSource.onopen = () => {
    reconnectDelay = 5000
  }

  eventSource.onerror = () => {
    closeEventSource()
    if (!reconnectTimeout) {
      reconnectTimeout = setTimeout(startEventSource, reconnectDelay)
      reconnectDelay = Math.min(reconnectDelay * 2, 60000)
    }
  }
}

function closeEventSource() {
  if (eventSource) {
    eventSource.close()
    eventSource = null
  }
}

export function connectNotificationStream() {
  disconnectNotificationStream()

  const token = localStorage.getItem('token')
  if (!token) return

  if ('Notification' in window && Notification.permission === 'default') {
    Notification.requestPermission()
  }

  channel = new BroadcastChannel('notification-stream')
  channel.onmessage = (event) => {
    if (event.data?.type === 'counts') {
      const notificationsStore = useNotificationsStore()
      for (const key of Object.keys(event.data.counts) as (keyof NotificationCounts)[]) {
        notificationsStore[key] = event.data.counts[key]
      }
    } else {
      handleNotificationEvent(event.data)
    }
  }

  // Only one tab holds the lock and maintains the SSE connection.
  // Other tabs receive events via BroadcastChannel.
  // When the leader tab closes, the lock is automatically released
  // and the next tab in the queue becomes the leader.
  lockAbort = new AbortController()
  navigator.locks
    .request('notification-stream', { signal: lockAbort.signal }, async () => {
      refreshNotificationCounts()
      startEventSource()
      await new Promise<void>((resolve) => {
        releaseLock = resolve
      })
      closeEventSource()
    })
    .catch(() => {})
}

export function disconnectNotificationStream() {
  if (reconnectTimeout) {
    clearTimeout(reconnectTimeout)
    reconnectTimeout = null
  }
  lockAbort?.abort()
  lockAbort = null
  if (releaseLock) {
    releaseLock()
    releaseLock = null
  }
  closeEventSource()
  if (channel) {
    channel.close()
    channel = null
  }
}
