<template>
  <div>
    <span v-if="titleGroup.platform">
      <RouterLink :class="{ prefix: true, bold }" :to="`/platform?name=${titleGroup.platform}`">
        {{ titleGroup.platform }}
      </RouterLink>
      -
    </span>
    <span v-if="series && series?.id && titleGroup.content_type !== 'movie' && !hideSeriesName">
      <RouterLink :class="{ prefix: true, bold }" :to="`/series/${series.id}`">{{ series.name }} </RouterLink>
      -
    </span>
    <template v-if="titleGroup.content_type !== 'tv_show'">
      <!-- if there are more than 2 artists, the backend can return a dummy artist instead of all of them, in the torrent search for example -->
      <span v-if="affiliatedArtists.length > 2 || (affiliatedArtists.length > 0 && affiliatedArtists[0].artist_id === 0)">
        <span :class="{ prefix: true, bold }">Various Artists</span>
        -
      </span>
      <span v-else-if="affiliatedArtists.length > 0">
        <template v-for="(artist, index) in affiliatedArtists" :key="artist.artist_id">
          <RouterLink :class="{ prefix: true, bold }" :to="`/artist/${artist.artist_id}`">
            {{ artist.name }}
          </RouterLink>
          <span v-if="index === 0 && affiliatedArtists.length === 2"> & </span>
        </template>
        -
      </span>
    </template>
    <RouterLink :class="{ 'title-group-name': true, bold }" v-if="nameLink" :to="`/title-group/${titleGroup.id}`">
      {{ titleGroup.name }}
    </RouterLink>
    <span :class="{ 'title-group-name': true, bold }" v-else>{{ titleGroup.name }}</span>
    <span class="year" v-if="titleGroup.original_release_date">({{ titleGroup.original_release_date.substring(0, 4) }})</span>
  </div>
</template>
<script setup lang="ts">
import type { AffiliatedArtistLite, SeriesLite, TitleGroup, TitleGroupHierarchyLite, TitleGroupLite } from '@/services/api-schema'

defineProps<{
  titleGroup: TitleGroup | TitleGroupLite | TitleGroupHierarchyLite
  series?: SeriesLite | null
  affiliatedArtists: AffiliatedArtistLite[]
  nameLink?: boolean
  bold?: boolean
  hideSeriesName?: boolean
}>()
</script>
<style scoped>
.title-group-name {
  margin-right: 5px;
}
</style>
