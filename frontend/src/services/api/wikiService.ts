import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type WikiArticle = components['schemas']['WikiArticle']

export const getWikiArticle = async (articleId: number) => {
  return (await api.get<WikiArticle>(`/wiki/articles?id=${articleId}`)).data
}
