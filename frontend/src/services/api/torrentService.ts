import { config } from '@/config'
import api from './api.ts'
import type { EditTorrent200Response } from '../api-schema/api.ts'

export const uploadTorrent = async (torrentForm: object) => {
  const formData = new FormData()
  for (const [key, value] of Object.entries(torrentForm)) {
    if (value != null) {
      formData.append(key, value)
    }
  }
  return (
    // TODO: use the function from the generated client
    (
      await api.post<EditTorrent200Response>('/api/torrents', formData, {
        headers: {
          'Content-Type': 'multipart/form-data',
        },
      })
    ).data.data
  )
}

export const downloadUserTorrentsArchive = async (kind: 'uploaded' | 'snatched') => {
  // The generated client returns `void` for file endpoints, so the blob is fetched directly,
  // like `downloadTorrent` below. The endpoint only ever serves the current user's own torrents.
  const response = await api.get('/api/users/me/torrents-archive?type=' + kind, {
    responseType: 'blob',
  })

  const blob = response.data
  const url = window.URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `[${config.site_name}] ${kind} torrents.zip`
  document.body.appendChild(a)
  a.click()
  window.URL.revokeObjectURL(url)
  document.body.removeChild(a)
}

export const downloadTorrent = async (torrentId: number, titleGroupName: string, seriesName?: string, artistNames?: string[]) => {
  // TODO: use the function from the generated client
  const response = await api.get('/api/torrents?id=' + torrentId, {
    responseType: 'blob',
  })

  const artistPart = artistNames && artistNames.length > 0 ? (artistNames.length > 2 ? 'Various Artists' : artistNames.join(', ')) : ''
  const nameParts = [seriesName, artistPart, titleGroupName].filter(Boolean).join(' - ')

  const blob = response.data
  const url = window.URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `[${config.site_name}] ${nameParts} (${torrentId}).torrent`
  document.body.appendChild(a)
  a.click()
  window.URL.revokeObjectURL(url)
  document.body.removeChild(a)
}
