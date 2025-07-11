import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type SeriesAndTitleGroupHierarchyLite = components['schemas']['SeriesAndTitleGroupHierarchyLite']

export type Series = components['schemas']['Series']

export const getSeries = async (id: number): Promise<SeriesAndTitleGroupHierarchyLite> => {
  return (await api.get<SeriesAndTitleGroupHierarchyLite>('/series?id=' + id)).data
}
