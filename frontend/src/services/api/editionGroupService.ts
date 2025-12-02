import type { components } from '@/api-schema/schema'
import api from './api.ts'

export type UserCreatedEditionGroup = components['schemas']['UserCreatedEditionGroup']

export type EditionGroup = components['schemas']['EditionGroup']

export type EditionGroupInfoLite = components['schemas']['EditionGroupInfoLite']

export type EditionGroupHierarchyLite = components['schemas']['EditionGroupHierarchyLite']

export type EditionGroupHierarchy = components['schemas']['EditionGroupHierarchy']

export type Source = components['schemas']['Source']

export const createEditionGroup = async (editionGroup: UserCreatedEditionGroup) => {
  editionGroup.additional_information = editionGroup.additional_information
    ? Object.fromEntries(Object.entries(editionGroup.additional_information).filter(([, value]) => value !== null && value !== ''))
    : {}
  editionGroup.covers = editionGroup.covers.filter((cover) => cover.trim() !== '')
  editionGroup.external_links = editionGroup.external_links.filter((link) => link.trim() !== '')
  editionGroup.distributor = editionGroup.distributor == '' ? null : editionGroup.distributor
  return (await api.post<EditionGroup>('/edition-groups', editionGroup)).data
}
