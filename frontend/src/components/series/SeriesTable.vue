<template>
  <DataTable :value="series" size="small" id="series-table">
    <Column style="width: 2em">
      <template #body="slotProps">
        <ImagePreview :imageLink="slotProps.data.covers[0]" class="cover" />
      </template>
    </Column>
    <Column :header="t('general.name')">
      <template #body="slotProps">
        <RouterLink :to="`/series/${slotProps.data.id}`">{{ slotProps.data.name }}</RouterLink>
      </template>
    </Column>
    <Column :header="t('series.entry', 2)" field="title_groups_amount" />
    <Column :header="t('general.tags')">
      <template #body="slotProps">
        {{ slotProps.data.tags.join(', ') }}
      </template>
    </Column>
  </DataTable>
</template>

<script setup lang="ts">
import { Column, DataTable } from 'primevue'
import { RouterLink } from 'vue-router'
import { useI18n } from 'vue-i18n'
import ImagePreview from '../ImagePreview.vue'
import type { SeriesSearchResult } from '@/services/api-schema'

defineProps<{
  series: SeriesSearchResult[]
}>()

const { t } = useI18n()
</script>
<style>
#series-table {
  .cover {
    img {
      width: 5em;
      border-radius: 7px;
    }
  }
}
</style>
