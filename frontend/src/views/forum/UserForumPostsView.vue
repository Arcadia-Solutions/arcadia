<template>
  <UserCommentsList :searchComments="searchForumPosts" titleKey="forum.posts_of_user" :editSuccessMessage="t('forum.post_edited_success')">
    <template #location="{ comment: post }">
      <RouterLink to="/forum">{{ post.category_name }}</RouterLink>
      &gt;
      <RouterLink :to="`/forum/sub-category/${post.sub_category_id}`">{{ post.sub_category_name }}</RouterLink>
      &gt;
      <ForumThreadName :threadId="post.forum_thread_id" :threadName="post.thread_name" :postId="post.id" />
    </template>
    <template #default="{ comment: post, commentEdited, commentDeleted }">
      <GeneralComment
        :comment="post"
        :editCommentMethod="editForumPost"
        :deleteCommentMethod="deleteForumPost"
        @commentEdited="commentEdited(post.id, $event as EditedForumPost)"
        @commentDeleted="commentDeleted"
        :hasEditPermission="userStore.permissions.includes('edit_forum_post') || (post.created_by.id === userStore.id && !post.locked)"
        :hasDeletePermission="userStore.permissions.includes('delete_forum_post')"
      />
    </template>
  </UserCommentsList>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import GeneralComment from '@/components/community/GeneralComment.vue'
import UserCommentsList from '@/components/community/UserCommentsList.vue'
import ForumThreadName from '@/components/forum/ForumThreadName.vue'
import { deleteForumPost, editForumPost, searchForumPosts, type EditedForumPost } from '@/services/api-schema'
import { useUserStore } from '@/stores/user'

const userStore = useUserStore()
const { t } = useI18n()
</script>
