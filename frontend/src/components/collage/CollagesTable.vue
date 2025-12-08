<template>
  <DataTable :value="collages" size="small">
    <Column :header="t('collage.category')" field="category" />
    <Column :header="t('general.name')">
      <template #body="slotProps">
        <RouterLink :to="`/collage/${slotProps.data.id}`">{{ slotProps.data.name }}</RouterLink>
      </template>
    </Column>
    <Column v-if="showCollageType" :header="t('collage.collage_type')" field="collage_type" />
    <Column :header="t('collage.entry', 2)" field="entries_amount" />
    <Column v-if="showTags" :header="t('general.tags')">
      <template #body="slotProps">
        {{ slotProps.data.tags.join(', ') }}
      </template>
    </Column>
    <Column :header="t('general.started_by')">
      <template #body="slotProps">
        <UsernameEnriched :user="slotProps.data.created_by" />
      </template>
    </Column>
    <Column :header="t('general.created_at')">
      <template #body="slotProps">
        {{ timeAgo(slotProps.data.created_at) }}
      </template>
    </Column>
    <Column :header="t('collage.last_entry_at')">
      <template #body="slotProps">
        {{ slotProps.data.last_entry_at ? timeAgo(slotProps.data.last_entry_at) : null }}
      </template>
    </Column>
  </DataTable>
</template>

<script setup lang="ts">
import { Column, DataTable } from 'primevue'
import { RouterLink } from 'vue-router'
import { useI18n } from 'vue-i18n'
import UsernameEnriched from '../user/UsernameEnriched.vue'
import { timeAgo } from '@/services/helpers'
import type { CollageSearchResult } from '@/services/api-schema'

defineProps<{
  collages: CollageSearchResult[]
  showCollageType?: boolean
  showTags?: boolean
}>()

const { t } = useI18n()
</script>
<style scoped></style>
