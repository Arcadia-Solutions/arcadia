<template>
  <ContentContainer v-if="titleGroupPreview === 'cover-only'">
    <div class="title-groups-cover-only">
      <TitleGroupPreviewCoverOnly v-for="title_group in titleGroups" :key="title_group.id" :titleGroup="title_group" />
    </div>
  </ContentContainer>
  <template v-if="titleGroupPreview === 'table'">
    <TitleGroupPreviewTable
      v-for="title_group in titleGroups"
      :key="title_group.id"
      :title_group="title_group as TitleGroupHierarchyLite"
      class="preview-table"
    />
  </template>
</template>

<script setup lang="ts">
import type { TitleGroupHierarchyLite, TitleGroupLite } from '@/services/api-schema'
import ContentContainer from '../ContentContainer.vue'
import TitleGroupPreviewCoverOnly from './TitleGroupPreviewCoverOnly.vue'
import TitleGroupPreviewTable from './TitleGroupPreviewTable.vue'

export type titleGroupPreviewMode = 'table' | 'cover-only'

defineProps<
  | {
      titleGroups: TitleGroupHierarchyLite[]
      titleGroupPreview: 'table' | 'cover-only'
    }
  | {
      titleGroups: TitleGroupLite[]
      titleGroupPreview: 'cover-only'
    }
>()
</script>
<style scoped>
.title-groups-cover-only {
  display: flex;
  align-items: center;
  justify-content: space-around;
  flex-wrap: wrap;
}
.preview-table {
  margin-bottom: 15px;
}
</style>
