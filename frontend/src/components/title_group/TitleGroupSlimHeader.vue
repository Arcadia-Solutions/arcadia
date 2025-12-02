<template>
  <div>
    <span v-if="series && series?.id">
      <RouterLink class="prefix" :to="`/series/${series.id}`">{{ series.name }} </RouterLink>
      -
    </span>
    <span v-if="titleGroup.platform">
      <RouterLink class="prefix" :to="`/platform?name=${titleGroup.platform}`">
        {{ titleGroup.platform }}
      </RouterLink>
      -
    </span>
    <RouterLink class="title-group-name" v-if="nameLink" :to="nameLink">
      {{ titleGroup.name }}
    </RouterLink>
    <span class="title-group-name" v-else>{{ titleGroup.name }}</span>
    <span class="year">({{ titleGroup.original_release_date.substring(0, 4) }})</span>
  </div>
</template>
<script setup lang="ts">
import type { SeriesLite } from '@/services/api/seriesService'
import type { TitleGroup, TitleGroupLite } from '@/services/api/titleGroupService'

defineProps<{
  titleGroup: TitleGroup | TitleGroupLite
  series?: SeriesLite
  // affiliatedArtists?: AffiliatedArtistHierarchy
  nameLink?: string
}>()
</script>
<style scoped>
.title-group-name {
  font-weight: bold;
  margin-right: 5px;
}
.prefix {
  font-weight: bold;
}
</style>
