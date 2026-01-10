<template>
  <ContentContainer class="search-form">
    <FloatLabel>
      <InputText v-model="searchForm.username" size="small" />
      <label>{{ t('user.username') }}</label>
    </FloatLabel>
    <div class="line">
      <FloatLabel>
        <Select v-model="searchForm.order_by" :options="orderByOptions" optionLabel="label" optionValue="value" size="small" />
        <label>{{ t('general.order_by') }}</label>
      </FloatLabel>
      <FloatLabel>
        <Select v-model="searchForm.order_by_direction" :options="orderByDirectionOptions" optionLabel="label" optionValue="value" size="small" />
        <label>{{ t('general.sort_by') }}</label>
      </FloatLabel>
    </div>
    <div class="wrapper-center">
      <Button :label="t('general.search')" size="small" @click="updateUrl" />
    </div>
  </ContentContainer>
  <PaginatedResults
    v-if="searchResults.length > 0"
    :totalItems="totalResults"
    :pageSize="searchForm.page_size"
    :initialPage="searchForm.page"
    :totalPages="totalPages"
  >
    <DataTable :value="searchResults" size="small" :sortField="searchForm.order_by" :sortOrder="sortOrder" @sort="onSort">
      <Column style="width: 50px">
        <template #body="slotProps">
          <img :src="slotProps.data.avatar ?? '/default_user_avatar.png'" class="avatar" />
        </template>
      </Column>
      <Column field="username" :header="t('user.username')" sortable>
        <template #body="slotProps">
          <UsernameEnriched :user="slotProps.data" />
        </template>
      </Column>
      <Column field="class_name" :header="t('user.class')" />
      <Column field="created_at" :header="t('user.joined_at')" sortable>
        <template #body="slotProps">{{ timeAgo(slotProps.data.created_at) }}</template>
      </Column>
      <Column field="uploaded" :header="t('general.uploaded')" sortable>
        <template #body="slotProps">{{ bytesToReadable(slotProps.data.uploaded) }}</template>
      </Column>
      <Column field="downloaded" :header="t('general.downloaded')" sortable>
        <template #body="slotProps">{{ bytesToReadable(slotProps.data.downloaded) }}</template>
      </Column>
      <Column field="torrents" :header="t('statistics.torrents')" sortable />
      <Column field="title_groups" :header="t('artist.title_groups')" sortable />
      <Column field="title_group_comments" :header="t('community.title_group_comments')" sortable />
      <Column field="forum_posts" :header="t('community.forum_posts')" sortable />
      <Column field="forum_threads" :header="t('community.forum_threads')" sortable />
    </DataTable>
  </PaginatedResults>
</template>

<script setup lang="ts">
import { onMounted, ref, computed, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter, useRoute } from 'vue-router'
import { Button, FloatLabel, InputText, Select, DataTable, Column } from 'primevue'
import ContentContainer from '@/components/ContentContainer.vue'
import PaginatedResults from '@/components/PaginatedResults.vue'
import UsernameEnriched from '@/components/user/UsernameEnriched.vue'
import { searchUsers, type UserSearchResult, UserSearchOrderBy, OrderByDirection } from '@/services/api-schema'
import { timeAgo, bytesToReadable, getOrderByDirectionOptions } from '@/services/helpers'
import type { DataTableSortEvent } from 'primevue/datatable'

const { t } = useI18n()
const router = useRouter()
const route = useRoute()

interface SearchForm {
  username: string
  order_by: UserSearchOrderBy
  order_by_direction: OrderByDirection
  page: number
  page_size: number
}

const searchForm = ref<SearchForm>({
  username: '',
  order_by: UserSearchOrderBy.CreatedAt,
  order_by_direction: OrderByDirection.Asc,
  page: 1,
  page_size: 25,
})

const searchResults = ref<UserSearchResult[]>([])
const totalResults = ref(0)
const totalPages = computed(() => Math.ceil(totalResults.value / searchForm.value.page_size))

const orderByOptions = [
  { label: t('user.username'), value: UserSearchOrderBy.Username },
  { label: t('user.joined_at'), value: UserSearchOrderBy.CreatedAt },
  { label: t('general.uploaded'), value: UserSearchOrderBy.Uploaded },
  { label: t('general.downloaded'), value: UserSearchOrderBy.Downloaded },
  { label: t('statistics.torrents'), value: UserSearchOrderBy.Torrents },
  { label: t('artist.title_groups'), value: UserSearchOrderBy.TitleGroups },
  { label: t('community.title_group_comments'), value: UserSearchOrderBy.TitleGroupComments },
  { label: t('community.forum_posts'), value: UserSearchOrderBy.ForumPosts },
  { label: t('community.forum_threads'), value: UserSearchOrderBy.ForumThreads },
]

const orderByDirectionOptions = getOrderByDirectionOptions(t)

const userSearchOrderByValues: string[] = Object.values(UserSearchOrderBy)
const isUserSearchOrderBy = (value: unknown): value is UserSearchOrderBy => typeof value === 'string' && userSearchOrderByValues.includes(value)

const orderByDirectionValues: string[] = Object.values(OrderByDirection)
const isOrderByDirection = (value: unknown): value is OrderByDirection => typeof value === 'string' && orderByDirectionValues.includes(value)

const sortOrder = computed(() => (searchForm.value.order_by_direction === OrderByDirection.Asc ? 1 : -1))

const onSort = (event: DataTableSortEvent) => {
  if (typeof event.sortField === 'string' && isUserSearchOrderBy(event.sortField)) {
    searchForm.value.order_by = event.sortField
    searchForm.value.order_by_direction = event.sortOrder === 1 ? OrderByDirection.Asc : OrderByDirection.Desc
    updateUrl()
  }
}

const updateUrl = () => {
  router.push({
    query: {
      username: searchForm.value.username || undefined,
      order_by: searchForm.value.order_by,
      order_by_direction: searchForm.value.order_by_direction,
      page: searchForm.value.page.toString(),
    },
  })
}

const fetchSearchResults = async () => {
  const orderBy = route.query.order_by
  const orderByDirection = route.query.order_by_direction

  searchForm.value.page = route.query.page ? parseInt(route.query.page.toString()) : 1
  searchForm.value.username = route.query.username?.toString() ?? ''
  searchForm.value.order_by = isUserSearchOrderBy(orderBy) ? orderBy : UserSearchOrderBy.CreatedAt
  searchForm.value.order_by_direction = isOrderByDirection(orderByDirection) ? orderByDirection : OrderByDirection.Desc

  const response = await searchUsers({
    username: searchForm.value.username || undefined,
    order_by: searchForm.value.order_by,
    order_by_direction: searchForm.value.order_by_direction,
    page: searchForm.value.page,
    page_size: searchForm.value.page_size,
  })
  searchResults.value = response.results
  totalResults.value = response.total_items
}

onMounted(() => fetchSearchResults())

watch(
  () => route.query,
  () => fetchSearchResults(),
  { deep: true },
)
</script>

<style scoped>
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
</style>
