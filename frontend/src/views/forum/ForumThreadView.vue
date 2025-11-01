<template>
  <div v-if="forumThread">
    <div class="title">
      <RouterLink to="/forum">{{ forumThread.forum_category_name }}</RouterLink> >
      <RouterLink :to="`/forum/sub-category/${forumThread.forum_sub_category_id}`">{{ forumThread.forum_sub_category_name }}</RouterLink> >
      {{ forumThread.name }}
    </div>
    <PaginatedResults :totalItems="totalPosts" @change-page="fetchForumThreadPosts($event.page, $event.pageSize)" :page-size="pageSize">
      <GeneralComment v-for="post in forumThreadPosts" :key="post.id" :comment="post" />
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
            <Button type="submit" label="Post" icon="pi pi-send" :loading="sendingPost" class="post-button" />
          </template>
        </BBCodeEditor>
        <Message v-if="$form.content?.invalid" severity="error" size="small" variant="simple">
          {{ $form.content.error?.message }}
        </Message>
      </div>
    </Form>
  </div>
</template>

<script setup lang="ts">
import {
  getForumThread,
  postForumPost,
  type UserCreatedForumPost,
  type ForumPostHierarchy,
  type ForumThreadEnriched,
  getForumThreadPosts,
} from '@/services/api/forumService'
import { onMounted } from 'vue'
import { ref } from 'vue'
import { useRoute } from 'vue-router'
import GeneralComment from '@/components/community/GeneralComment.vue'
import type { FormSubmitEvent } from '@primevue/forms'
import { useI18n } from 'vue-i18n'
import { useUserStore } from '@/stores/user'
import { Form } from '@primevue/forms'
import { Button } from 'primevue'
import BBCodeEditor from '@/components/community/BBCodeEditor.vue'
import PaginatedResults from '@/components/PaginatedResults.vue'

const route = useRoute()
const { t } = useI18n()

const forumThread = ref<null | ForumThreadEnriched>(null)
const forumThreadPosts = ref<ForumPostHierarchy[]>([])
const totalPosts = ref(0)
const pageSize = ref(10)
const newPost = ref<UserCreatedForumPost>({
  content: '',
  forum_thread_id: 0,
})
const sendingPost = ref(false)
const bbcodeEditorEmptyInput = ref(false)
const siteName = import.meta.env.VITE_SITE_NAME

const fetchForumThreadPosts = async (page: number, page_size: number) => {
  const paginatedPosts = await getForumThreadPosts(parseInt(route.params.id as string), page, page_size)
  forumThreadPosts.value = paginatedPosts.results
  totalPosts.value = paginatedPosts.total_items
}

onMounted(async () => {
  ;[forumThread.value] = await Promise.all([getForumThread(+route.params.id!), fetchForumThreadPosts(1, pageSize.value)])

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
    ...(await postForumPost(newPost.value)),
    created_by: useUserStore(),
  }
  newPost.value.content = ''
  forumThreadPosts.value.push(createdPost)
  bbcodeEditorEmptyInput.value = true
  sendingPost.value = false
}
</script>

<style scoped>
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
