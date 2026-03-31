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
      <i
        v-if="userStore.permissions.includes('edit_forum_category')"
        class="pi pi-arrow-right-arrow-left"
        style="cursor: pointer; color: white"
        v-tooltip.top="t('forum.reorder_categories')"
        @click="reorderCategoriesDialogVisible = true"
      />
    </div>
    <ForumCategoryOverview class="forum-category" v-for="category in forumOverview.forum_categories" :key="category.id" :forum-category="category" />
    <Dialog closeOnEscape modal :header="t('forum.reorder_categories')" v-model:visible="reorderCategoriesDialogVisible">
      <ReorderForumCategoryOrSubCategoryDialog
        v-if="reorderCategoriesDialogVisible"
        mode="categories"
        :initial-items="forumOverview.forum_categories.map((c) => ({ id: c.id, name: c.name }))"
        @reordered="onCategoriesReordered"
      />
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { ref } from 'vue'
import ForumCategoryOverview from '@/components/forum/ForumCategoryOverview.vue'
import ForumSearchResults from '@/components/forum/ForumSearchResults.vue'
import ReorderForumCategoryOrSubCategoryDialog from '@/components/forum/ReorderForumCategoryOrSubCategoryDialog.vue'
import ContentContainer from '@/components/ContentContainer.vue'
import { useI18n } from 'vue-i18n'
import { RouterLink, useRouter } from 'vue-router'
import { Dialog } from 'primevue'
import { getForum, type ForumOverview } from '@/services/api-schema'
import { useUserStore } from '@/stores/user'

const { t } = useI18n()
const userStore = useUserStore()
const router = useRouter()

const forumOverview = ref<null | ForumOverview>(null)
const reorderCategoriesDialogVisible = ref(false)

onMounted(async () => {
  forumOverview.value = await getForum()
})

const onCategoriesReordered = () => {
  reorderCategoriesDialogVisible.value = false
  router.go(0)
}
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
