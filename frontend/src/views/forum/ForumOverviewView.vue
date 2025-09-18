<template>
  <ForumSearchForm />
  <LatestForumPosts v-if="latestForumPosts" :latestPosts="latestForumPosts" />
  <div v-if="forumOverview">
    <ForumCategoryOverview class="forum-category" v-for="category in forumOverview.forum_categories" :key="category.id" :forum-category="category" />
  </div>
</template>

<script setup lang="ts">
import { getForum, type ForumOverview, type LatestForumPost } from '@/services/api/forumService'
import { onMounted, ref } from 'vue'
import ForumCategoryOverview from '@/components/forum/ForumCategoryOverview.vue'
import ForumSearchForm from '@/components/forum/ForumSearchForm.vue'
import LatestForumPosts from '@/components/forum/LatestForumPosts.vue'

const forumOverview = ref<ForumOverview | null>(null)
const latestForumPosts = ref<LatestForumPost[]>([])

onMounted(async () => {
  const data = await getForum()
  forumOverview.value = data.forum_overview
  latestForumPosts.value = data.latest_forum_posts
})
</script>

<style scoped>
.forum-category {
  margin-bottom: 15px;
}
</style>
