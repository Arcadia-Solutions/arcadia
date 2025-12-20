<template>
  <div v-if="searchResults">
    <div class="actions">
      <RouterLink v-if="userStore.permissions.includes('create_torrent_request')" to="/new-torrent-request">
        <i class="pi pi-plus" v-tooltip.top="t('torrent_request.new_request')" />
      </RouterLink>
      <i class="pi pi-user" v-tooltip.top="t('torrent_request.my_requests')" />
      <i class="pi pi-heart" v-tooltip.top="t('torrent_request.voted_requests')" />
      <i class="pi pi-bookmark" v-tooltip.top="t('torrent_request.bookmarked_requests')" />
    </div>
    <TorrentRequestSearchInputs ref="searchInputsRef" class="torrent-request-search-inputs" @search="search" :loading :initialForm />
    <ResultsPagination v-if="searchInputsRef" :page="searchInputsRef.searchForm.page" @changePage="searchInputsRef.changePage" />
    <TorrentRequestsTable :torrentRequests="searchResults" displayTitleGroup />
    <ResultsPagination v-if="searchInputsRef" :page="searchInputsRef.searchForm.page" @changePage="searchInputsRef.changePage" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import ResultsPagination from '@/components/ResultsPagination.vue'
import TorrentRequestsTable from '@/components/torrent_request/TorrentRequestsTable.vue'
import TorrentRequestSearchInputs from '@/components/torrent_request/TorrentRequestSearchInputs.vue'
import { useRoute } from 'vue-router'
import { watch } from 'vue'
import type { VNodeRef } from 'vue'
import { useI18n } from 'vue-i18n'
import { searchTorrentRequests, type SearchTorrentRequestsQuery, type TorrentRequestWithTitleGroupLite } from '@/services/api-schema'
import { useUserStore } from '@/stores/user'

const { t } = useI18n()
const route = useRoute()
const userStore = useUserStore()

const searchInputsRef = ref<VNodeRef | null>(null)

const searchResults = ref<TorrentRequestWithTitleGroupLite[]>()
const loading = ref(false)
const initialForm = ref<SearchTorrentRequestsQuery>({
  title_group_name: null,
  tags: null,
  page: 1,
  page_size: 20,
})

const search = async (form: SearchTorrentRequestsQuery) => {
  loading.value = true
  searchResults.value = await searchTorrentRequests(form).finally(() => {
    loading.value = false
  })
}

const loadSearchForm = async () => {
  initialForm.value.title_group_name = route.query.title_group_name?.toString() ?? null
  initialForm.value.tags = route.query.tags ? (Array.isArray(route.query.tags) ? (route.query.tags as string[]) : [route.query.tags.toString()]) : null

  search(initialForm.value)
}

onMounted(async () => {
  loadSearchForm()
})

watch(
  () => route.query,
  (newQuery, oldQuery) => {
    if (oldQuery !== undefined) {
      loadSearchForm()
    }
  },
  { immediate: true },
)
</script>

<style scoped>
.torrent-request-search-inputs {
  margin-bottom: 25px;
}
.actions {
  text-align: center;
  i {
    margin: 15px 10px;
    color: white;
  }
}
</style>
