import api from './api.ts'
import type { CreateEmojiRequest, EditEmoji200Response, EditEmojiRequest } from '../api-schema/api.ts'

// The generated createEmoji/editEmoji build no request body (an openapi-generator template bug
// that also affects uploadTorrent above), so these bypass the generated client and post the
// multipart form directly, the same way uploadTorrent does.

const appendEmojiFields = (formData: FormData, fields: Pick<CreateEmojiRequest, 'name' | 'image' | 'unicode_character'>) => {
  formData.append('name', fields.name)
  // Exactly one representation is sent when provided; sending neither on an edit keeps the
  // emoji's current representation.
  if (fields.image) {
    formData.append('image', fields.image)
  } else if (fields.unicode_character) {
    formData.append('unicode_character', fields.unicode_character)
  }
}

export const createEmoji = async (request: CreateEmojiRequest) => {
  const formData = new FormData()
  appendEmojiFields(formData, request)
  return (
    await api.post<EditEmoji200Response>('/api/emojis', formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
    })
  ).data.data
}

export const editEmoji = async (request: EditEmojiRequest) => {
  const formData = new FormData()
  formData.append('id', String(request.id))
  appendEmojiFields(formData, request)
  return (
    await api.put<EditEmoji200Response>('/api/emojis', formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
    })
  ).data.data
}
