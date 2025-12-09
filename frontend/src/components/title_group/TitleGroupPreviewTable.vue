<template>
  <ContentContainer>
    <div class="title-group-preview-table">
      <!-- TODO : add tags and other potentially useful information -->
      <!-- TODO : clicking on a torrent should redirect to the title_group page
      edit the titlegrouptable component to have a prop that allows this -->
      <ImagePreview class="cover" :imageLink="title_group.covers[0]" />
      <div class="right">
        <div class="title">
          <RouterLink :to="`/title-group/${title_group.id}`">{{ title_group.name }}</RouterLink>
          <span class="year">({{ title_group.original_release_date.substring(0, 4) }})</span>
        </div>
        <TitleGroupTable :title_group="title_group" :editionGroups="title_group.edition_groups" :preview="true" />
      </div>
    </div>
  </ContentContainer>
</template>
<script setup lang="ts">
import TitleGroupTable from './TitleGroupTable.vue'
import ContentContainer from '../ContentContainer.vue'
import ImagePreview from '../ImagePreview.vue'
import type { TitleGroupHierarchyLite } from '@/services/api-schema'

defineProps<{
  title_group: TitleGroupHierarchyLite
}>()
</script>
<style scoped>
.title-group-preview-table {
  display: flex;
  justify-content: center;
  align-items: start;
}
.right {
  width: 100%;
}
.title {
  margin-bottom: 5px;
}
.year {
  font-size: 0.8em;
  margin-left: 5px;
}
</style>
<style>
.title-group-preview-table .cover {
  margin-right: 20px;
}
.title-group-preview-table .cover img {
  border-radius: 7px;
  width: 18em;
}
</style>
