<template>
  <DataTable :value="artists" size="small" id="artists-table" lazy :sortField :sortOrder @sort="onSort">
    <Column style="width: 2em">
      <template #body="slotProps">
        <ImagePreview v-if="slotProps.data.pictures?.length" :imageLink="slotProps.data.pictures[0]" class="cover" />
      </template>
    </Column>
    <Column :header="t('general.name')" field="name" sortable>
      <template #body="slotProps">
        <RouterLink :to="`/artist/${slotProps.data.id}`">{{ slotProps.data.name }}</RouterLink>
      </template>
    </Column>
    <Column :header="t('artist.title_groups')" field="title_groups_amount" sortable />
    <Column :header="t('general.created_at')" field="created_at" sortable>
      <template #body="slotProps">
        {{ timeAgo(slotProps.data.created_at) }}
      </template>
    </Column>
  </DataTable>
</template>

<script setup lang="ts">
import { Column, DataTable } from 'primevue'
import { RouterLink } from 'vue-router'
import { useI18n } from 'vue-i18n'
import ImagePreview from '../ImagePreview.vue'
import type { ArtistSearchResult, ArtistSearchOrderByColumn } from '@/services/api-schema'
import { timeAgo } from '@/services/helpers'
import type { DataTableSortEvent } from 'primevue/datatable'

defineProps<{
  artists: ArtistSearchResult[]
  sortField: ArtistSearchOrderByColumn
  sortOrder: 1 | -1
}>()

const emit = defineEmits<{
  sort: [event: DataTableSortEvent]
}>()

const onSort = (event: DataTableSortEvent) => {
  emit('sort', event)
}

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
