<template>
  <ContentContainer :containerTitle="t('forum.latest_posts')" v-if="latestPosts && latestPosts.length > 0">
    <DataTable :value="latestPosts" class="latest-posts-table">
      <Column field="forum_thread_name" :header="t('general.title')" style="width: 30%">
        <template #body="slotProps">
          <RouterLink :to="`/forum/thread/${slotProps.data.forum_thread_id}`">
            {{ slotProps.data.forum_thread_name }}
          </RouterLink>
        </template>
      </Column>
      <Column field="content" :header="t('general.content')" style="width: 30%">
        <template #body="slotProps">
          {{ truncateContent(slotProps.data.content) }}
        </template>
      </Column>
      <Column field="created_at" :header="t('forum.latest_post')" style="width: 25%">
        <template #body="slotProps">
          {{ timeAgo(slotProps.data.created_at) }} {{ t('general.by') }}
          <RouterLink :to="`/user/${slotProps.data.created_by_id}`">
            {{ slotProps.data.created_by_username }}
          </RouterLink>
        </template>
      </Column>
      <Column field="forum_sub_category_name" :header="t('general.category')" style="width: 15%">
        <template #body="slotProps">
          <RouterLink :to="`/forum/sub-category/${slotProps.data.forum_sub_category_id}`">
            {{ slotProps.data.forum_sub_category_name }}
          </RouterLink>
        </template>
      </Column>
    </DataTable>
  </ContentContainer>
</template>

<script setup lang="ts">
import ContentContainer from '@/components/ContentContainer.vue'
import { useI18n } from 'vue-i18n'
import { RouterLink } from 'vue-router'
import { timeAgo } from '@/services/helpers'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import type { LatestForumPost } from '@/services/api/homeService'

const { t } = useI18n()

defineProps<{
  latestPosts: LatestForumPost[]
}>()

const truncateContent = (content: string, maxLength = 100) => {
  const cleanContent = content.replace(/\[.*?\]/g, '').trim()
  return cleanContent.length <= maxLength ? cleanContent : cleanContent.substring(0, maxLength) + '...'
}
</script>

<style scoped>
.latest-posts-table :deep(.p-datatable-tbody > tr > td) {
  padding: 0.5rem 0.75rem;
}

.latest-posts-table :deep(.p-datatable-thead > tr > th) {
  padding: 0.5rem 0.75rem;
}

.content-container {
  margin-top: 15px;
}
</style>
