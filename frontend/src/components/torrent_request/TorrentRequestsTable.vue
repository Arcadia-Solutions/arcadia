<template>
  <DataTable :value="torrentRequests" size="small">
    <Column :header="t('general.title')" v-if="displayTitleGroup">
      <template #body="slotProps">
        <TitleGroupSlimHeader
          :titleGroup="slotProps.data.title_group"
          :series="slotProps.data.series"
          :nameLink="`/title-group/${slotProps.data.title_group.id}`"
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
    <Column :header="t('user.upload')">
      <template #body="slotProps">{{ bytesToReadable(slotProps.data.bounty.upload) }}</template>
    </Column>
    <Column :header="t('user.bonus_points')">
      <template #body="slotProps">{{ slotProps.data.bounty.bonus_points }}</template>
    </Column>
    <Column :header="t('torrent_request.voter', 2)">
      <template #body="slotProps"> {{ slotProps.data.user_votes_amount }} </template>
    </Column>
  </DataTable>
</template>

<script setup lang="ts">
import { Column, DataTable } from 'primevue'
import TorrentRequestSlug from './TorrentRequestSlug.vue'
import TitleGroupSlimHeader from '../title_group/TitleGroupSlimHeader.vue'
import { RouterLink } from 'vue-router'
import { bytesToReadable } from '@/services/helpers'
import { useI18n } from 'vue-i18n'
import type { ContentType, TorrentRequestHierarchyLite, TorrentRequestWithTitleGroupLite } from '@/services/api-schema'

defineProps<{
  torrentRequests: TorrentRequestHierarchyLite[] | TorrentRequestWithTitleGroupLite[]
  contentType?: ContentType
  displayTitleGroup?: boolean
}>()

const { t } = useI18n()
</script>
<style scoped></style>
