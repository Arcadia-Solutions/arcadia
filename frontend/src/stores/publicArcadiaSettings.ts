import type { PublicArcadiaSettings } from '@/services/api-schema'
import { defineStore } from 'pinia'

const initialState: PublicArcadiaSettings = {
  open_signups: false,
  global_download_factor: 100,
  global_upload_factor: 100,
  logo_subtitle: null,
  bonus_points_alias: '',
  bonus_points_decimal_places: 0,
  displayed_top_bar_stats: [],
  displayable_user_stats: [],
  torrent_request_vote_currencies: [],
}

export const usePublicArcadiaSettingsStore = defineStore('publicArcadiaSettings', {
  state: (): PublicArcadiaSettings => initialState,

  actions: {
    setSettings(settings: PublicArcadiaSettings) {
      Object.assign(this.$state, settings)
    },
    removeSettings() {
      Object.assign(this.$state, initialState)
    },
  },
})
