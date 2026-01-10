<template>
  <PaginatedResults
    v-if="paginatedResults && paginatedResults.total_items > 0"
    :totalItems="paginatedResults.total_items"
    :pageSize="filters.page_size!"
    :totalPages="Math.ceil(paginatedResults.total_items / filters.page_size!)"
    :initialPage="filters.page!"
  >
    <UserApplicationComponent
      v-for="userApplication in paginatedResults.results"
      :key="userApplication.id"
      :userApplication
      @applicationUpdated="applicationUpdated"
    />
  </PaginatedResults>
  <div v-else-if="!loading && paginatedResults?.total_items === 0">
    {{ t('staff.user_application.no_applications') }}
  </div>
</template>

<script setup lang="ts">
import {
  getUserApplications,
  type GetUserApplicationsQuery,
  type PaginatedResultsUserApplicationHierarchy,
  type PaginatedResultsUserApplicationHierarchyResultsInner,
} from '@/services/api-schema'
import UserApplicationComponent from './UserApplication.vue'
import PaginatedResults from '../PaginatedResults.vue'
import { ref, onMounted, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute } from 'vue-router'

const { t } = useI18n()
const route = useRoute()

const loading = ref(true)

const filters = ref<GetUserApplicationsQuery>({
  page: 1,
  page_size: 50,
  status: null,
})

const paginatedResults = ref<PaginatedResultsUserApplicationHierarchy | null>(null)

const fetchApplications = async () => {
  loading.value = true
  paginatedResults.value = await getUserApplications(filters.value).finally(() => (loading.value = false))
}

const loadFiltersFromUrl = () => {
  filters.value.page = route.query.page ? parseInt(route.query.page as string) : 1
}

const applicationUpdated = (app: PaginatedResultsUserApplicationHierarchyResultsInner) => {
  if (!paginatedResults.value) return
  paginatedResults.value.results = paginatedResults.value.results.some((a) => a.id === app.id)
    ? paginatedResults.value.results.map((a) => (a.id === app.id ? app : a))
    : [...paginatedResults.value.results, app]
}

watch(
  () => route.query,
  () => {
    loadFiltersFromUrl()
    fetchApplications()
  },
  { deep: true },
)

onMounted(() => {
  loadFiltersFromUrl()
  fetchApplications()
})
</script>

<style scoped></style>
