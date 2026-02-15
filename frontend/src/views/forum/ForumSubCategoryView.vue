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
        <RouterLink v-if="userStore.permissions.includes('create_forum_thread')" :to="`/forum/thread/new?subCategoryId=${route.params.id}`">
          <i v-tooltip.top="t('forum.new_thread')" class="pi pi-plus cursor-pointer" />
        </RouterLink>
        <i
          v-if="userStore.permissions.includes('delete_forum_sub_category')"
          v-tooltip.top="t('forum.delete_subcategory')"
          class="pi pi-trash cursor-pointer"
          @click="deleteSubCategoryDialogVisible = true"
        />
      </div>
    </div>
    <DataTable :value="forumSubCategory.threads">
      <Column style="width: 1em">
        <template #body="slotProps">
          <div class="left-icon" v-tooltip.top="threadTooltip(slotProps.data)">
            <i :class="[slotProps.data.pinned ? 'pi pi-thumbtack' : 'pi pi-align-left', { unread: slotProps.data.has_new_posts }]" />
            <i class="pi pi-sparkles" v-if="!slotProps.data.ever_opened" />
          </div>
        </template>
      </Column>
      <Column field="name" :header="t('general.name')">
        <template #body="slotProps">
          <RouterLink :to="`/forum/thread/${slotProps.data.id}`" @click="onThreadClick(slotProps.data)">
            {{ slotProps.data.name }}
          </RouterLink>
        </template>
      </Column>
      <Column field="latest_post" :header="t('forum.latest_post')">
        <template #body="slotProps">
          {{ timeAgo(slotProps.data.latest_post.created_at) }} {{ t('general.by') }}
          <UsernameEnriched :user="slotProps.data.latest_post.created_by" />
        </template>
      </Column>
      <Column field="posts_amount" :header="t('forum.posts')" />
      <Column field="views_count" :header="t('forum.views')" />
    </DataTable>
  </div>
  <Dialog closeOnEscape modal :header="t('forum.delete_subcategory')" v-model:visible="deleteSubCategoryDialogVisible">
    <DeleteForumSubCategoryDialog :subCategoryId="parseInt(route.params.id as string)" @deleted="onSubCategoryDeleted" />
  </Dialog>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import { timeAgo } from '@/services/helpers'
import { RouterLink, useRouter } from 'vue-router'
import { useRoute } from 'vue-router'
import { onMounted } from 'vue'
import { ref } from 'vue'
import { getForumSubCategoryThreads, type ForumSubCategoryHierarchy, type ForumThreadHierarchy } from '@/services/api-schema'
import { useUserStore } from '@/stores/user'
import { useNotificationsStore } from '@/stores/notifications'
import UsernameEnriched from '@/components/user/UsernameEnriched.vue'
import { Dialog } from 'primevue'
import DeleteForumSubCategoryDialog from '@/components/forum/DeleteForumSubCategoryDialog.vue'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()
const userStore = useUserStore()
const notificationsStore = useNotificationsStore()

const forumSubCategory = ref<null | ForumSubCategoryHierarchy>(null)
const siteName = import.meta.env.VITE_SITE_NAME
const deleteSubCategoryDialogVisible = ref(false)

onMounted(async () => {
  forumSubCategory.value = await getForumSubCategoryThreads(parseInt(route.params.id as string))

  document.title = forumSubCategory.value ? `${forumSubCategory.value.name} - ${siteName}` : `Forum category - ${siteName}`
})

const threadTooltip = (thread: ForumThreadHierarchy) =>
  [
    thread.pinned && t('forum.pinned'),
    !thread.ever_opened && t('forum.new'),
    thread.has_new_posts && t('forum.unread'),
    !thread.has_new_posts && t('forum.read'),
  ]
    .filter(Boolean)
    .join(', ')

const onThreadClick = (thread: ForumThreadHierarchy) => {
  if (!thread.ever_opened && route.params.id === '1' && notificationsStore.unread_announcements_amount > 0) {
    notificationsStore.unread_announcements_amount--
  }
}

const onSubCategoryDeleted = () => {
  deleteSubCategoryDialogVisible.value = false
  router.push('/forum')
}
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
.unread {
  color: var(--p-primary-color);
}
.left-icon {
  display: flex;
  .pi-sparkles {
    font-size: 0.95em;
    margin-left: -0.5em;
    margin-top: -0.4em;
  }
}
</style>
