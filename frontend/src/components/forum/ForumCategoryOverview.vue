<template>
  <div>
    <div class="top">
      <div class="title">{{ forumCategory.name }}</div>
      <div class="actions">
        <RouterLink
          :to="`/forum/category/${forumCategory.id}/edit`"
          v-if="userStore.permissions.includes('edit_forum_category')"
          v-tooltip.top="t('forum.edit_category')"
        >
          <i class="pi pi-pen-to-square" />
        </RouterLink>
        <RouterLink
          :to="{ path: '/forum/sub-category/new', query: { categoryId: forumCategory.id, categoryName: forumCategory.name } }"
          v-if="userStore.permissions.includes('create_forum_sub_category')"
          v-tooltip.top="t('forum.create_sub_category')"
        >
          <i class="pi pi-plus" />
        </RouterLink>
        <i
          v-if="userStore.permissions.includes('delete_forum_category')"
          class="pi pi-trash"
          v-tooltip.top="t('forum.delete_category')"
          @click="deleteCategoryDialogVisible = true"
        />
      </div>
    </div>
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
          <RouterLink :to="'/forum/thread/' + slotProps.data.latest_post_in_thread.thread_id" v-if="slotProps.data.latest_post_in_thread">
            {{ slotProps.data.latest_post_in_thread.name }}
          </RouterLink>
        </template>
      </Column>
      <Column style="width: 25%" field="latest_post_in_thread.created_at">
        <template #body="slotProps">
          <template v-if="slotProps.data.latest_post_in_thread">
            {{ timeAgo(slotProps.data.latest_post_in_thread.created_at) }} {{ t('general.by') }}
            <RouterLink :to="'/user/' + slotProps.data.latest_post_in_thread.created_by.id">
              {{ slotProps.data.latest_post_in_thread.created_by.username }}
            </RouterLink>
          </template>
        </template>
      </Column>
      <Column style="width: 5%" field="threads_amount" :header="t('forum.thread', 2)" />
      <Column style="width: 5%" field="posts_amount" :header="t('forum.posts')" />
    </DataTable>
  </div>
  <Dialog closeOnEscape modal :header="t('forum.delete_category')" v-model:visible="deleteCategoryDialogVisible">
    <DeleteForumCategoryDialog :categoryId="forumCategory.id" @deleted="onCategoryDeleted" />
  </Dialog>
  <Dialog closeOnEscape modal :header="t('forum.delete_subcategory')" v-model:visible="deleteSubCategoryDialogVisible">
    <DeleteForumSubCategoryDialog v-if="subCategoryToDelete" :subCategoryId="subCategoryToDelete" @deleted="onSubCategoryDeleted" />
  </Dialog>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import { timeAgo } from '@/services/helpers'
import { RouterLink, useRouter } from 'vue-router'
import type { ForumCategoryHierarchy } from '@/services/api-schema'
import { useUserStore } from '@/stores/user'
import { Dialog } from 'primevue'
import { ref } from 'vue'
import DeleteForumCategoryDialog from './DeleteForumCategoryDialog.vue'
import DeleteForumSubCategoryDialog from './DeleteForumSubCategoryDialog.vue'

defineProps<{
  forumCategory: ForumCategoryHierarchy
}>()

const userStore = useUserStore()
const router = useRouter()
const { t } = useI18n()

const deleteCategoryDialogVisible = ref(false)
const deleteSubCategoryDialogVisible = ref(false)
const subCategoryToDelete = ref<number | null>(null)

const onCategoryDeleted = () => {
  deleteCategoryDialogVisible.value = false
  router.go(0)
}

const onSubCategoryDeleted = () => {
  deleteSubCategoryDialogVisible.value = false
  router.go(0)
}
</script>

<style scoped>
.top {
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
  margin-bottom: 10px;
  i {
    color: white;
    margin-left: 7px;
    cursor: pointer;
  }
}
</style>
