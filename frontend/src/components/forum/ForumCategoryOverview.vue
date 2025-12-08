<template>
  <div>
    <div class="title">{{ forumCategory.name }}</div>
    <DataTable :value="forumCategory.sub_categories">
      <Column style="width: 30%" field="name" :header="t('general.name')">
        <template #body="slotProps">
          <RouterLink :to="'/forum/sub-category/' + slotProps.data.id">
            {{ slotProps.data.name }}
          </RouterLink>
        </template>
      </Column>
      <Column style="width: 35%" field="latest_post_in_thread.name" :header="t('forum.latest_post')">
        <template #body="slotProps">
          <RouterLink :to="'/forum/thread/' + slotProps.data.latest_post_in_thread.id">
            {{ slotProps.data.latest_post_in_thread.name }}
          </RouterLink>
        </template>
      </Column>
      <Column style="width: 25%" field="latest_post_in_thread.created_at">
        <template #body="slotProps">
          {{ timeAgo(slotProps.data.latest_post_in_thread.created_at) }} {{ t('general.by') }}
          <RouterLink :to="'/user/' + slotProps.data.latest_post_in_thread.created_by.id">
            {{ slotProps.data.latest_post_in_thread.created_by.username }}
          </RouterLink>
        </template>
      </Column>
      <Column style="width: 5%" field="threads_amount" :header="t('forum.thread', 2)" />
      <Column style="width: 5%" field="posts_amount" :header="t('forum.posts')" />
    </DataTable>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import { timeAgo } from '@/services/helpers'
import { RouterLink } from 'vue-router'
import type { ForumCategoryHierarchy } from '@/services/api-schema'

defineProps<{
  forumCategory: ForumCategoryHierarchy
}>()

const { t } = useI18n()
</script>

<style scoped>
.title {
  margin-bottom: 10px;
}
</style>
