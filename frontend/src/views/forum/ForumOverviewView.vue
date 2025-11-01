<template>
  <div v-if="forumOverview">
    <ForumCategoryOverview class="forum-category" v-for="category in forumOverview.forum_categories" :key="category.id" :forum-category="category" />
  </div>
</template>

<script setup lang="ts">
import { getForum, type ForumOverview } from '@/services/api/forumService'
import { onMounted } from 'vue'
import { ref } from 'vue'
import ForumCategoryOverview from '@/components/forum/ForumCategoryOverview.vue'

const forumOverview = ref<null | ForumOverview>(null)

onMounted(async () => {
  forumOverview.value = await getForum()
})
</script>

<style scoped>
.forum-category {
  margin-bottom: 15px;
}
</style>
