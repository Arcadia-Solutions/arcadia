<template>
  <div style="display: flex; justify-content: center">
    <ContentContainer class="send-invite wrapper-center">
      <div style="margin-bottom: 15px">{{ t('invitation.available_invitations') }}: {{ userStore.invitations }}</div>
      <Button :label="t('user.send_invitation')" :disabled="userStore.invitations === 0" size="small" @click="showDialog = true" />
    </ContentContainer>
  </div>
  <ContentContainer class="search-form">
    <FloatLabel>
      <InputText v-model="searchForm.receiver_username" size="small" />
      <label>{{ t('invitation.receiver_username') }}</label>
    </FloatLabel>
    <div class="wrapper-center">
      <Button :label="t('general.search')" size="small" :loading="loading" @click="updateUrl" />
    </div>
  </ContentContainer>
  <PaginatedResults
    v-if="searchResults.length > 0"
    :totalItems="totalResults"
    :pageSize="searchForm.page_size"
    :initialPage="searchForm.page"
    :totalPages="totalPages"
  >
    <DataTable
      :value="searchResults"
      size="small"
      :sortField="searchForm.order_by_column"
      :sortOrder="sortOrder"
      lazy
      @sort="onSort"
      tableStyle="table-layout: fixed"
    >
      <Column field="receiver_email" :header="t('user.email')">
        <template #body="slotProps">{{ slotProps.data.receiver_email }}</template>
      </Column>
      <Column field="receiver_username" :header="t('user.user')" sortable>
        <template #body="slotProps">
          <div v-if="slotProps.data.receiver" class="flex align-items-center gap-2">
            <img :src="slotProps.data.receiver.avatar ?? '/default_user_avatar.png'" class="avatar" />
            <UsernameEnriched :user="slotProps.data.receiver" displayAllInfo />
          </div>
        </template>
      </Column>
      <Column field="inviter_notes" :header="t('invitation.inviter_notes')" style="width: 25em !important">
        <template #body="slotProps">{{ slotProps.data.inviter_notes }}</template>
      </Column>
      <Column field="created_at" :header="t('invitation.sent_at')" sortable>
        <template #body="slotProps">{{ timeAgo(slotProps.data.created_at) }}</template>
      </Column>
      <Column field="expires_at" :header="t('invitation.expires_at')">
        <template #body="slotProps">
          <span v-if="!slotProps.data.receiver">{{ timeAgo(slotProps.data.expires_at) }}</span>
        </template>
      </Column>
    </DataTable>
  </PaginatedResults>
  <Dialog v-model:visible="showDialog" :header="t('user.send_invitation')" modal>
    <SendInvitationDialog receiver-email="" @invitation-sent="onInvitationSent" />
  </Dialog>
</template>

<script setup lang="ts">
import { onMounted, ref, computed, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter, useRoute } from 'vue-router'
import { Button, FloatLabel, InputText, DataTable, Column, Dialog } from 'primevue'
import ContentContainer from '@/components/ContentContainer.vue'
import PaginatedResults from '@/components/PaginatedResults.vue'
import UsernameEnriched from '@/components/user/UsernameEnriched.vue'
import SendInvitationDialog from '@/components/user/SendInvitationDialog.vue'
import {
  searchSentInvitations,
  type PaginatedResultsInvitationHierarchyResultsInner,
  InvitationSearchOrderByColumn,
  OrderByDirection,
} from '@/services/api-schema'
import { timeAgo } from '@/services/helpers'
import { useUserStore } from '@/stores/user'
import type { DataTableSortEvent } from 'primevue/datatable'

const { t } = useI18n()
const router = useRouter()
const route = useRoute()
const userStore = useUserStore()

interface SearchForm {
  receiver_username: string
  order_by_column: InvitationSearchOrderByColumn
  order_by_direction: OrderByDirection
  page: number
  page_size: number
}

const searchForm = ref<SearchForm>({
  receiver_username: '',
  order_by_column: InvitationSearchOrderByColumn.CreatedAt,
  order_by_direction: OrderByDirection.Desc,
  page: 1,
  page_size: 25,
})

const searchResults = ref<PaginatedResultsInvitationHierarchyResultsInner[]>([])
const totalResults = ref(0)
const loading = ref(false)
const showDialog = ref(false)
const totalPages = computed(() => Math.ceil(totalResults.value / searchForm.value.page_size))

const orderByColumnValues: string[] = Object.values(InvitationSearchOrderByColumn)
const isOrderByColumn = (value: unknown): value is InvitationSearchOrderByColumn => typeof value === 'string' && orderByColumnValues.includes(value)

const orderByDirectionValues: string[] = Object.values(OrderByDirection)
const isOrderByDirection = (value: unknown): value is OrderByDirection => typeof value === 'string' && orderByDirectionValues.includes(value)

const sortOrder = computed(() => (searchForm.value.order_by_direction === OrderByDirection.Asc ? 1 : -1))

const onSort = (event: DataTableSortEvent) => {
  if (typeof event.sortField === 'string' && isOrderByColumn(event.sortField)) {
    searchForm.value.order_by_column = event.sortField
    searchForm.value.order_by_direction = event.sortOrder === 1 ? OrderByDirection.Asc : OrderByDirection.Desc
    updateUrl()
  }
}

const updateUrl = () => {
  router.push({
    query: {
      receiver_username: searchForm.value.receiver_username || undefined,
      order_by_column: searchForm.value.order_by_column,
      order_by_direction: searchForm.value.order_by_direction,
      page: searchForm.value.page.toString(),
    },
  })
}

const fetchSearchResults = () => {
  const orderByColumn = route.query.order_by_column
  const orderByDirection = route.query.order_by_direction

  searchForm.value.page = route.query.page ? parseInt(route.query.page.toString()) : 1
  searchForm.value.receiver_username = route.query.receiver_username?.toString() ?? ''
  searchForm.value.order_by_column = isOrderByColumn(orderByColumn) ? orderByColumn : InvitationSearchOrderByColumn.CreatedAt
  searchForm.value.order_by_direction = isOrderByDirection(orderByDirection) ? orderByDirection : OrderByDirection.Desc

  loading.value = true
  searchSentInvitations({
    receiver_username: searchForm.value.receiver_username || undefined,
    order_by_column: searchForm.value.order_by_column,
    order_by_direction: searchForm.value.order_by_direction,
    page: searchForm.value.page,
    page_size: searchForm.value.page_size,
  })
    .then((response) => {
      searchResults.value = response.results
      totalResults.value = response.total_items
    })
    .finally(() => {
      loading.value = false
    })
}

const onInvitationSent = () => {
  userStore.invitations--
  fetchSearchResults()
}

onMounted(() => fetchSearchResults())

watch(
  () => route.query,
  () => fetchSearchResults(),
  { deep: true },
)
</script>

<style scoped>
.send-invite {
  margin-bottom: 15px;
  width: 20em;
}
.search-form {
  margin-bottom: 15px;
  display: flex;
  gap: 15px;
  flex-wrap: wrap;
  align-items: center;
}
.avatar {
  width: 50px;
  border-radius: 7px;
  object-fit: cover;
}
:deep(td) {
  word-wrap: break-word;
  overflow-wrap: break-word;
}
</style>
