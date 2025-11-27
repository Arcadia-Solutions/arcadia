import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type TitleGroupTagLite = components['schemas']['TitleGroupTagLite']

export type TitleGroupTagEnriched = components['schemas']['TitleGroupTagEnriched']

export type PaginatedResults_TitleGroupTagLite = components['schemas']['PaginatedResults_TitleGroupTagLite']

export type PaginatedResults_TitleGroupTagEnriched = components['schemas']['PaginatedResults_TitleGroupTagEnriched']

export type UserCreatedTitleGroupTag = components['schemas']['UserCreatedTitleGroupTag']

export type TitleGroupTag = components['schemas']['TitleGroupTag']

export type AppliedTitleGroupTag = components['schemas']['AppliedTitleGroupTag']

export type EditedTitleGroupTag = components['schemas']['EditedTitleGroupTag']

export type DeleteTagRequest = components['schemas']['DeleteTagRequest']

export type SearchTitleGroupTagsQuery = components['schemas']['SearchTitleGroupTagsQuery']

export const searchTitleGroupTags = async (form: SearchTitleGroupTagsQuery): Promise<PaginatedResults_TitleGroupTagEnriched> => {
  return (await api.get<PaginatedResults_TitleGroupTagEnriched>('/search/title-group-tags', { params: form })).data
}

export type SearchTitleGroupTagsLiteQuery = components['schemas']['SearchTitleGroupTagsLiteQuery']

export const searchTitleGroupTagsLite = async (form: SearchTitleGroupTagsLiteQuery): Promise<PaginatedResults_TitleGroupTagLite> => {
  return (await api.get<PaginatedResults_TitleGroupTagLite>('/search/title-group-tags/lite', { params: form })).data
}

export const createTitleGroupTag = async (titleGroupTag: UserCreatedTitleGroupTag): Promise<TitleGroupTag> => {
  return (await api.post<TitleGroupTag>('/title-group-tags', titleGroupTag)).data
}

export const applyTitleGroupTag = async (titleGroupTag: AppliedTitleGroupTag) => {
  return (await api.post('/title-group-tags/apply', titleGroupTag)).data
}

export const editTitleGroupTag = async (tag: EditedTitleGroupTag): Promise<TitleGroupTag> => {
  return (await api.put<TitleGroupTag>('/title-group-tags', tag)).data
}

export const deleteTitleGroupTag = async (tagId: number) => {
  return (await api.delete('/title-group-tags', { data: { id: tagId } })).data
}
