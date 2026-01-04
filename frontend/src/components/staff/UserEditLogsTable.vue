<template>
  <div class="user-edit-logs-table">
    <UserEditLogsSearchForm v-if="initialForm" ref="searchFormRef" :loading :initialForm />
    <PaginatedResults
      v-if="initialForm && records"
      :totalPages
      :initialPage="initialForm.page"
      :totalItems="totalItems"
      :pageSize
      @changePage="searchFormRef.changePage($event.page)"
    >
      <DataTable :value="records" scrollable scrollHeight="70vh" size="small">
        <Column field="edited_by.username" :header="t('user_edit_log.edited_by')">
          <template #body="slotProps">
            <UsernameEnriched :user="slotProps.data.edited_by" />
          </template>
        </Column>
        <Column field="item_type" :header="t('user_edit_log.item_type')">
          <template #body="slotProps">
            {{ slotProps.data.item_type }}
          </template>
        </Column>
        <Column field="item_id" :header="t('user_edit_log.item_id')">
          <template #body="slotProps">
            <RouterLink
              v-if="getItemLink(slotProps.data.item_type, slotProps.data.item_id)"
              :to="getItemLink(slotProps.data.item_type, slotProps.data.item_id)!"
            >
              {{ slotProps.data.item_id }}
            </RouterLink>
            <span v-else>{{ slotProps.data.item_id }}</span>
          </template>
        </Column>
        <Column field="edits" :header="t('user_edit_log.edits')">
          <template #body="slotProps">
            <DiffViewer :edits="slotProps.data.edits" />
          </template>
        </Column>
        <Column field="edited_at" :header="t('user_edit_log.edited_at')">
          <template #body="slotProps">
            {{ timeAgo(slotProps.data.edited_at) }}
          </template>
        </Column>
      </DataTable>
    </PaginatedResults>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, watch, nextTick } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute, RouterLink } from 'vue-router'
import { Column, DataTable } from 'primevue'
import {
  searchUserEditChangeLogs,
  type PaginatedResultsUserEditChangeLogResultResultsInner,
  type SearchUserEditChangeLogsRequest,
  UserEditChangeLogSortByColumn,
} from '@/services/api-schema'
import PaginatedResults from '@/components/PaginatedResults.vue'
import UserEditLogsSearchForm from '@/components/staff/UserEditLogsSearchForm.vue'
import DiffViewer from '@/components/staff/DiffViewer.vue'
import type { VNodeRef } from 'vue'
import { timeAgo } from '@/services/helpers'
import UsernameEnriched from '../user/UsernameEnriched.vue'

const { t } = useI18n()
const route = useRoute()

const getItemLink = (itemType: string, itemId: number): string | null => {
  const routes: Record<string, string> = {
    artist: `/artist/${itemId}`,
    title_group: `/title-group/${itemId}`,
    series: `/series/${itemId}`,
    forum_thread: `/forum/thread/${itemId}`,
    forum_sub_category: `/forum/sub-category/${itemId}`,
    torrent_request: `/torrent-request/${itemId}`,
    wiki_article: `/wiki/article/${itemId}`,
    collage: `/collage/${itemId}`,
  }
  return routes[itemType] ?? null
}

const searchFormRef = ref<VNodeRef | null>(null)
const records = ref<PaginatedResultsUserEditChangeLogResultResultsInner[]>()
const loading = ref(false)
const initialForm = ref<SearchUserEditChangeLogsRequest | null>(null)
const totalItems = ref(0)
const pageSize = ref(20)
const totalPages = computed(() => Math.ceil(totalItems.value / pageSize.value))

const search = async (form: SearchUserEditChangeLogsRequest) => {
  loading.value = true
  const response = await searchUserEditChangeLogs(form).finally(() => {
    loading.value = false
  })
  pageSize.value = form.page_size
  totalItems.value = response.total_items
  records.value = response.results
}

const loadFormFromUrl = async () => {
  loading.value = true
  initialForm.value = null
  await nextTick()

  const form: SearchUserEditChangeLogsRequest = {
    page: route.query.page ? parseInt(route.query.page as string) : 1,
    page_size: route.query.page_size ? parseInt(route.query.page_size as string) : 20,
    user_id: route.query.user_id ? parseInt(route.query.user_id as string) : undefined,
    item_type: route.query.item_type ? (route.query.item_type as string) : undefined,
    sort_by_column:
      (route.query.sort_by_column as (typeof UserEditChangeLogSortByColumn)[keyof typeof UserEditChangeLogSortByColumn]) ??
      UserEditChangeLogSortByColumn.EditedAt,
    sort_by_direction: (route.query.sort_by_direction as 'asc' | 'desc') ?? 'desc',
  }

  initialForm.value = form
  pageSize.value = form.page_size
  search(form)
}

onMounted(() => {
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
.user-edit-logs-table {
  margin-top: 20px;
}
</style>
