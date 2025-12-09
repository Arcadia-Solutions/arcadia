<template>
  <ContentContainer id="series-full-header" class="header-wrapper">
    <div class="header">
      <ImagePreview :imageLink="series.covers[0]" />
      <div class="textual-information">
        <div class="name">{{ series.name }}</div>
        <div class="description">
          <BBCodeRenderer :content="series.description" />
        </div>
      </div>
    </div>
  </ContentContainer>
</template>

<script setup lang="ts">
import ContentContainer from '@/components/ContentContainer.vue'
import BBCodeRenderer from '@/components/community/BBCodeRenderer.vue'
import ImagePreview from '../ImagePreview.vue'
import type { Series } from '@/services/api-schema'

defineProps<{
  series: Series
}>()
</script>

<style scoped>
.header-wrapper {
  margin-bottom: 15px;
}
.header {
  display: flex;
}
.series-covers {
  margin-right: 10px;
}
.name {
  font-weight: bold;
  font-size: 2em;
}
</style>
<style>
#series-full-header .series-covers img {
  width: 100%;
  height: auto;
  border-radius: 7px;
  object-fit: contain;
}
</style>
