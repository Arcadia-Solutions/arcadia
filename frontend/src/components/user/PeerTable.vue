<template>
  <DataTable :value="peers">
    <Column field="first_seen_at" header="First seen">
      <template #body="slotProps">
        {{ timeAgo(slotProps.data.first_seen_at) }}
      </template>
    </Column>
    <Column field="last_seen_at" header="Last seen">
      <template #body="slotProps">
        {{ timeAgo(slotProps.data.last_seen_at) }}
      </template>
    </Column>
    <Column field="ip" header="IP"></Column>
    <Column field="port" header="Port"></Column>
    <Column field="agent" header="User Agent"></Column>
    <Column field="real_downloaded" :header="t('user.real_downloaded')">
      <template #body="slotProps">
        {{ bytesToReadable(slotProps.data.real_downloaded) }}
      </template>
    </Column>
    <Column field="real_uploaded" :header="t('user.real_uploaded')">
      <template #body="slotProps">
        {{ bytesToReadable(slotProps.data.real_uploaded) }}
      </template>
    </Column>
  </DataTable>
</template>

<script setup lang="ts">
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import { bytesToReadable, timeAgo } from '@/services/helpers'
import { useI18n } from 'vue-i18n'
import type { Peer } from '@/services/api-schema'

const { t } = useI18n()

defineProps<{
  peers: Peer[]
}>()
</script>

<style></style>
