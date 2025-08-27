import type { components } from '@/api-schema/schema'
import api from './api'

export type TorrentRequest = components['schemas']['TorrentRequest']

export type TorrentRequestHierarchyLite = components['schemas']['TorrentRequestHierarchyLite']

export type TorrentRequestWithTitleGroupLite = components['schemas']['TorrentRequestWithTitleGroupLite']

export type TorrentRequestAndAssociatedData = components['schemas']['TorrentRequestAndAssociatedData']

export type SearchTorrentRequestsQuery = components['schemas']['SearchTorrentRequestsQuery']

export type UserCreatedTorrentRequest = components['schemas']['UserCreatedTorrentRequest']

export const searchTorrentRequests = async (form: SearchTorrentRequestsQuery): Promise<TorrentRequestWithTitleGroupLite[]> => {
  return (await api.get<TorrentRequestWithTitleGroupLite[]>('/search/torrent-requests', { params: form })).data
}

export const getTorrentRequest = async (id: number): Promise<TorrentRequestAndAssociatedData> => {
  return (await api.get<TorrentRequestAndAssociatedData>('/torrent-requests', { params: { id } })).data
}

export const createTorrentRequest = async (torrentRequest: UserCreatedTorrentRequest): Promise<TorrentRequest> => {
  return (await api.post<TorrentRequest>('/torrent-request', torrentRequest)).data
}
