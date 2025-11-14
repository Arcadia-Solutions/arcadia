import type { components } from '@/api-schema/schema.js'
import api from './api.ts'

export type NotificationForumThreadPost = components['schemas']['NotificationForumThreadPost']

export const getNotificationsForumThreadPosts = async (includeRead: boolean): Promise<NotificationForumThreadPost[]> => {
  return (await api.get(`notifications/forum-thread-posts?include_read=${includeRead}`)).data
}
