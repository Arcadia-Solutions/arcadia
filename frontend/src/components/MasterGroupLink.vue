<template>
  <RouterLink v-tooltip.top="title_group.platform ?? title_group.name" :to="`/title-group/${title_group.id}`">
    <img :src="`/logos/platforms/${title_group.platform ?? title_group.content_type}.svg`" />
  </RouterLink>
</template>

<script setup lang="ts">
import type { TitleGroup, TitleGroupLite } from '@/services/api-schema'

defineProps<{
  title_group: TitleGroup | TitleGroupLite
}>()
</script>
<style scoped>
img {
  width: 30px;
}
a {
  background-color: transparent;
}
</style>
