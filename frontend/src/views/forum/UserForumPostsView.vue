<template>
  <div v-if="posts.length > 0">
    <div class="page-title">{{ t('forum.posts_of_user', [posts[0].created_by.username]) }}</div>
    <PaginatedResults :totalItems="totalPosts" :pageSize="pageSize" :initialPage="page" :totalPages="totalPages" @changePage="changePage($event.page)">
      <div v-for="post in posts" :key="post.id" class="post">
        <div class="location">
          <RouterLink to="/forum">{{ post.category_name }}</RouterLink>
          &gt;
          <RouterLink :to="`/forum/sub-category/${post.sub_category_id}`">{{ post.sub_category_name }}</RouterLink>
          &gt;
          <ForumThreadName :threadId="post.forum_thread_id" :threadName="post.thread_name" :postId="post.id" />
        </div>
        <GeneralComment
          :comment="post"
          :editCommentMethod="editForumPostMethod"
          :deleteCommentMethod="deleteForumPost"
          @commentEdited="postEdited($event as EditedForumPost)"
          @commentDeleted="postDeleted"
          :hasEditPermission="userStore.permissions.includes('edit_forum_post') || (post.created_by.id === userStore.id && !post.locked)"
          :hasDeletePermission="userStore.permissions.includes('delete_forum_post')"
        />
      </div>
    </PaginatedResults>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import GeneralComment from '@/components/community/GeneralComment.vue'
import ForumThreadName from '@/components/forum/ForumThreadName.vue'
import PaginatedResults from '@/components/PaginatedResults.vue'
import { deleteForumPost, editForumPost, searchForumPosts, type EditedForumPost, type ForumPostWithLocation } from '@/services/api-schema'
import { useUserStore } from '@/stores/user'
import { showToast } from '@/main'

const route = useRoute()
const router = useRouter()
const userStore = useUserStore()
const { t } = useI18n()

const pageSize = 10
const page = ref(1)
const posts = ref<ForumPostWithLocation[]>([])
const totalPosts = ref(0)
const totalPages = computed(() => Math.ceil(totalPosts.value / pageSize))

const editForumPostMethod = async (post: EditedForumPost) => {
  editForumPost(post)
}

const postEdited = (editedPost: EditedForumPost) => {
  const index = posts.value.findIndex((post) => post.id === editedPost.id)
  if (index !== -1) {
    posts.value[index] = { ...posts.value[index], ...editedPost }
    showToast('', t('forum.post_edited_success'), 'success', 2000)
  }
}

const postDeleted = (postId: number) => {
  const index = posts.value.findIndex((post) => post.id === postId)
  if (index !== -1) {
    posts.value.splice(index, 1)
    totalPosts.value--
  }
}

const fetchPostsFromUrl = () => {
  page.value = route.query.page ? parseInt(route.query.page as string) : 1
  searchForumPosts({
    created_by_id: parseInt(route.query.created_by_id as string),
    page: page.value,
    page_size: pageSize,
  }).then((paginatedPosts) => {
    posts.value = paginatedPosts.results
    totalPosts.value = paginatedPosts.total_items
  })
}

const changePage = (newPage: number) => {
  router.push({ query: { ...route.query, page: newPage.toString() } })
}

onMounted(() => {
  fetchPostsFromUrl()
})

watch(
  () => route.query,
  () => {
    fetchPostsFromUrl()
  },
  { deep: true },
)
</script>

<style scoped>
.page-title {
  font-size: 1.2em;
  margin-bottom: 10px;
}
.post {
  margin-bottom: 15px;
}
.location {
  display: flex;
  gap: 5px;
  font-size: 0.9em;
  margin-bottom: 3px;
}
</style>
