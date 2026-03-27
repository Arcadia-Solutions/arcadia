<template>
  <a target="_blank" :href="link"><img :src="logoSrc" @error="onImgError" /></a>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  link: string
}>()

const knownLogos: Record<string, string> = {
  'wikipedia.org': 'wikipedia.png',
  'openlibrary.org': 'open-library.svg',
  'musicbrainz.org': 'musicbrainz.svg',
  'discogs.com': 'discogs.svg',
  'imdb.com': 'imdb.svg',
  'themoviedb.org': 'tmdb.svg',
  'thetvdb.com': 'tvdb.svg',
  'store.steampowered.com': 'steam.svg',
  'comicvine.gamespot.com': 'comic_vine.svg',
  'redacted.sh': 'red.ico',
  'orpheus.network': 'ops.ico',
  'passthepopcorn.me': 'ptp.ico',
  'anthelion.me': 'ant.ico',
  'secret-cinema.pw': 'sc.ico',
  'beyond-hd.me': 'bhd.ico',
}

const logoSrc = computed(() => {
  for (const [domain, logo] of Object.entries(knownLogos)) {
    if (props.link.includes(domain)) return '/logos/external_links/' + logo
  }
  try {
    const { origin } = new URL(props.link)
    return `${origin}/favicon.ico`
  } catch {
    return '/logos/external_links/default.svg'
  }
})

const onImgError = (e: Event) => {
  ;(e.target as HTMLImageElement).src = '/logos/external_links/default.svg'
}
</script>
<style scoped>
img {
  width: 30px;
}
a {
  background-color: transparent;
}
</style>
