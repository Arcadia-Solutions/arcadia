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
      release_date_only_year_known: false,
      additional_information: { type: '' },
    }
  },
})
