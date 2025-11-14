<template>
  <ContentContainer class="search-form">
    <FloatLabel>
      <InputText v-model="searchForm.thread_name" size="small" />
      <label for="name">{{ t('forum.thread_name') }}</label>
    </FloatLabel>
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
    <ForumSearchResults :searchResults />
  </PaginatedResults>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { ref } from 'vue'
import ForumSearchResults from '@/components/forum/ForumSearchResults.vue'
import { useI18n } from 'vue-i18n'
import { Button, FloatLabel, InputText } from 'primevue'
import { searchForum, type ForumSearchQuery, type ForumSearchResult } from '@/services/api/forumService'
import ContentContainer from '@/components/ContentContainer.vue'
import PaginatedResults from '@/components/PaginatedResults.vue'
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { useRoute } from 'vue-router'
import { watch } from 'vue'
import { nextTick } from 'vue'

const { t } = useI18n()
const router = useRouter()
const route = useRoute()

const loading = ref(false)
const searchForm = ref<ForumSearchQuery>({ thread_name: '', page: 1, page_size: 20 })
const searchResults = ref<ForumSearchResult[]>([])
const totalResults = ref<number>(0)
const totalPages = computed(() => Math.ceil(totalResults.value / searchForm.value.page_size))

const updateUrl = () => {
  router.push({
    query: {
      thread_name: searchForm.value.thread_name,
      page: searchForm.value.page.toString(),
      page_size: searchForm.value.page_size.toString(),
    },
  })
}

const fetchSearchResultsFromUrl = async () => {
  loading.value = true
  searchForm.value.page = route.query.page ? parseInt(route.query.page as string) : 1
  searchForm.value.page_size = route.query.page_size ? parseInt(route.query.page_size as string) : 20
  searchForm.value.thread_name = route.query.thread_name ? (route.query.thread_name as string) : ''
  const response = await searchForum(searchForm.value)
  // resets the pagination component
  searchResults.value.length = 0
  await nextTick()
  searchResults.value = response.results
  loading.value = false
}

onMounted(async () => {
  await fetchSearchResultsFromUrl()
})

watch(
  () => route.query,
  () => {
    fetchSearchResultsFromUrl()
  },
  { deep: true },
)
</script>

<style scoped>
.search-form {
  margin-bottom: 15px;
}
</style>
