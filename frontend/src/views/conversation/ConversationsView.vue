<template>
  <div class="title">{{ t('conversation.conversation', 2) }}</div>
  <ContentContainer class="search-form">
    <FloatLabel>
      <InputText v-model="searchForm.search_term" size="small" />
      <label>{{ t('general.search') }}</label>
    </FloatLabel>
    <div class="search-titles-only">
      <Checkbox v-model="searchForm.search_titles_only" :binary="true" inputId="search-titles-only" />
      <label for="search-titles-only">{{ t('conversation.search_titles_only') }}</label>
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
    @changePage="onChangePage"
  >
    <div class="card">
      <DataTable :value="searchResults" :rowClass="(conversation) => (isConversationRead(conversation) ? '' : 'bg-unread')">
        <Column :header="t('conversation.subject')">
          <template #body="slotProps">
            <RouterLink
              :to="`/conversation/${slotProps.data.conversation_id}`"
              @click="isConversationRead(slotProps.data) ? null : (notificationsStore.conversations -= 1)"
            >
              {{ slotProps.data.subject }}
            </RouterLink>
          </template>
        </Column>
        <Column :header="t('conversation.last_message')">
          <template #body="slotProps">
            {{ timeAgo(slotProps.data.last_message_created_at) }}
            {{ t('general.by') }}
            <UsernameEnriched
              :user="{ id: slotProps.data.last_message_created_by_id, username: slotProps.data.last_message_created_by_username, warned: false, banned: false }"
            />
          </template>
        </Column>
        <Column :header="t('general.started')">
          <template #body="slotProps">
            {{ timeAgo(slotProps.data.conversation_created_at) }}
          </template>
        </Column>
        <Column :header="t('conversation.correspondent')">
          <template #body="slotProps">
            <UsernameEnriched
              :user="{
                id: slotProps.data.correspondant_id,
                username: slotProps.data.correspondant_username,
                warned: slotProps.data.correspondant_warned,
                banned: slotProps.data.correspondant_banned,
              }"
            />
          </template>
        </Column>
      </DataTable>
    </div>
  </PaginatedResults>
</template>

<script setup lang="ts">
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import { useI18n } from 'vue-i18n'
import { onMounted, ref, computed, watch } from 'vue'
import { timeAgo } from '@/services/helpers'
import { RouterLink, useRouter, useRoute } from 'vue-router'
import { useUserStore } from '@/stores/user'
import { useNotificationsStore } from '@/stores/notifications'
import { searchConversations, type PaginatedResultsConversationSearchResultResultsInner } from '@/services/api-schema'
import UsernameEnriched from '@/components/user/UsernameEnriched.vue'
import ContentContainer from '@/components/ContentContainer.vue'
import PaginatedResults from '@/components/PaginatedResults.vue'
import { Button, Checkbox, FloatLabel, InputText } from 'primevue'
import { nextTick } from 'vue'

const { t } = useI18n()
const router = useRouter()
const route = useRoute()
const notificationsStore = useNotificationsStore()

const searchForm = ref({ search_term: '', search_titles_only: true, page: 1, page_size: 50 })
const searchResults = ref<PaginatedResultsConversationSearchResultResultsInner[]>([])
const totalResults = ref(0)
const totalPages = computed(() => Math.ceil(totalResults.value / searchForm.value.page_size))

const updateUrl = () => {
  router.push({
    query: {
      ...(searchForm.value.search_term ? { search_term: searchForm.value.search_term } : {}),
      ...(!searchForm.value.search_titles_only ? { search_titles_only: 'false' } : {}),
      page: searchForm.value.page.toString(),
      page_size: searchForm.value.page_size.toString(),
    },
  })
}

const onChangePage = (pagination: { page: number; pageSize: number }) => {
  searchForm.value.page = pagination.page
  updateUrl()
}

const fetchConversations = () => {
  searchForm.value.page = route.query.page ? parseInt(route.query.page as string) : 1
  searchForm.value.page_size = route.query.page_size ? parseInt(route.query.page_size as string) : 50
  searchForm.value.search_term = (route.query.search_term as string) || ''
  searchForm.value.search_titles_only = route.query.search_titles_only !== 'false'

  searchConversations({
    search_term: searchForm.value.search_term || undefined,
    search_titles_only: searchForm.value.search_titles_only,
    page: searchForm.value.page,
    page_size: searchForm.value.page_size,
  }).then(async (response) => {
    searchResults.value.length = 0
    await nextTick()
    searchResults.value = response.results
    totalResults.value = response.total_items
  })
}

const isConversationRead = (c: PaginatedResultsConversationSearchResultResultsInner) => {
  const userId = useUserStore().id
  return (
    c.last_message_created_by_id === userId ||
    (userId === c.receiver_id
      ? c.receiver_last_seen_at != null && new Date(c.receiver_last_seen_at).getTime() > new Date(c.last_message_created_at).getTime()
      : new Date(c.sender_last_seen_at).getTime() > new Date(c.last_message_created_at).getTime())
  )
}

onMounted(() => {
  fetchConversations()
})

watch(
  () => route.query,
  () => {
    fetchConversations()
  },
  { deep: true },
)
</script>

<style scoped>
.title {
  margin-bottom: 10px;
}
.search-form {
  margin-bottom: 15px;
}
.search-titles-only {
  display: flex;
  align-items: center;
  gap: 5px;
  margin-top: 5px;
}
</style>
