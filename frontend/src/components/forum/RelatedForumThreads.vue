<template>
  <ContentContainer v-if="threads.length > 0 || canManageRelatedForumThread" :container-title="t('forum.related_thread', 2)">
    <ul class="related-threads">
      <li v-for="thread in threads" :key="thread.forum_thread_id">
        <RouterLink :to="`/forum/thread/${thread.forum_thread_id}`">{{ thread.thread_name }}</RouterLink>
        <i
          v-if="canManageRelatedForumThread"
          class="pi pi-times"
          v-tooltip.top="t('forum.remove_related_thread')"
          @click="requestRemove(thread.forum_thread_id)"
        />
      </li>
    </ul>
    <div v-if="canManageRelatedForumThread" style="margin-top: 10px">
      <AutoComplete
        v-model="newThreadName"
        :suggestions="foundThreads"
        @complete="searchThreads"
        size="small"
        :placeholder="t('forum.add_related_thread')"
        optionLabel="thread_name"
        @option-select="addRelatedThread($event.value)"
      />
    </div>
    <Dialog v-model:visible="confirmDialogVisible" modal :header="t('forum.remove_related_thread')">
      <p>{{ t('forum.remove_related_thread_confirm') }}</p>
      <div class="wrapper-center" style="gap: 10px; margin-top: 20px">
        <Button :label="t('general.cancel')" size="small" severity="secondary" @click="confirmDialogVisible = false" />
        <Button :label="t('general.confirm')" size="small" severity="danger" @click="confirmRemove" />
      </div>
    </Dialog>
  </ContentContainer>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { AutoComplete, Button, Dialog } from 'primevue'
import ContentContainer from '@/components/ContentContainer.vue'
import {
  createRelatedForumThread,
  deleteRelatedForumThread,
  searchForum,
  SiteHighlightItemType,
  type ForumSearchResult,
  type RelatedForumThread,
} from '@/services/api-schema'
import { useUserStore } from '@/stores/user'
import { showToast } from '@/main'

const { t } = useI18n()
const userStore = useUserStore()

const props = defineProps<{
  itemType: SiteHighlightItemType
  itemId: number
}>()

const threads = defineModel<RelatedForumThread[]>({ default: () => [] })

const canManageRelatedForumThread = computed(() => userStore.permissions.includes('manage_related_forum_thread'))

const newThreadName = ref('')
const foundThreads = ref<ForumSearchResult[]>([])
const confirmDialogVisible = ref(false)
const pendingRemoveForumThreadId = ref<number | null>(null)

const searchThreads = () => {
  searchForum({ thread_name: newThreadName.value, page: 1, page_size: 10 }).then((results) => {
    const existing = new Set(threads.value.map((thread) => thread.forum_thread_id))
    foundThreads.value = results.results.filter((result) => !existing.has(result.thread_id))
  })
}

const addRelatedThread = (selected: ForumSearchResult) => {
  createRelatedForumThread({
    item_type: props.itemType,
    item_id: props.itemId,
    forum_thread_id: selected.thread_id,
  }).then(() => {
    threads.value = [...threads.value, { forum_thread_id: selected.thread_id, thread_name: selected.thread_name, created_at: new Date().toISOString() }]
    newThreadName.value = ''
    showToast('Success', t('forum.related_thread_added'), 'success', 2000)
  })
}

const requestRemove = (forumThreadId: number) => {
  pendingRemoveForumThreadId.value = forumThreadId
  confirmDialogVisible.value = true
}

const confirmRemove = () => {
  const forumThreadId = pendingRemoveForumThreadId.value
  if (forumThreadId === null) return
  deleteRelatedForumThread({
    item_type: props.itemType,
    item_id: props.itemId,
    forum_thread_id: forumThreadId,
  })
    .then(() => {
      threads.value = threads.value.filter((thread) => thread.forum_thread_id !== forumThreadId)
      showToast('Success', t('forum.related_thread_removed'), 'success', 2000)
    })
    .finally(() => {
      confirmDialogVisible.value = false
      pendingRemoveForumThreadId.value = null
    })
}
</script>

<style scoped>
.related-threads {
  list-style: none;
  padding: 0;
  margin: 0;
}
.related-threads li {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
}
.related-threads i {
  cursor: pointer;
}
</style>
