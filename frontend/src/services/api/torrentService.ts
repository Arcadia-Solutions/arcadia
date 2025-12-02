import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type UploadedTorrent = components['schemas']['UploadedTorrent']

export type Torrent = components['schemas']['Torrent']

export type EditedTorrent = components['schemas']['EditedTorrent']

export type TorrentHierarchyLite = components['schemas']['TorrentHierarchyLite']

export type TorrentHierarchy = components['schemas']['TorrentHierarchy']

export const uploadTorrent = async (torrentForm: object) => {
  const formData = new FormData()
  for (const [key, value] of Object.entries(torrentForm)) {
    if (value != null) {
      formData.append(key, value)
    }
  }
  return (
    await api.post<Torrent>('/torrents', formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
    })
  ).data
}

export const downloadTorrent = async (torrent: Torrent, titleGroupName: string) => {
  const response = await api.get('/torrents?id=' + torrent.id, {
    responseType: 'blob',
  })

  const blob = response.data
  const url = window.URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `[${import.meta.env.VITE_SITE_NAME}] - ${titleGroupName} (${torrent.id}).torrent`
  document.body.appendChild(a)
  a.click()
  window.URL.revokeObjectURL(url)
  document.body.removeChild(a)
}

export type TorrentSearch = components['schemas']['TorrentSearch']

export type PaginatedResults_TitleGroupHierarchyLite = components['schemas']['PaginatedResults_TitleGroupHierarchyLite']

export const searchTorrentsLite = async (searchOptions: TorrentSearch): Promise<PaginatedResults_TitleGroupHierarchyLite> => {
  return (await api.get<PaginatedResults_TitleGroupHierarchyLite>('/search/torrents/lite', { params: searchOptions })).data
}

export type UserCreatedTorrentReport = components['schemas']['UserCreatedTorrentReport']

export type TorrentReport = components['schemas']['TorrentReport']

export type Features = components['schemas']['Features']

export type Extras = components['schemas']['Extras']

export const reportTorrent = async (torrentReport: UserCreatedTorrentReport) => {
  return (await api.post<TorrentReport>('/torrents/reports', torrentReport)).data
}

export const editTorrent = async (editedTorrent: EditedTorrent) => {
  return (await api.put<Torrent>('/torrents', editedTorrent)).data
}

export type UploadInformation = components['schemas']['UploadInformation']

export const getUploadInformation = async () => {
  return (await api.get<UploadInformation>('/torrents/upload-info')).data
}

export type TorrentToDelete = components['schemas']['TorrentToDelete']

export const deleteTorrent = async (form: TorrentToDelete) => {
  return (await api.delete('/torrents', { data: form })).data
}
