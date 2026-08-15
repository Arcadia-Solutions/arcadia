<template>
  <div v-if="comments.length > 0">
    <I18nT :keypath="titleKey" tag="div" scope="global" class="page-title">
      <template #username>
        <UsernameEnriched :user="comments[0].created_by" />
      </template>
    </I18nT>
    <PaginatedResults :totalItems="totalComments" :pageSize="pageSize" :initialPage="page" :totalPages="totalPages" @changePage="changePage($event.page)">
      <div v-for="comment in comments" :key="comment.id" class="comment">
        <div class="location">
          <slot name="location" :comment="comment" />
        </div>
        <slot :comment="comment" :commentEdited="commentEdited" :commentDeleted="commentDeleted" />
      </div>
    </PaginatedResults>
  </div>
</template>

<script setup lang="ts" generic="T extends { id: number; created_by: UserLiteAvatar }">
import { computed, onMounted, ref, shallowRef, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { I18nT } from 'vue-i18n'
import PaginatedResults from '@/components/PaginatedResults.vue'
import UsernameEnriched from '@/components/user/UsernameEnriched.vue'
import type { UserLiteAvatar } from '@/services/api-schema'
import { showToast } from '@/main'

const props = defineProps<{
  // fetches one page of the comments written by the user given in the url
  searchComments: (request: { created_by_id: number; page: number; page_size: number }) => Promise<{ results: T[]; total_items: number }>
  // translation key of the page title, taking the linked username as its `username` parameter
  titleKey: string
  // shown when a comment of the list is edited, for the lists whose comments can be edited
  editSuccessMessage?: string
}>()

const route = useRoute()
const router = useRouter()

const pageSize = 10
const page = ref(1)
const comments = shallowRef<T[]>([])
const totalComments = ref(0)
const totalPages = computed(() => Math.ceil(totalComments.value / pageSize))

const commentEdited = (commentId: number, changes: Partial<T>) => {
  comments.value = comments.value.map((comment) => (comment.id === commentId ? { ...comment, ...changes } : comment))
  if (props.editSuccessMessage) {
    showToast('', props.editSuccessMessage, 'success', 2000)
  }
}

const commentDeleted = (commentId: number) => {
  comments.value = comments.value.filter((comment) => comment.id !== commentId)
  totalComments.value--
}

const fetchCommentsFromUrl = () => {
  page.value = route.query.page ? parseInt(route.query.page as string) : 1
  props
    .searchComments({
      created_by_id: parseInt(route.query.created_by_id as string),
      page: page.value,
      page_size: pageSize,
    })
    .then((paginatedComments) => {
      comments.value = paginatedComments.results
      totalComments.value = paginatedComments.total_items
    })
}

const changePage = (newPage: number) => {
  router.push({ query: { ...route.query, page: newPage.toString() } })
}

onMounted(() => {
  fetchCommentsFromUrl()
})

watch(
  () => route.query,
  () => {
    fetchCommentsFromUrl()
  },
  { deep: true },
)
</script>

<style scoped>
.page-title {
  font-size: 1.2em;
  margin-bottom: 10px;
}
.comment {
  margin-bottom: 15px;
}
.location {
  display: flex;
  gap: 5px;
  align-items: center;
  font-size: 0.9em;
  margin-bottom: 3px;
}
.location :deep(i) {
  font-size: 0.7em;
}
</style>
