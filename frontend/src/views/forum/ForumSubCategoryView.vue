<template>
  <div v-if="forumSubCategory">
    <div class="top-bar">
      <div class="title">
        <RouterLink to="/forum">{{ forumSubCategory.category.name }}</RouterLink> >
        <RouterLink to="">{{ forumSubCategory.name }}</RouterLink>
      </div>
      <div class="actions">
        <RouterLink :to="`/forum/sub-category/${route.params.id}/edit`" v-if="forumSubCategory && userStore.permissions.includes('edit_forum_sub_category')">
          <i v-tooltip.top="t('forum.edit_subcategory')" class="pi pi-pen-to-square cursor-pointer" />
        </RouterLink>
        <RouterLink :to="`/forum/thread/new?subCategoryId=${route.params.id}`">
          <i v-tooltip.top="t('forum.new_thread')" class="pi pi-plus cursor-pointer" />
        </RouterLink>
      </div>
    </div>
    <DataTable :value="forumSubCategory.threads">
      <Column field="name" :header="t('general.name')">
        <template #body="slotProps">
          <RouterLink :to="`/forum/thread/${slotProps.data.id}`">
            {{ slotProps.data.name }}
          </RouterLink>
        </template>
      </Column>
      <Column field="latest_post" :header="t('forum.latest_post')">
        <template #body="slotProps">
          {{ timeAgo(slotProps.data.latest_post.created_at) }} {{ t('general.by') }}
          <RouterLink :to="`/user/${slotProps.data.latest_post.created_by.id}`">
            {{ slotProps.data.latest_post.created_by.username }}
          </RouterLink>
        </template>
      </Column>
      <Column field="posts_amount" :header="t('forum.posts')" />
    </DataTable>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import { timeAgo } from '@/services/helpers'
import { RouterLink } from 'vue-router'
import { useRoute } from 'vue-router'
import { onMounted } from 'vue'
import { ref } from 'vue'
import { getForumSubCategoryThreads, type ForumSubCategoryHierarchy } from '@/services/api-schema'
import { useUserStore } from '@/stores/user'

const { t } = useI18n()
const route = useRoute()
const userStore = useUserStore()

const forumSubCategory = ref<null | ForumSubCategoryHierarchy>(null)
const siteName = import.meta.env.VITE_SITE_NAME

onMounted(async () => {
  forumSubCategory.value = await getForumSubCategoryThreads(parseInt(route.params.id as string))

  document.title = forumSubCategory.value ? `${forumSubCategory.value.name} - ${siteName}` : `Forum category - ${siteName}`
})
</script>

<style scoped>
.top-bar {
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
  margin-bottom: 10px;
  .actions {
    i {
      color: white;
      margin-left: 7px;
    }
  }
}
</style>
