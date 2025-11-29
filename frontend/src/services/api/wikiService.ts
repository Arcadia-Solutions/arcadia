import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type WikiArticle = components['schemas']['WikiArticle']

export const getWikiArticle = async (articleId: number) => {
  return (await api.get<WikiArticle>(`/wiki/articles?id=${articleId}`)).data
}

export type EditedWikiArticle = components['schemas']['EditedWikiArticle']

export const editWikiArticle = async (article: EditedWikiArticle) => {
  return (await api.put<WikiArticle>('/wiki/articles', article)).data
}
