<template>
  <div id="collage-sidebar">
    <ContentContainer :containerTitle="t('general.description')">{{ collage.description }}</ContentContainer>
  </div>
</template>
<script setup lang="ts">
import type { Collage } from '@/services/api-schema'
import ContentContainer from '../ContentContainer.vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

defineProps<{
  collage: Collage
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
