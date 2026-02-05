<template>
  <div v-if="initialForm">
    <div class="wrapper-center actions">
      <i class="pi pi-bookmark" v-tooltip.top="t('artist.bookmarked_artists')" />
    </div>
    <ArtistsSearchForm ref="searchFormRef" :initialForm :loading style="margin-bottom: 15px" />
    <PaginatedResults
      v-if="initialForm"
      :totalPages
      :initialPage="initialForm.page"
      :totalItems="totalResults"
      :pageSize
      @changePage="searchFormRef.changePage($event.page)"
    >
      <ArtistsTable :artists :sortField="initialForm.order_by_column" :sortOrder @sort="onSort" />
    </PaginatedResults>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch, nextTick } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute, useRouter } from 'vue-router'
import type { VNodeRef } from 'vue'
import ArtistsSearchForm from '@/components/artists/ArtistsSearchForm.vue'
import ArtistsTable from '@/components/artists/ArtistsTable.vue'
import PaginatedResults from '@/components/PaginatedResults.vue'
import { searchArtists, type SearchArtistsRequest, type ArtistSearchResult, type ArtistSearchOrderByColumn, OrderByDirection } from '@/services/api-schema'
import type { DataTableSortEvent } from 'primevue/datatable'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()

const searchFormRef = ref<VNodeRef | null>(null)

const artists = ref<ArtistSearchResult[]>([])
const loading = ref(false)
const initialForm = ref<SearchArtistsRequest | null>(null)
const totalResults = ref(0)
const pageSize = ref(0)
const totalPages = computed(() => Math.ceil(totalResults.value / pageSize.value))
const sortOrder = computed(() => (initialForm.value?.order_by_direction === OrderByDirection.Asc ? 1 : -1))

const onSort = (event: DataTableSortEvent) => {
  router.push({
    query: {
      ...route.query,
      order_by_column: event.sortField as string,
      order_by_direction: event.sortOrder === 1 ? OrderByDirection.Asc : OrderByDirection.Desc,
    },
  })
}

const search = async (form: SearchArtistsRequest) => {
  loading.value = true
  const results = await searchArtists(form).finally(() => {
    loading.value = false
  })
  pageSize.value = form.page_size
  totalResults.value = results.total_items
  artists.value = results.results
}

const loadFormFromUrl = async () => {
  loading.value = true
  initialForm.value = null
  await nextTick()
  const form: SearchArtistsRequest = {
    name: route.query.name?.toString() ?? '',
    page: route.query.page ? parseInt(route.query.page as string) : 1,
    page_size: route.query.page_size ? parseInt(route.query.page_size as string) : 50,
    order_by_column: (route.query.order_by_column as ArtistSearchOrderByColumn) ?? 'name',
    order_by_direction: (route.query.order_by_direction as OrderByDirection) ?? 'asc',
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
.actions i {
  margin: 10px;
  color: white;
}
</style>
