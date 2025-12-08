<template>
  <div class="wrapper-center actions">
    <RouterLink to="/new-series">
      <i class="pi pi-plus" v-tooltip.top="t('series.new_series')" />
    </RouterLink>
    <i class="pi pi-bookmark" v-tooltip.top="t('series.bookmarked_series')" />
  </div>
  <SeriesSearchForm @gotResults="gotResults" style="margin-bottom: 15px" />
  <SeriesTable :series />
</template>
<script setup lang="ts">
import { ref } from 'vue'
import SeriesTable from '@/components/series/SeriesTable.vue'
import { useI18n } from 'vue-i18n'
import SeriesSearchForm from '@/components/series/SeriesSearchForm.vue'
import type { SeriesSearchResponse, SeriesSearchResult } from '@/services/api-schema'

const { t } = useI18n()

const series = ref<SeriesSearchResult[]>([])

const gotResults = (seriesSearchResponse: SeriesSearchResponse) => {
  series.value = seriesSearchResponse.results
}
</script>
<style scoped>
#series-view {
  display: flex;
}
.actions {
  i {
    margin: 10px;
    color: white;
  }
}
.main-content {
  width: 80%;
  margin-right: 10px;
}
</style>
