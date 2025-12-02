import type { components } from '@/api-schema/schema'
import api from './api.ts'
import type { TitleGroupHierarchyLite } from './artistService.ts'

export type TitleGroup = components['schemas']['TitleGroup']

export type EditedTitleGroup = components['schemas']['EditedTitleGroup']

export type TitleGroupCategory = components['schemas']['TitleGroupCategory']

export type TitleGroupLite = components['schemas']['TitleGroupLite']

export type TitleGroupAndAssociatedData = components['schemas']['TitleGroupAndAssociatedData']

export type ContentType = components['schemas']['ContentType']

export type PublicRating = components['schemas']['PublicRating']

export type Language = components['schemas']['Language']

export type EmbeddedLinks = {
  [key: string]: {
    [key: string]: string
  }
}

export const getTitleGroup = async (id: number): Promise<TitleGroupAndAssociatedData> => {
  return (await api.get<TitleGroupAndAssociatedData>('/title-groups?id=' + id)).data
}

export const getTitleGroupLite = async (id: number): Promise<TitleGroupLite> => {
  return (await api.get<TitleGroupLite>('/title-groups/lite?id=' + id)).data
}

export const searchTitleGroupLite = async (name: string, contentType: ContentType | null): Promise<TitleGroupHierarchyLite[]> => {
  return (await api.get<TitleGroupHierarchyLite[]>('/search/title-groups/lite?name=' + name + (contentType ? `&content_type=${contentType}` : ''))).data
}

export type UserCreatedTitleGroup = components['schemas']['UserCreatedTitleGroup']

export const createTitleGroup = async (titleGroup: UserCreatedTitleGroup) => {
  return (await api.post<TitleGroup>('/title-groups', titleGroup)).data
}

export const editTitleGroup = async (title_group: EditedTitleGroup): Promise<TitleGroup> => {
  return (await api.put<TitleGroup>('/title-groups', title_group)).data
}
