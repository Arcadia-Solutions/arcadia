<template>
  <ProgressSpinner v-if="loading" />
  <div v-else-if="peers.length === 0">{{ t('torrent.no_peers') }}</div>
  <DataTable v-else :value="peers">
    <Column field="user" :header="t('user.user')">
      <template #body="slotProps">
        <UsernameEnriched :user="slotProps.data.user" />
      </template>
    </Column>
    <Column field="seeder" :header="t('torrent.status')">
      <template #body="slotProps">
        {{ slotProps.data.seeder ? t('torrent.seeding') : t('torrent.leeching') }}
      </template>
    </Column>
    <Column field="uploaded" :header="t('torrent.uploaded_session')">
      <template #body="slotProps">
        {{ bytesToReadable(slotProps.data.uploaded) }}
      </template>
    </Column>
    <Column field="downloaded" :header="t('torrent.downloaded_session')">
      <template #body="slotProps">
        {{ bytesToReadable(slotProps.data.downloaded) }}
      </template>
    </Column>
    <Column field="agent" :header="t('torrent.user_agent')"></Column>
    <Column field="ip" :header="t('torrent.ip')">
      <template #body="slotProps">
        {{ slotProps.data.ip ?? '-' }}
      </template>
    </Column>
    <Column field="port" :header="t('torrent.port')">
      <template #body="slotProps">
        {{ slotProps.data.port ?? '-' }}
      </template>
    </Column>
  </DataTable>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import ProgressSpinner from 'primevue/progressspinner'
import { bytesToReadable } from '@/services/helpers'
import { useI18n } from 'vue-i18n'
import { getTorrentPeers, type PublicPeer } from '@/services/api-schema'
import UsernameEnriched from '@/components/user/UsernameEnriched.vue'

const props = defineProps<{
  torrentId: number
}>()

const { t } = useI18n()
const loading = ref(true)
const peers = ref<PublicPeer[]>([])

onMounted(() => {
  getTorrentPeers(props.torrentId)
    .then((result) => {
      peers.value = result
    })
    .finally(() => {
      loading.value = false
    })
})
</script>
