<template>
  <div v-if="search_results">
    <TorrentSearchInputs v-if="initialForm" ref="searchInputsRef" class="torrent-search-inputs" :loading :initialForm="initialForm" />
    <PaginatedResults
      v-if="initialForm"
      :totalPages
      :initialPage="initialForm.page"
      :totalItems="totalResults"
      :pageSize
      @changePage="searchInputsRef.changePage($event.page)"
    >
      <TitleGroupList :titleGroups="search_results" :titleGroupPreview />
    </PaginatedResults>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { searchTorrentsLite, type TorrentSearch } from '@/services/api/torrentService'
import { type TitleGroupHierarchyLite } from '@/services/api/artistService'
import TorrentSearchInputs from '@/components/torrent/TorrentSearchInputs.vue'
import TitleGroupList from '@/components/title_group/TitleGroupList.vue'
import type { titleGroupPreviewMode } from '@/components/title_group/TitleGroupList.vue'
import { useRoute } from 'vue-router'
import type { VNodeRef } from 'vue'
import PaginatedResults from '@/components/PaginatedResults.vue'
import { computed } from 'vue'
import { watch } from 'vue'
import { nextTick } from 'vue'

const route = useRoute()

const searchInputsRef = ref<VNodeRef | null>(null)

const search_results = ref<TitleGroupHierarchyLite[]>([])
const titleGroupPreview = ref<titleGroupPreviewMode>('table') // TODO: make a select button to switch from cover-only to table
const loading = ref(false)
const initialForm = ref<TorrentSearch | null>(null)
const totalResults = ref(0)
const pageSize = ref(0)
const totalPages = computed(() => Math.ceil(totalResults.value / pageSize.value))

const search = async (torrentSearch: TorrentSearch) => {
  const results = await searchTorrentsLite(torrentSearch).finally(() => {
    loading.value = false
  })
  // page.value = torrentSearch.page
  pageSize.value = torrentSearch.page_size
  totalResults.value = results.total_items
  search_results.value = results.results
}

const loadFormFromUrl = async () => {
  loading.value = true
  initialForm.value = null
  await nextTick()
  const form: TorrentSearch = {
    title_group_name: route.query.title_group_name?.toString() ?? '',
    page: route.query.page ? parseInt(route.query.page as string) : 1,
    page_size: route.query.page_size ? parseInt(route.query.page_size as string) : 10,
    torrent_created_by_id: route.query.torrent_created_by_id ? parseInt(route.query.torrent_created_by_id as string) : null,
    torrent_snatched_by_id: route.query.torrent_snatched_by_id ? parseInt(route.query.torrent_snatched_by_id as string) : null,
    torrent_staff_checked: route.query.torrent_staff_checked === 'true' ? true : null,
    torrent_reported: route.query.torrent_reported === 'true' ? true : null,
    // @ts-expect-error what is placed in this query always comes from the form, so there shouldn't be a wrong value
    order_by_column: route.query.order_by_column ? (route.query.order_by_column as string) : 'torrent_created_at',
    // @ts-expect-error what is placed in this query always comes from the form, so there shouldn't be a wrong value
    order_by_direction: route.query.order_by_direction ? (route.query.order_by_direction as string) : 'desc',
    title_group_include_empty_groups: route.query.title_group_include_empty_groups === 'true' ? true : false,
  }
  initialForm.value = form
  pageSize.value = initialForm.value.page_size
  search(initialForm.value)
}

onMounted(async () => {
  loadFormFromUrl()
})

watch(
  () => route.query,
  () => {
    loadFormFromUrl()
  },
  { deep: true },
)
</script>

<style scoped>
.torrent-search-inputs {
  margin-bottom: 25px;
}
</style>
