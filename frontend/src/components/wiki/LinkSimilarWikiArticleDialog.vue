<template>
  <Dialog :visible="visible" @update:visible="$emit('update:visible', $event)" modal :header="t('wiki.link_similar_article')" :style="{ width: '30em' }">
    <AutoComplete
      v-model="searchInput"
      :suggestions="searchSuggestions"
      @complete="searchArticles"
      @option-select="onArticleSelected"
      optionLabel="title"
      :placeholder="t('wiki.search_article_to_link')"
      size="small"
      fluid
    />
  </Dialog>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { AutoComplete, Dialog, type AutoCompleteOptionSelectEvent } from 'primevue'
import { linkSimilarWikiArticles, searchWikiArticles, type WikiSearchResult } from '@/services/api-schema'
import { showToast } from '@/main'

const props = defineProps<{
  visible: boolean
  articleId: number
  linkedArticleIds: number[]
}>()

const emit = defineEmits<{
  'update:visible': [boolean]
  linked: [WikiSearchResult]
}>()

const { t } = useI18n()

const searchInput = ref('')
const searchSuggestions = ref<WikiSearchResult[]>([])

const searchArticles = () => {
  if (searchInput.value === '') {
    searchSuggestions.value = []
    return
  }
  searchWikiArticles({
    search_string: searchInput.value,
    title_only: true,
    page: 1,
    page_size: 10,
  }).then((response) => {
    const linkedIds = new Set(props.linkedArticleIds)
    searchSuggestions.value = response.results.filter((r) => r.id !== props.articleId && !linkedIds.has(r.id))
  })
}

const onArticleSelected = (event: AutoCompleteOptionSelectEvent) => {
  const selected = event.value as WikiSearchResult
  linkSimilarWikiArticles({
    wiki_article_id_1: props.articleId,
    wiki_article_id_2: selected.id,
  }).then(() => {
    showToast('Success', t('wiki.similar_article_linked'), 'success', 4000)
    emit('linked', selected)
    emit('update:visible', false)
    searchInput.value = ''
    searchSuggestions.value = []
  })
}
</script>
