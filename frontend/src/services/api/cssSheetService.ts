import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type CssSheet = components['schemas']['CssSheet']

export type UserCreatedCssSheet = components['schemas']['UserCreatedCssSheet']

export type EditedCssSheet = components['schemas']['EditedCssSheet']

export type CssSheetsEnriched = components['schemas']['CssSheetsEnriched']

export const getCssSheets = async (): Promise<CssSheetsEnriched> => {
  return (await api.get<CssSheetsEnriched>('/css-sheets')).data
}

export const getCssSheet = async (name: string): Promise<CssSheet> => {
  return (await api.get<CssSheet>(`/css-sheets/${name}`)).data
}

export const createCssSheet = async (sheet: UserCreatedCssSheet): Promise<CssSheet> => {
  return (await api.post<CssSheet>('/css-sheets', sheet)).data
}

export const editCssSheet = async (sheet: EditedCssSheet): Promise<CssSheet> => {
  return (await api.put<CssSheet>('/css-sheets', sheet)).data
}
