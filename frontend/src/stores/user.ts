import type { User } from '@/services/api/userService'
import { defineStore } from 'pinia'

const initialState: User = {
  artist_comments: 0,
  avatar: null,
  average_seeding_time: 0,
  bonus_points: 0,
  class: 'newbie',
  collages_started: 0,
  created_at: '',
  description: '',
  downloaded: 0,
  email: '',
  forum_posts: 0,
  forum_threads: 0,
  freeleech_tokens: 0,
  id: 0,
  invitations: 0,
  invited: 0,
  last_seen: '',
  leeching: 0,
  password_hash: '',
  ratio: 0.0,
  real_downloaded: 0,
  real_uploaded: 0,
  registered_from_ip: '',
  request_comments: 0,
  requests_filled: 0,
  requests_voted: 0,
  required_ratio: 0,
  seeding: 0,
  seeding_size: 0,
  snatched: 0,
  torrent_comments: 0,
  uploaded: 0,
  username: '',
  warned: false,
  banned: false,
  staff_note: '',
  passkey: '',
  css_sheet_name: '',
}

export const useUserStore = defineStore('user', {
  state: (): User => initialState,

  actions: {
    setUser(user: User) {
      Object.assign(this.$state, user)
    },
    removeUser() {
      Object.assign(this.$state, initialState)
    },
  },
})
