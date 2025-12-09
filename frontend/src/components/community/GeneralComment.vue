<template>
  <ContentContainer class="comment-container" :id="`post-${comment.id}`">
    <div style="float: right">
      <RouterLink
        :to="{
          query: { post_id: comment.id },
          hash: `#post-${comment.id}`,
        }"
      >
        <i class="pi pi-link" />
      </RouterLink>
    </div>
    <div class="comment">
      <div class="user">
        <img class="avatar" :src="comment.created_by.avatar ?? '/default_user_avatar.jpg'" :alt="comment.created_by.username + '\'s avatar'" />
        <UsernameEnriched :user="comment.created_by" />
        <span class="time-ago">
          {{ timeAgo(comment.created_at) }}
        </span>
      </div>
      <div class="comment-body">
        <BBCodeRenderer :content="comment.content" />
      </div>
    </div>
  </ContentContainer>
</template>

<script setup lang="ts">
import ContentContainer from '@/components/ContentContainer.vue'
import BBCodeRenderer from '@/components/community/BBCodeRenderer.vue'
import { timeAgo } from '@/services/helpers'
import UsernameEnriched from '../user/UsernameEnriched.vue'
import type { ConversationMessageHierarchy, ForumPostHierarchy, TitleGroupCommentHierarchy } from '@/services/api-schema'

defineProps<{
  comment: TitleGroupCommentHierarchy | ForumPostHierarchy | ConversationMessageHierarchy
}>()
</script>

<style scoped>
.comment-container {
  margin-top: 10px;
}
.comment {
  display: flex;
  align-items: flex-start;
}
.user {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 9px;
  background-color: var(--color-background-primary);
  border-radius: 7px;
}
.avatar {
  width: 9em;
  border-radius: 7px;
}
.comment-body {
  padding: 7px;
}
.time-ago {
  font-size: 0.8em;
  margin-top: 5px;
}
</style>
