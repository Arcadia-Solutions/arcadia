<template>
  <div id="collage-sidebar">
    <ContentContainer :containerTitle="t('general.description')">{{ collage.description }}</ContentContainer>
    <CatalogStats :titleGroupStats />
  </div>
</template>
<script setup lang="ts">
import { type Collage, type TitleGroupStatsResponse } from '@/services/api-schema'
import CatalogStats from '@/components/stats/CatalogStats.vue'
import ContentContainer from '../ContentContainer.vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

defineProps<{
  collage: Collage
  titleGroupStats: TitleGroupStatsResponse
}>()
</script>
<style scoped>
#collage-sidebar {
  display: block;
  > * {
    margin-bottom: 10px;
  }
}
</style>
