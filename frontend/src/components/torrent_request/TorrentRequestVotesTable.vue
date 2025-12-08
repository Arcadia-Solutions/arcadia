<template>
  <DataTable :value="votes" size="small">
    <Column :header="t('torrent_request.voter')">
      <template #body="slotProps">
        <UsernameEnriched :user="slotProps.data.created_by" />
      </template>
    </Column>
    <Column :header="t('user.upload')">
      <template #body="slotProps">
        {{ bytesToReadable(slotProps.data.bounty_upload) }}
      </template>
    </Column>
    <Column :header="t('user.bonus_points')">
      <template #body="slotProps">
        {{ slotProps.data.bounty_bonus_points }}
      </template>
    </Column>
    <Column :header="t('torrent_request.voted_at')">
      <template #body="slotProps">
        {{ timeAgo(slotProps.data.created_at) }}
      </template>
    </Column>
  </DataTable>
</template>

<script setup lang="ts">
import { Column, DataTable } from 'primevue'
import UsernameEnriched from '../user/UsernameEnriched.vue'
import { bytesToReadable, timeAgo } from '@/services/helpers'
import { useI18n } from 'vue-i18n'
import type { TorrentRequestVoteHierarchy } from '@/services/api-schema'

const { t } = useI18n()

defineProps<{
  votes: TorrentRequestVoteHierarchy[]
}>()
</script>
<style scoped></style>
