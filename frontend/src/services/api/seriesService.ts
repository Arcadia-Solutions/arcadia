import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type SeriesAndTitleGroupHierarchyLite = components['schemas']['SeriesAndTitleGroupHierarchyLite']

export type Series = components['schemas']['Series']

export type SeriesLite = components['schemas']['SeriesLite']

export type SearchSeriesQuery = components['schemas']['SearchSeriesQuery']

export type SeriesSearchResponse = components['schemas']['SeriesSearchResponse']

export type SeriesSearchResult = components['schemas']['SeriesSearchResult']

export type UserCreatedSeries = components['schemas']['UserCreatedSeries']

export const getSeries = async (id: number): Promise<SeriesAndTitleGroupHierarchyLite> => {
  return (await api.get<SeriesAndTitleGroupHierarchyLite>('/series?id=' + id)).data
}

export const searchSeries = async (form: SearchSeriesQuery): Promise<SeriesSearchResponse> => {
  return (await api.get<SeriesSearchResponse>('/search/series', { params: form })).data
}

export const createSeries = async (series: UserCreatedSeries): Promise<Series> => {
  return (await api.post<Series>('/series', series)).data
}
