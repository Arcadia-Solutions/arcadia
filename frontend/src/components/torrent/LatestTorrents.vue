<template>
  <ContentContainer :containerTitle :containerTitleLink>
    <div class="last-uploads" v-if="titleGroups">
      <TitleGroupPreviewCoverOnly v-for="titleGroup in titleGroups" :key="titleGroup.id" :titleGroup="titleGroup" class="title-group" />
    </div>
    <div v-else>
      {{ t(`user.no_${props.type}_explanation`) }}
    </div>
  </ContentContainer>
</template>

<script setup lang="ts">
import type { TitleGroupLite } from '@/services/api-schema'
import ContentContainer from '../ContentContainer.vue'
import TitleGroupPreviewCoverOnly from '../title_group/TitleGroupPreviewCoverOnly.vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps<{
  titleGroups: TitleGroupLite[]
  containerTitleLink: string
  containerTitle: string
  type?: 'snatches' | 'uploads'
}>()
</script>

<style>
.last-uploads {
  display: flex;
  justify-content: space-around;
  overflow-y: scroll;
  .title-group {
    margin: 0 5px;
  }
}
</style>
