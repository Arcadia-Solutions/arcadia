<template>
  <PaginatedResults
    v-if="paginatedResults && paginatedResults.total_items > 0"
    :totalItems="paginatedResults.total_items"
    :pageSize="filters.page_size!"
    :totalPages="Math.ceil(paginatedResults.total_items / filters.page_size!)"
    :initialPage="filters.page!"
    @changePage="onChangePage"
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
import { getUserApplications, type GetUserApplicationsQuery, type UserApplication, type PaginatedResultsUserApplication } from '@/services/api-schema'
import UserApplicationComponent from './UserApplication.vue'
import PaginatedResults from '../PaginatedResults.vue'
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const loading = ref(true)

const filters = ref<GetUserApplicationsQuery>({
  page: 1,
  page_size: 50,
  status: null,
})

const paginatedResults = ref<PaginatedResultsUserApplication | null>(null)

const fetchApplications = async () => {
  loading.value = true
  paginatedResults.value = await getUserApplications(filters.value).finally(() => (loading.value = false))
}

const applicationUpdated = (app: UserApplication) => {
  if (!paginatedResults.value) return
  paginatedResults.value.results = paginatedResults.value.results.some((a) => a.id === app.id)
    ? paginatedResults.value.results.map((a) => (a.id === app.id ? app : a))
    : [...paginatedResults.value.results, app]
}

const onChangePage = ({ page }: { page: number }) => {
  filters.value.page = page
  fetchApplications()
}

onMounted(() => {
  fetchApplications()
})
</script>

<style scoped></style>
