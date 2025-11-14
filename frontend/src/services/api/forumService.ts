import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type ForumOverview = components['schemas']['ForumOverview']

export type ForumCategoryHierarchy = components['schemas']['ForumCategoryHierarchy']

export const getForum = async (): Promise<ForumOverview> => {
  return (await api.get<ForumOverview>('/forum')).data
}

export type ForumSubCategoryHierarchy = components['schemas']['ForumSubCategoryHierarchy']

export const getForumSubCategory = async (forumSubCategoryId: number): Promise<ForumSubCategoryHierarchy> => {
  return (await api.get<ForumSubCategoryHierarchy>('/forum/sub-category?id=' + forumSubCategoryId)).data
}

export type ForumThreadHierarchy = components['schemas']['ForumThreadHierarchy']

export const getForumThreads = async (params: { id: number }): Promise<ForumThreadHierarchy[]> => {
  return (await api.get<ForumThreadHierarchy[]>(`/forum/thread?id=${params.id}`)).data
}

export type ForumPostHierarchy = components['schemas']['ForumPostHierarchy']

export type ForumThreadEnriched = components['schemas']['ForumThreadEnriched']

export const getForumThread = async (forumThreadId: number): Promise<ForumThreadEnriched> => {
  return (await api.get<ForumThreadEnriched>('/forum/thread?id=' + forumThreadId)).data
}

export type PaginatedResults_ForumPostHierarchy = components['schemas']['PaginatedResults_ForumPostHierarchy']

export type GetForumThreadPostsQuery = components['schemas']['GetForumThreadPostsQuery']

export const getForumThreadPosts = async (query: GetForumThreadPostsQuery): Promise<PaginatedResults_ForumPostHierarchy> => {
  return (
    await api.get<PaginatedResults_ForumPostHierarchy>(
      `/forum/thread/posts?thread_id=${query.thread_id}&page_size=${query.page_size}` +
        (query.page !== null ? `&page=${query.page}` : '') +
        (query.post_id !== null ? `&post_id=${query.post_id}` : ''),
    )
  ).data
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

export type ForumSearchResult = components['schemas']['ForumSearchResult']

export type ForumSearchQuery = components['schemas']['ForumSearchQuery']

export type PaginatedResults_ForumSearchResult = components['schemas']['PaginatedResults_ForumSearchResult']

export const searchForum = async (form: ForumSearchQuery): Promise<PaginatedResults_ForumSearchResult> => {
  return (await api.get<PaginatedResults_ForumSearchResult>('/search/forum', { params: form })).data
}
