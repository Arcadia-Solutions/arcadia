import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type HomePage = components['schemas']['HomePage']

export type ForumPostAndThreadName = components['schemas']['ForumPostAndThreadName']

export type HomeStats = components['schemas']['HomeStats']

export type LatestForumPost = {
  id: number
  forum_thread_id: number
  created_at: string
  updated_at: string
  content: string
  sticky: boolean
  forum_thread_name: string
  forum_sub_category_id: number
  forum_sub_category_name: string
  forum_category_id: number
  forum_category_name: string
  created_by_id: number
  created_by_username: string
}

export const getHome = async () => {
  return (await api.get<HomePage>('/home')).data
}
