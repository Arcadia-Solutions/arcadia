import api from './api.ts'
import type { Torrent } from '../api-schema/api.ts'

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
