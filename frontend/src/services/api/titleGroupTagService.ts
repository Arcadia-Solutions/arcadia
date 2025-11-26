import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type TitleGroupTagSearchResult = components['schemas']['TitleGroupTagSearchResult']

export type UserCreatedTitleGroupTag = components['schemas']['UserCreatedTitleGroupTag']

export type TitleGroupTag = components['schemas']['TitleGroupTag']

export type AppliedTitleGroupTag = components['schemas']['AppliedTitleGroupTag']

export const searchTitleGroupTag = async (name: string): Promise<TitleGroupTagSearchResult[]> => {
  return (await api.get<TitleGroupTagSearchResult[]>('/search/title-group-tags', { params: { name } })).data
}

export const createTitleGroupTag = async (titleGroupTag: UserCreatedTitleGroupTag): Promise<TitleGroupTag> => {
  return (await api.post<TitleGroupTag>('/title-group-tags', titleGroupTag)).data
}

export const applyTitleGroupTag = async (titleGroupTag: AppliedTitleGroupTag) => {
  return (await api.post('/title-group-tags/apply', titleGroupTag)).data
}
