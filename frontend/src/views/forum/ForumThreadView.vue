<template>
  <div v-if="forumThread">
    <div class="top-bar">
      <div class="title">
        <RouterLink to="/forum">{{ forumThread.forum_category_name }}</RouterLink> >
        <RouterLink :to="`/forum/sub-category/${forumThread.forum_sub_category_id}`">{{ forumThread.forum_sub_category_name }}</RouterLink> >
        {{ forumThread.name }}
      </div>
      <div class="actions">
        <i
          v-if="userStore.class === 'staff' || forumThread.created_by_id === userStore.id"
          class="pi pi-pen-to-square"
          v-tooltip.top="t('forum.edit_thread')"
          @click="editThreadDialogVisible = true"
        />
        <i v-if="togglingSubscription" class="pi pi-hourglass" />
        <i
          v-else
          v-tooltip.top="t(`general.${forumThread.is_subscribed ? 'un' : ''}subscribe`)"
          @click="toggleSubscribtion"
          :class="`pi pi-bell${forumThread.is_subscribed ? '-slash' : ''}`"
        />
      </div>
    </div>
    <PaginatedResults
      v-if="forumThreadPosts.length > 0"
      :totalPages
      :initialPage
      :totalItems="totalPosts"
      @change-page="changePage($event.page)"
      :page-size="pageSize"
    >
      <GeneralComment
        v-for="post in forumThreadPosts"
        :key="post.id"
        :comment="post"
        :editCommentMethod="editForumPostMethod"
        @commentEdited="postEdited($event as EditedForumPost)"
      />
    </PaginatedResults>
    <Form v-slot="$form" :initialValues="newPost" :resolver @submit="onFormSubmit" validateOnSubmit :validateOnValueUpdate="false">
      <div class="new-post">
        <BBCodeEditor
          :emptyInput="bbcodeEditorEmptyInput"
          @value-change="newPostUpdated"
          @input-emptied="bbcodeEditorEmptyInput = false"
          :label="t('forum.new_post')"
        >
          <template #buttons>
            <Button
              type="submit"
              label="Post"
              icon="pi pi-send"
              :loading="sendingPost"
              class="post-button"
              :disabled="currentPage !== totalPages"
              v-tooltip.top="currentPage !== totalPages ? t('forum.go_to_last_page_to_reply') : ''"
            />
          </template>
        </BBCodeEditor>
        <Message v-if="$form.content?.invalid" severity="error" size="small" variant="simple">
          {{ $form.content.error?.message }}
        </Message>
      </div>
    </Form>
  </div>
  <Dialog closeOnEscape modal :header="t('forum.edit_thread')" v-model:visible="editThreadDialogVisible">
    <EditForumThreadDialog v-if="forumThread" :forumThread="forumThread" @done="threadEdited" />
  </Dialog>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { ref } from 'vue'
import { useRoute } from 'vue-router'
import GeneralComment from '@/components/community/GeneralComment.vue'
import type { FormSubmitEvent } from '@primevue/forms'
import { useI18n } from 'vue-i18n'
import { useUserStore } from '@/stores/user'
import { Form } from '@primevue/forms'
import { Button, Message, Dialog } from 'primevue'
import BBCodeEditor from '@/components/community/BBCodeEditor.vue'
import PaginatedResults from '@/components/PaginatedResults.vue'
import EditForumThreadDialog from '@/components/forum/EditForumThreadDialog.vue'
import { nextTick } from 'vue'
import { scrollToHash } from '@/services/helpers'
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { watch } from 'vue'
import { showToast } from '@/main'
import {
  createForumPost,
  createForumThreadPostsSubscription,
  editForumPost,
  getForumThread,
  getForumThreadsPosts,
  removeForumThreadPostsSubscription,
  type EditedForumPost,
  type ForumPostHierarchy,
  type ForumThreadEnriched,
  type UserCreatedForumPost,
} from '@/services/api-schema'

const router = useRouter()
const route = useRoute()
const userStore = useUserStore()
const { t } = useI18n()

const editThreadDialogVisible = ref(false)
const togglingSubscription = ref(false)
const forumThread = ref<null | ForumThreadEnriched>(null)
const forumThreadPosts = ref<ForumPostHierarchy[]>([])
const totalPosts = ref(0)
const pageSize = ref(10)
const totalPages = computed(() => Math.ceil(totalPosts.value / pageSize.value))
const currentPage = ref(1)
let initialPage: number | null = null
const newPost = ref<UserCreatedForumPost>({
  content: '',
  forum_thread_id: 0,
})
const sendingPost = ref(false)
const bbcodeEditorEmptyInput = ref(false)
const siteName = import.meta.env.VITE_SITE_NAME

