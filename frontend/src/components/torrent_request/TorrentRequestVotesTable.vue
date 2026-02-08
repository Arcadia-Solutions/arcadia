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
    <Column :header="publicArcadiaSettings.bonus_points_alias">
      <template #body="slotProps">
        {{ formatBp(slotProps.data.bounty_bonus_points, publicArcadiaSettings.bonus_points_decimal_places) }}
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
import { bytesToReadable, timeAgo, formatBp } from '@/services/helpers'
import { useI18n } from 'vue-i18n'
import type { TorrentRequestVoteHierarchy } from '@/services/api-schema'
import { usePublicArcadiaSettingsStore } from '@/stores/publicArcadiaSettings'

const { t } = useI18n()
const publicArcadiaSettings = usePublicArcadiaSettingsStore()

defineProps<{
  votes: TorrentRequestVoteHierarchy[]
}>()
</script>
<style scoped></style>
