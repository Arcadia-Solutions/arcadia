import type { GetForumThreadPostsQuery, PaginatedResultsForumPostHierarchy } from '../api-schema/api.ts'
import api from './api.ts'

export const getForumThreadPosts = async (query: GetForumThreadPostsQuery): Promise<PaginatedResultsForumPostHierarchy> => {
  return (
    await api.get<PaginatedResultsForumPostHierarchy>(
      `/forum/thread/posts?thread_id=${query.thread_id}&page_size=${query.page_size}` +
        (query.page !== null ? `&page=${query.page}` : '') +
        (query.post_id !== null ? `&post_id=${query.post_id}` : ''),
    )
  ).data
}
