import type { EditionGroupInfoLite } from '@/services/api-schema'
import { defineStore } from 'pinia'

export const useEditionGroupStore = defineStore('editionGroup', {
  state: (): EditionGroupInfoLite => {
    return {
      id: 0,
      name: '',
      distributor: null,
      source: null,
      release_date: '',
      additional_information: { type: '' },
    }
  },
})
