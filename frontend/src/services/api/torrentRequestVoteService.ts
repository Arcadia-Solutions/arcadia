import type { components } from '@/api-schema/schema'
import api from './api'

export type UserCreatedTorrentRequestVote = components['schemas']['UserCreatedTorrentRequestVote']

export type TorrentRequestVote = components['schemas']['TorrentRequestVote']

export type TorrentRequestVoteHierarchy = components['schemas']['TorrentRequestVoteHierarchy']

export const newTorrentRequestVote = async (form: UserCreatedTorrentRequestVote): Promise<TorrentRequestVote> => {
  return (await api.post<TorrentRequestVote>('/torrent-requests/vote', form)).data
}
