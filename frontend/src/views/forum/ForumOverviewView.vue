<template>
  <div v-if="forumOverview">
    <ContentContainer :container-title="t('forum.latest_post', 2)" style="margin-bottom: 30px">
      <ForumSearchResults :search-results="forumOverview.latest_posts_in_threads" />
    </ContentContainer>
    <div class="actions">
      <RouterLink to="">
        <i class="pi pi-bookmark" v-tooltip.top="'Not implemented yet'" />
      </RouterLink>
      <RouterLink to="">
        <i class="pi pi-bell" v-tooltip.top="'Not implemented yet'" />
      </RouterLink>
      <RouterLink to="/forum/search">
        <i class="pi pi-search" v-tooltip.top="t('forum.search')" />
      </RouterLink>
      <RouterLink to="/forum/category/new" v-if="userStore.permissions.includes('create_forum_category')">
        <i class="pi pi-plus" v-tooltip.top="t('forum.create_category')" />
      </RouterLink>
    </div>
    <ForumCategoryOverview class="forum-category" v-for="category in forumOverview.forum_categories" :key="category.id" :forum-category="category" />
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { ref } from 'vue'
import ForumCategoryOverview from '@/components/forum/ForumCategoryOverview.vue'
import ForumSearchResults from '@/components/forum/ForumSearchResults.vue'
import ContentContainer from '@/components/ContentContainer.vue'
import { useI18n } from 'vue-i18n'
import { RouterLink } from 'vue-router'
import { getForum, type ForumOverview } from '@/services/api-schema'
import { useUserStore } from '@/stores/user'

const { t } = useI18n()
const userStore = useUserStore()

const forumOverview = ref<null | ForumOverview>(null)

onMounted(async () => {
  forumOverview.value = await getForum()
})
</script>

<style scoped>
.actions {
  margin-bottom: 15px;
  display: flex;
  justify-content: center;
  > a {
    margin: 0 10px;
    color: white;
  }
}
.forum-category {
  margin-bottom: 20px;
}
</style>
