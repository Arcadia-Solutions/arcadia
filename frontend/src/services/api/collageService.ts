import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type CollageAndAssociatedData = components['schemas']['CollageAndAssociatedData']

export type Collage = components['schemas']['Collage']

export type PaginatedResults_CollageSearchResult = components['schemas']['PaginatedResults_CollageSearchResult']

export type SearchCollagesQuery = components['schemas']['SearchCollagesQuery']

export type CollageSearchResult = components['schemas']['CollageSearchResult']

export type UserCreatedCollage = components['schemas']['UserCreatedCollage']

export type CollageType = components['schemas']['CollageType']

export type CollageCategory = components['schemas']['CollageCategory']

export type UserCreatedCollageEntry = components['schemas']['UserCreatedCollageEntry']

export type CollageEntry = components['schemas']['CollageEntry']

export const getCollage = async (id: number): Promise<CollageAndAssociatedData> => {
  return (await api.get<CollageAndAssociatedData>('/collages?id=' + id)).data
}

export const searchCollages = async (form: SearchCollagesQuery): Promise<PaginatedResults_CollageSearchResult> => {
  return (await api.get<PaginatedResults_CollageSearchResult>('/search/collages', { params: form })).data
}

export const createCollage = async (form: UserCreatedCollage): Promise<Collage> => {
  return (await api.post<Collage>('/collages', form)).data
}

export const createCollageEntries = async (form: UserCreatedCollageEntry[]): Promise<CollageEntry[]> => {
  return (await api.post<CollageEntry[]>('/collages/entries', form)).data
}
