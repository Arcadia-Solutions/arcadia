import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type Artist = components['schemas']['Artist']

export type ArtistLite = components['schemas']['ArtistLite']

export type AffiliatedArtist = components['schemas']['AffiliatedArtist']

export type AffiliatedArtistHierarchy = components['schemas']['AffiliatedArtistHierarchy']

export type ArtistAndTitleGroupsLite = components['schemas']['ArtistAndTitleGroupsLite']

export type TitleGroupHierarchyLite = components['schemas']['TitleGroupHierarchyLite']

export type UserCreatedAffiliatedArtist = components['schemas']['UserCreatedAffiliatedArtist']

export type UserCreatedArtist = components['schemas']['UserCreatedArtist']

export const getArtist = async (id: number): Promise<ArtistAndTitleGroupsLite> => {
  return (await api.get<ArtistAndTitleGroupsLite>('/artists?id=' + id)).data
}

export const createArtists = async (artists: UserCreatedArtist[]): Promise<Artist[]> => {
  return (await api.post<Artist[]>('/artists', artists)).data
}

export const searchArtistsLite = async (name: string): Promise<ArtistLite[]> => {
  return (await api.get<ArtistLite[]>(`/search/artists/lite?name=${name}`)).data
}

export const createArtistAffiliation = async (affiliations: UserCreatedAffiliatedArtist[]): Promise<AffiliatedArtistHierarchy[]> => {
  return (await api.post<AffiliatedArtistHierarchy[]>('/affiliated-artists', affiliations)).data
}

export const removeArtistAffiliations = async (affiliationIds: number[]) => {
  const params = new URLSearchParams()
  affiliationIds.forEach((id) => {
    params.append('affiliation_ids', id.toString())
  })
  return (await api.delete(`/affiliated-artists?${params.toString()}`)).data
}

export const addArtistAffiliations = async (affiliatedArtists: UserCreatedAffiliatedArtist[]) => {
  return (await api.post('/affiliated-artists', affiliatedArtists)).data
}
