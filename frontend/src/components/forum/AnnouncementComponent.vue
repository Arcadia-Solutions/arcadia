<template>
  <ContentContainer :containerTitle="announcement.forum_thread_name" :containerTitleLink="`/forum/thread/${announcement.forum_thread_id}`">
    <template #top-right>({{ timeAgo(announcement.created_at) }})</template>
    <BBCodeRenderer :content="announcement.content" />
  </ContentContainer>
</template>

<script setup lang="ts">
import ContentContainer from '../ContentContainer.vue'
import { timeAgo } from '@/services/helpers'
import BBCodeRenderer from '../community/BBCodeRenderer.vue'
import type { ForumPostAndThreadName } from '@/services/api-schema'

defineProps<{
  announcement: ForumPostAndThreadName
}>()
</script>

<style scoped></style>
