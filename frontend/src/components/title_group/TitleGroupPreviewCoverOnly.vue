<template>
  <div class="title-group-preview-cover-only">
    <RouterLink :to="`/title-group/${titleGroup.id}`">
      <img class="title-group-cover" :src="titleGroup.covers[0]" v-tooltip.top="titleGroup.name" alt="Title Group Cover" />
    </RouterLink>
  </div>
</template>
<script setup lang="ts">
import type { TitleGroupLite } from '@/services/api/titleGroupService'

defineProps<{
  titleGroup: TitleGroupLite
}>()
</script>

<style>
.title-group-preview-cover-only .title-group-cover {
  height: 20em;
  border-radius: 7px;
}
.title-group-preview-cover-only a:hover {
  background-color: transparent;
}
</style>
