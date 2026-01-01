<template>
  <ContentContainer>
    <div id="torrent-request-search-inputs">
      <div class="line">
        <FloatLabel>
          <InputText class="title-group-name" size="small" v-model="searchForm.title_group_name" name="title_group_name" />
          <label for="title_group_name">{{ t('general.search_terms') }}</label>
        </FloatLabel>
      </div>
      <div class="line">
        <FloatLabel>
          <InputText class="tags" size="small" v-model="tagsInput" name="tags" />
          <label for="tags">{{ t('general.tags_comma_separated') }}</label>
        </FloatLabel>
      </div>
      <div class="flex justify-content-center" style="margin-top: 15px">
        <Button :loading :label="t('general.search')" @click="search" />
      </div>
    </div>
  </ContentContainer>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import ContentContainer from '../ContentContainer.vue'
import InputText from 'primevue/inputtext'
import FloatLabel from 'primevue/floatlabel'
import Button from 'primevue/button'
import type { SearchTorrentRequestsRequest } from '@/services/api-schema'

const { t } = useI18n()
const router = useRouter()

const props = defineProps<{
  loading: boolean
  initialForm: SearchTorrentRequestsRequest
}>()

const searchForm = ref<SearchTorrentRequestsRequest>({
  title_group_name: null,
  tags: null,
  page: 1,
  page_size: 25,
})

const tagsInput = computed({
  get: () => searchForm.value.tags?.join(', ') || '',
  set: (value: string) => {
    searchForm.value.tags = value.trim() ? value.split(',').map((tag) => tag.trim()) : null
  },
})

const changePage = (page: number) => {
  searchForm.value.page = page
  search()
}

const search = () => {
  router.push({
    query: Object.fromEntries(
      Object.entries({
        title_group_name: searchForm.value.title_group_name,
        tags: searchForm.value.tags,
        page: searchForm.value.page,
        page_size: searchForm.value.page_size,
      }).filter(([, v]) => v !== undefined && v !== null),
    ),
  })
}

defineExpose({
  searchForm,
  changePage,
})

onMounted(async () => {
  searchForm.value = props.initialForm
})

watch(
  () => searchForm.value,
  (newVal, oldVal) => {
    if (newVal.page === oldVal.page) {
      searchForm.value.page = 1
    }
  },
  { deep: true },
)
</script>

<style>
.title-group-name {
  width: 40em;
}
.tags {
  width: 30em;
}
</style>
