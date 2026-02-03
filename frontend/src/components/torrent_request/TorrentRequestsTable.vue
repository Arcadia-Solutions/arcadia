<template>
  <DataTable :value="torrentRequests" size="small" :sortField :sortOrder lazy @sort="onSort">
    <Column :header="t('general.title')" v-if="displayTitleGroup">
      <template #body="slotProps">
        <TitleGroupSlimHeader
          :titleGroup="slotProps.data.title_group"
          :series="slotProps.data.series"
          nameLink
          :affiliatedArtists="slotProps.data.affiliated_artists"
        />
      </template>
    </Column>
    <Column :header="t('torrent_request.requirement', 2)">
      <template #body="slotProps">
        <RouterLink :to="`/torrent-request/${slotProps.data.torrent_request.id}`">
          <TorrentRequestSlug :torrentRequest="slotProps.data.torrent_request" :contentType="contentType ?? slotProps.data.title_group.content_type" />
        </RouterLink>
      </template>
    </Column>
    <Column field="upload" :header="t('user.upload')" :sortable="sortable">
      <template #body="slotProps">{{ bytesToReadable(slotProps.data.bounty.upload) }}</template>
    </Column>
    <Column field="bonus_points" :header="publicArcadiaSettings.bonus_points_alias" :sortable="sortable">
      <template #body="slotProps">{{ slotProps.data.bounty.bonus_points }}</template>
    </Column>
    <Column field="voters" :header="t('torrent_request.voter', 2)" :sortable="sortable">
      <template #body="slotProps"> {{ slotProps.data.user_votes_amount }} </template>
    </Column>
    <Column field="created_at" :header="t('general.created_at')" :sortable="sortable">
      <template #body="slotProps">{{ timeAgo(slotProps.data.torrent_request.created_at) }}</template>
    </Column>
    <Column :header="t('torrent_request.filled')">
      <template #body="slotProps">
        <i v-if="slotProps.data.torrent_request.filled_by_torrent_id" class="pi pi-check" />
        <i v-else class="pi pi-times" />
      </template>
    </Column>
  </DataTable>
</template>

<script setup lang="ts">
import { Column, DataTable } from 'primevue'
import TorrentRequestSlug from './TorrentRequestSlug.vue'
import TitleGroupSlimHeader from '../title_group/TitleGroupSlimHeader.vue'
import { RouterLink } from 'vue-router'
import { bytesToReadable, timeAgo } from '@/services/helpers'
import { useI18n } from 'vue-i18n'
import type { ContentType, TorrentRequestHierarchyLite, TorrentRequestWithTitleGroupLite, TorrentRequestSearchOrderBy } from '@/services/api-schema'
import { usePublicArcadiaSettingsStore } from '@/stores/publicArcadiaSettings'
import type { DataTableSortEvent } from 'primevue/datatable'

defineProps<{
  torrentRequests: TorrentRequestHierarchyLite[] | TorrentRequestWithTitleGroupLite[]
  contentType?: ContentType
  displayTitleGroup?: boolean
  sortable?: boolean
  sortField?: TorrentRequestSearchOrderBy
  sortOrder?: number
}>()

const emit = defineEmits<{
  sort: [event: DataTableSortEvent]
}>()

const onSort = (event: DataTableSortEvent) => {
  emit('sort', event)
}

const { t } = useI18n()
const publicArcadiaSettings = usePublicArcadiaSettingsStore()
</script>
<style scoped></style>
