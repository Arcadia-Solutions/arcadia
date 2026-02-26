<template>
  <div class="subscription-table">
    <PaginatedResults v-if="results" :totalPages :initialPage="page" :totalItems :pageSize @changePage="onPageChange($event.page)">
      <DataTable v-if="results.length > 0" :value="results" size="small">
        <Column :header="t('forum.thread_name')">
          <template #body="{ data }">
            <RouterLink :to="`/forum/thread/${data.id}`">{{ data.name }}</RouterLink>
          </template>
        </Column>
        <Column style="width: 3rem">
          <template #body="{ data }">
            <Button v-tooltip.top="t('subscription.unsubscribe')" icon="pi pi-bell-slash" severity="danger" size="small" text @click="unsubscribe(data.id)" />
          </template>
        </Column>
      </DataTable>
      <div v-else class="wrapper-center">{{ t('subscription.no_subscription') }}</div>
    </PaginatedResults>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute, useRouter } from 'vue-router'
import { Button, Column, DataTable } from 'primevue'
import { showToast } from '@/main'
import PaginatedResults from '@/components/PaginatedResults.vue'
import { getForumThreadPostsSubscriptions, removeForumThreadPostsSubscription, type PaginatedResultsForumThreadLiteResultsInner } from '@/services/api-schema'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()
const results = ref<PaginatedResultsForumThreadLiteResultsInner[]>()
const totalItems = ref(0)
const page = ref(1)
const pageSize = ref(20)
const totalPages = computed(() => Math.ceil(totalItems.value / pageSize.value))

const load = () => {
  getForumThreadPostsSubscriptions({
    page: page.value,
    page_size: pageSize.value,
    order_by_direction: (route.query.order_by_direction as 'asc' | 'desc') ?? 'desc',
  }).then((data) => {
    results.value = data.results
    totalItems.value = data.total_items
  })
}

const onPageChange = (newPage: number) => {
  router.push({ query: { ...route.query, page: String(newPage) } })
}

const unsubscribe = (threadId: number) => {
  removeForumThreadPostsSubscription(threadId).then(() => {
    showToast('', t('title_group.unsubscription_successful'), 'success', 3000)
    load()
  })
}

onMounted(() => {
  page.value = route.query.page ? parseInt(route.query.page as string) : 1
  load()
})

watch(
  () => route.query,
  () => {
    page.value = route.query.page ? parseInt(route.query.page as string) : 1
    load()
  },
  { deep: true },
)
</script>

<style scoped>
.subscription-table {
  margin-top: 20px;
}
</style>
