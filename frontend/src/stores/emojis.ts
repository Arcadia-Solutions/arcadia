import { defineStore } from 'pinia'
import { getEmojis, type Emoji } from '@/services/api-schema'

// Deliberately outside the store state: Pinia would hand back a reactive proxy of the promise,
// and calling then() on that proxy instead of the promise itself throws.
let inFlightRequest: Promise<void> | null = null

/**
 * The emoji catalogue, fetched at most once per session and only when a user opens the emoji
 * picker. Displaying existing reactions never needs it: the reactions returned with the posts
 * already carry what is needed to render them.
 */
export const useEmojisStore = defineStore('emojis', {
  state: () => ({
    emojis: [] as Emoji[],
    loaded: false,
  }),
  actions: {
    loadOnce(): Promise<void> {
      if (this.loaded) {
        return Promise.resolve()
      }

      // Callers arriving while a request is in flight await that same request. Returning early
      // instead would resolve them with an empty catalogue, and their picker would claim no
      // emoji is configured.
      inFlightRequest ??= getEmojis()
        .then((emojis) => {
          this.emojis = emojis
          this.loaded = true
        })
        .finally(() => {
          // Cleared on failure too, so the next picker opened retries.
          inFlightRequest = null
        })

      return inFlightRequest
    },
    /** Drops the cached catalogue, so the next picker opened fetches it again. */
    invalidate() {
      inFlightRequest = null
      this.$reset()
    },
  },
})
