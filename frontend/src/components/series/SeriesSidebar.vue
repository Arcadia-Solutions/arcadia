<template>
  <div id="series-sidebar">
    <ImagePreview :imageLink="series.covers[0]" class="series-covers" />
    <ContentContainer class="header-wrapper" container-title="Description">
      <div class="description">
        <BBCodeRenderer :content="series.description" />
      </div>
    </ContentContainer>
    <RelatedForumThreads :itemType="SiteHighlightItemType.Series" :itemId="series.id" v-model="relatedThreads" />
  </div>
</template>

<script setup lang="ts">
import ContentContainer from '@/components/ContentContainer.vue'
import BBCodeRenderer from '@/components/community/BBCodeRenderer.vue'
import RelatedForumThreads from '@/components/forum/RelatedForumThreads.vue'
import ImagePreview from '../ImagePreview.vue'
import { SiteHighlightItemType, type RelatedForumThread, type Series } from '@/services/api-schema'

defineProps<{
  series: Series
}>()

const relatedThreads = defineModel<RelatedForumThread[]>('relatedThreads')
</script>

<style scoped>
#series-sidebar {
  display: flex;
  flex-direction: column;
}
.content-container {
  margin-top: 10px;
}
</style>
<style>
#series-view .series-covers img {
  width: 100%;
  border-radius: 7px;
}
</style>
