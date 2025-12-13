<template>
  <DataTable :value="artists" size="small" id="artists-table">
    <Column style="width: 2em">
      <template #body="slotProps">
        <ImagePreview v-if="slotProps.data.pictures?.length" :imageLink="slotProps.data.pictures[0]" class="cover" />
      </template>
    </Column>
    <Column :header="t('general.name')">
      <template #body="slotProps">
        <RouterLink :to="`/artist/${slotProps.data.id}`">{{ slotProps.data.name }}</RouterLink>
      </template>
    </Column>
    <Column :header="t('artist.title_groups')" field="title_groups_amount" />
  </DataTable>
</template>

<script setup lang="ts">
import { Column, DataTable } from 'primevue'
import { RouterLink } from 'vue-router'
import { useI18n } from 'vue-i18n'
import ImagePreview from '../ImagePreview.vue'
import type { ArtistSearchResult } from '@/services/api-schema'

defineProps<{
  artists: ArtistSearchResult[]
}>()

const { t } = useI18n()
</script>

<style>
#artists-table {
  .cover img {
    width: 5em;
    border-radius: 7px;
  }
}
</style>
