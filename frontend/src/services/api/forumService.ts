import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type ForumOverview = components['schemas']['ForumOverview']

export type ForumCategoryHierarchy = components['schemas']['ForumCategoryHierarchy']

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

export const getForum = async () => {
  return (await api.get('/forum')).data
}

export type ForumSubCategoryHierarchy = components['schemas']['ForumSubCategoryHierarchy']

export const getForumSubCategory = async (forumSubCategoryId: number): Promise<ForumSubCategoryHierarchy> => {
  return (await api.get<ForumSubCategoryHierarchy>('/forum/sub-category?id=' + forumSubCategoryId)).data
}

export type ForumThreadAndPosts = components['schemas']['ForumThreadAndPosts']

export type ForumThreadHierarchy = components['schemas']['ForumThreadHierarchy']

export const getForumThreads = async (params: { id: number }): Promise<ForumThreadHierarchy[]> => {
  return (await api.get<ForumThreadHierarchy[]>(`/forum/thread?id=${params.id}`)).data
}
export const searchForumThreads = async (params: { title: string; offset?: number; limit?: number }): Promise<ForumThreadHierarchy[]> => {
  return (await api.get<ForumThreadHierarchy[]>(`/search/forum/thread?title=${params.title}`)).data
}

export type ForumPostHierarchy = components['schemas']['ForumPostHierarchy']

export const getForumThread = async (forumThreadId: number): Promise<ForumThreadAndPosts> => {
  return (await api.get<ForumThreadAndPosts>('/forum/thread?id=' + forumThreadId)).data
}

export type UserCreatedForumPost = components['schemas']['UserCreatedForumPost']

export type ForumPost = components['schemas']['ForumPost']

export const postForumPost = async (form: UserCreatedForumPost): Promise<ForumPost> => {
  return (await api.post<ForumPost>('/forum/post', form)).data
}

export type UserCreatedForumThread = components['schemas']['UserCreatedForumThread']

export type ForumThread = components['schemas']['ForumThread']

export const postForumThread = async (form: UserCreatedForumThread): Promise<ForumThread> => {
  return (await api.post<ForumThread>('/forum/thread', form)).data
}
