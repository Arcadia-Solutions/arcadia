<template>
  <div>
    <RouterLink :to="`/forum/thread/${threadId}`">
      {{ threadName }}
    </RouterLink>
    <RouterLink v-if="postId" :to="`/forum/thread/${threadId}?post_id=${postId}#post-${postId}`">
      <i class="pi pi-arrow-right" style="color: white; font-size: 0.7em; margin-left: 5px" />
    </RouterLink>
  </div>
</template>
<script setup lang="ts">
import { RouterLink } from 'vue-router'

defineProps<{
  threadName: string
  threadId: number
  postId?: number
}>()
</script>
