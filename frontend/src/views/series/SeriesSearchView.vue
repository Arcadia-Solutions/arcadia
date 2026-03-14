<template>
  <div class="wrapper-center actions">
    <RouterLink to="/new-series">
      <i class="pi pi-plus" v-tooltip.top="t('series.new_series')" />
    </RouterLink>
    <i class="pi pi-bookmark" v-tooltip.top="t('series.bookmarked_series')" />
  </div>
  <SeriesSearchForm ref="searchFormRef" v-model:loading="loading" @search="fetchSeries(1)" style="margin-bottom: 15px" />
  <PaginatedResults
    v-if="searchResults.length > 0"
    :totalItems="totalResults"
    :pageSize="pageSize"
    :initialPage="page"
    :totalPages="totalPages"
    @changePage="onPageChange"
  >
    <SeriesTable :series="searchResults" :sortField="orderByColumn" :sortOrder="sortOrder" @sort="onSort" />
  </PaginatedResults>
</template>
<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import SeriesTable from '@/components/series/SeriesTable.vue'
import SeriesSearchForm from '@/components/series/SeriesSearchForm.vue'
import PaginatedResults from '@/components/PaginatedResults.vue'
import { searchSeries, type SeriesSearchResult, SeriesSearchOrderByColumn, OrderByDirection } from '@/services/api-schema'
import type { DataTableSortEvent } from 'primevue/datatable'

const { t } = useI18n()

const searchFormRef = ref<InstanceType<typeof SeriesSearchForm>>()
const searchResults = ref<SeriesSearchResult[]>([])
const totalResults = ref(0)
const loading = ref(false)
const page = ref(1)
const pageSize = 50
const orderByColumn = ref<SeriesSearchOrderByColumn>(SeriesSearchOrderByColumn.Name)
const orderByDirection = ref<OrderByDirection>(OrderByDirection.Asc)
const totalPages = computed(() => Math.ceil(totalResults.value / pageSize))
const sortOrder = computed(() => (orderByDirection.value === OrderByDirection.Asc ? 1 : -1))

const fieldToColumn: Record<string, SeriesSearchOrderByColumn> = {
  name: SeriesSearchOrderByColumn.Name,
  title_groups_amount: SeriesSearchOrderByColumn.TitleGroupsAmount,
}

const fetchSeries = (newPage: number) => {
  page.value = newPage
  loading.value = true
  searchSeries({
    name: searchFormRef.value?.name ?? '',
    order_by_column: orderByColumn.value,
    order_by_direction: orderByDirection.value,
    page: newPage,
    page_size: pageSize,
  })
    .then((response) => {
      searchResults.value = response.results
      totalResults.value = response.total_items
    })
    .finally(() => {
      loading.value = false
    })
}

const onSort = (event: DataTableSortEvent) => {
  const column = typeof event.sortField === 'string' ? fieldToColumn[event.sortField] : undefined
  if (column) {
    orderByColumn.value = column
    orderByDirection.value = event.sortOrder === 1 ? OrderByDirection.Asc : OrderByDirection.Desc
    fetchSeries(page.value)
  }
}

const onPageChange = (pagination: { page: number }) => {
  fetchSeries(pagination.page)
}

onMounted(() => fetchSeries(1))
</script>
<style scoped>
.actions {
  i {
    margin: 10px;
    color: white;
  }
}
</style>
