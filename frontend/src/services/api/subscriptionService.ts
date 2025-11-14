import api from './api.ts'

export const subscribeToTitleGroupTorrents = async (title_group_id: number) => {
  return (await api.post(`/subscriptions/title-group-torrents?title_group_id=${title_group_id}`)).data
}
export const unsubscribeToTitleGroupTorrents = async (title_group_id: number) => {
  return (await api.delete(`/subscriptions/title-group-torrents?title_group_id=${title_group_id}`)).data
}
export const subscribeToForumThreadPosts = async (thread_id: number) => {
  return (await api.post(`/subscriptions/forum-thread-posts?thread_id=${thread_id}`)).data
}
export const unsubscribeToForumThreadPosts = async (thread_id: number) => {
  return (await api.delete(`/subscriptions/forum-thread-posts?thread_id=${thread_id}`)).data
}
