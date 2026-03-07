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
  emails_enabled: false,
  display_image_host_drag_and_drop: false,
  irc_enabled: false,
  irc_webchat_enabled: false,
  irc_webchat_default_channels: [],
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
