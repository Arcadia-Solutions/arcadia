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
  a.download = `[${import.meta.env.VITE_SITE_NAME}] ${nameParts} (${torrentId}).torrent`
  document.body.appendChild(a)
  a.click()
  window.URL.revokeObjectURL(url)
  document.body.removeChild(a)
}
