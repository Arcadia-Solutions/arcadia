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
        <Button :loading :label="t('general.search')" @click="emit('search', searchForm)" />
      </div>
    </div>
  </ContentContainer>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import ContentContainer from '../ContentContainer.vue'
import InputText from 'primevue/inputtext'
import FloatLabel from 'primevue/floatlabel'
import Button from 'primevue/button'
import type { SearchTorrentRequestsQuery } from '@/services/api/torrentRequestService'
import { onMounted } from 'vue'

const { t } = useI18n()

const props = defineProps<{
  loading: boolean
  initialForm: SearchTorrentRequestsQuery
}>()

const emit = defineEmits<{
  search: [form: SearchTorrentRequestsQuery]
}>()

const searchForm = ref<SearchTorrentRequestsQuery>({
  title_group_name: null,
  tags: null,
  page: 1,
  page_size: 20,
})

const tagsInput = computed({
  get: () => searchForm.value.tags?.join(', ') || '',
  set: (value: string) => {
    searchForm.value.tags = value.trim() ? value.split(',').map((tag) => tag.trim()) : null
  },
})

const changePage = (to: 'previous' | 'next') => {
  searchForm.value.page = to === 'previous' ? searchForm.value.page! - 1 : searchForm.value.page! + 1
  emit('search', searchForm.value)
}

defineExpose({
  searchForm,
  changePage,
})

onMounted(async () => {
  searchForm.value = props.initialForm
})
</script>

<style>
.title-group-name {
  width: 40em;
}
.tags {
  width: 30em;
}
</style>
