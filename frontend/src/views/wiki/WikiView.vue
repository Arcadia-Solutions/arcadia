<template>
  <div class="actions wrapper-center">
    <RouterLink to="/wiki/create-article" v-if="userStore.permissions.includes('create_wiki_article')">
      <i class="pi pi-plus" v-tooltip.top="t('wiki.create_article')" />
    </RouterLink>
    <RouterLink to="/wiki/search">
      <i class="pi pi-search" v-tooltip.top="'Not implemented yet'" />
    </RouterLink>
  </div>
  <div v-if="wikiArticle" class="wiki-article">
    <ContentContainer :containerTitle="wikiArticle.title">
      <template v-if="userStore.permissions.includes('edit_wiki_article')" #top-right>
        <RouterLink :to="`/wiki/article/${wikiArticle.id}/edit`" v-tooltip.top="t('wiki.edit_article')">
          <i class="pi pi-pen-to-square" style="color: white" />
        </RouterLink>
      </template>
      <BBCodeRenderer :content="wikiArticle.body" />
    </ContentContainer>
  </div>
</template>

<script setup lang="ts">
import BBCodeRenderer from '@/components/community/BBCodeRenderer.vue'
import ContentContainer from '@/components/ContentContainer.vue'
import { ref, watch } from 'vue'
import { onMounted } from 'vue'
import { RouterLink, useRoute } from 'vue-router'
import { useUserStore } from '@/stores/user'
import { useI18n } from 'vue-i18n'
import { getWikiArticle, type WikiArticle } from '@/services/api-schema'

const { t } = useI18n()
const route = useRoute()
const userStore = useUserStore()

const wikiArticle = ref<WikiArticle>()

const fetchWikiArticle = async () => {
  getWikiArticle(parseInt(route.params.id as string)).then((article) => {
    wikiArticle.value = article
  })
}

onMounted(() => {
  fetchWikiArticle()
})

watch(() => route.params.id, fetchWikiArticle, { immediate: true })
</script>

<style scoped>
.wiki-article {
  margin: 20px 0;
}
.actions {
  i {
    margin: 0 5px;
    color: white;
  }
}
</style>
