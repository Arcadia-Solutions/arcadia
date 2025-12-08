import api from './api.ts'
import type { EditionGroup, UserCreatedEditionGroup } from '../api-schema/api.ts'

export const createEditionGroup = async (editionGroup: UserCreatedEditionGroup) => {
  editionGroup.additional_information = editionGroup.additional_information
    ? Object.fromEntries(Object.entries(editionGroup.additional_information).filter(([, value]) => value !== null && value !== ''))
    : {}
  editionGroup.covers = editionGroup.covers.filter((cover) => cover.trim() !== '')
  editionGroup.external_links = editionGroup.external_links.filter((link) => link.trim() !== '')
  editionGroup.distributor = editionGroup.distributor == '' ? null : editionGroup.distributor
  return (await api.post<EditionGroup>('/edition-groups', editionGroup)).data
}
