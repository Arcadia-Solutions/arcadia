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
      <ArtistsTable :artists />
    </PaginatedResults>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch, nextTick } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute } from 'vue-router'
import type { VNodeRef } from 'vue'
import ArtistsSearchForm from '@/components/artists/ArtistsSearchForm.vue'
import ArtistsTable from '@/components/artists/ArtistsTable.vue'
import PaginatedResults from '@/components/PaginatedResults.vue'
import { searchArtists, type SearchArtistsRequest, type ArtistSearchResult } from '@/services/api-schema'

const { t } = useI18n()
const route = useRoute()

const searchFormRef = ref<VNodeRef | null>(null)

const artists = ref<ArtistSearchResult[]>([])
const loading = ref(false)
const initialForm = ref<SearchArtistsRequest | null>(null)
const totalResults = ref(0)
const pageSize = ref(0)
const totalPages = computed(() => Math.ceil(totalResults.value / pageSize.value))

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
