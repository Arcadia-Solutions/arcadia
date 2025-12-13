<template>
  <div class="wrapper-center actions">
    <i class="pi pi-bookmark" v-tooltip.top="t('artist.bookmarked_artists')" />
  </div>
  <ArtistsSearchForm @gotResults="gotResults" style="margin-bottom: 15px" />
  <ArtistsTable :artists />
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import ArtistsSearchForm from '@/components/artists/ArtistsSearchForm.vue'
import ArtistsTable from '@/components/artists/ArtistsTable.vue'
import type { PaginatedResultsArtistSearchResult, ArtistSearchResult } from '@/services/api-schema'

const { t } = useI18n()
const artists = ref<ArtistSearchResult[]>([])

const gotResults = (response: PaginatedResultsArtistSearchResult) => {
  artists.value = response.results
}
</script>

<style scoped>
.actions i {
  margin: 10px;
  color: white;
}
</style>
