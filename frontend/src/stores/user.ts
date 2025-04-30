import type { User } from '@/services/api/userService'
import { defineStore } from 'pinia'

export const useUserStore = defineStore('user', {
  state: () => ({
    artist_comments: 0,
    avatar: null,
    average_seeding_time: 0,
    bonus_points: 0,
    class: '',
    collages_started: 0,
    created_at: '',
    description: '',
    downloaded: 0,
    email: '',
    forum_posts: 0,
    forum_threads: 0,
    freeleech_tokens: 0,
    group_comments: 0,
    id: 0,
    invitations: 0,
    invited: 0,
    last_seen: '',
    leeching: 0,
    passkey_lower: 0,
    passkey_upper: 0,
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
    settings: {},
    snatched: 0,
    torrent_comments: 0,
    uploaded: 0,
    username: '',
  }),

  actions: {
    setUser(user: User) {
      Object.assign(this.$state, user)
    },
  },
})