const editForumPostMethod = async (post: EditedForumPost) => {
  editForumPost(post)
}

const postEdited = (editedPost: EditedForumPost) => {
  const index = forumThreadPosts.value.findIndex((post) => post.id === editedPost.id)
  if (index !== -1) {
    forumThreadPosts.value[index] = { ...forumThreadPosts.value[index], ...editedPost }
    showToast('', t('forum.post_edited_success'), 'success', 2000)
  }
}

const fetchForumThreadPostsFromUrl = async () => {
  let page: number | null = 1
  if (route.query.page) {
    page = parseInt(route.query.page as string)
    initialPage = page
  } else if (route.query.post_id) {
    page = null
  }
  const post_id = route.query.post_id ? parseInt(route.query.post_id as string) : null
  const paginatedPosts = await getForumThreadsPosts({
    thread_id: parseInt(route.params.id as string),
    page: page,
    page_size: pageSize.value,
    post_id: post_id,
  })
  // emptying this variable resets the pagination
  forumThreadPosts.value.length = 0
  await nextTick()
  forumThreadPosts.value = paginatedPosts.results
  totalPosts.value = paginatedPosts.total_items
  await nextTick()
  if (post_id !== null) {
    initialPage = paginatedPosts.page
    scrollToHash()
  }
  currentPage.value = paginatedPosts.page
}

onMounted(async () => {
  ;[forumThread.value] = await Promise.all([getForumThread(+route.params.id!), fetchForumThreadPostsFromUrl()])

  document.title = forumThread.value ? `${forumThread.value.name} - ${siteName}` : `Forum thread - ${siteName}`
})

const resolver = () => {
  const errors: Partial<Record<keyof UserCreatedForumPost, { message: string }[]>> = {}

  if (newPost.value.content.length < 5) {
    errors.content = [{ message: t('error.write_more_than_x_chars', [5]) }]
  }

  return {
    errors,
  }
}

const onFormSubmit = ({ valid }: FormSubmitEvent) => {
  if (valid) {
    sendPost()
  }
}

const newPostUpdated = (content: string) => {
  newPost.value.content = content
}

const sendPost = async () => {
  if (!forumThread.value) {
    return
  }
  sendingPost.value = true
  newPost.value.forum_thread_id = parseInt(route.params.id as string)
  const createdPost: ForumPostHierarchy = {
    ...(await createForumPost(newPost.value)),
    created_by: userStore,
  }
  newPost.value.content = ''
  forumThreadPosts.value.push(createdPost)
  bbcodeEditorEmptyInput.value = true
  sendingPost.value = false
}

const toggleSubscribtion = async () => {
  if (forumThread.value) {
    togglingSubscription.value = true
    if (forumThread.value.is_subscribed) {
      await removeForumThreadPostsSubscription(parseInt(route.params.id.toString()))
    } else {
      await createForumThreadPostsSubscription(parseInt(route.params.id.toString()))
    }
    forumThread.value.is_subscribed = !forumThread.value.is_subscribed
    showToast('Success', t(`title_group.${forumThread.value.is_subscribed ? 'subscription_successful' : 'unsubscription_successful'}`), 'success', 3000)
    togglingSubscription.value = false
  }
}

const changePage = (page: number) => {
  currentPage.value = page
  router.push({ query: { page } })
}

const threadEdited = (editedThread: ForumThreadEnriched) => {
  if (forumThread.value) {
    forumThread.value = editedThread
    editThreadDialogVisible.value = false
    showToast('', t('forum.thread_edited_success'), 'success', 2000)
  }
}

watch(
  () => route.query,
  () => {
    fetchForumThreadPostsFromUrl()
  },
  { deep: true },
)
</script>

<style scoped>
.top-bar {
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
  .actions {
    cursor: pointer;
    i {
      margin-left: 7px;
    }
  }
}
.new-post {
  display: flex;
  flex-direction: column;
  margin-top: 30px;
  margin-bottom: 30px;
  align-items: flex-end;
}
.post-button {
  width: 5em;
  margin-top: 5px;
}
</style>
