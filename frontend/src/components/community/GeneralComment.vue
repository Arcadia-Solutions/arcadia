<template>
  <ContentContainer class="comment-container" :id="`post-${comment.id}`">
    <div class="actions">
      <i
        class="pi pi-pen-to-square"
        v-if="(userStore.id === comment.created_by.id && 'locked' in comment && comment.locked === false) || userStore.class === 'staff'"
        @click="editCommentDialogVisible = true"
      />
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
  <!-- Only comments that containt the 'locked' key can be edited -->
  <Dialog closeOnEscape modal v-model:visible="editCommentDialogVisible" v-if="'locked' in comment">
    <EditCommentDialog :initialComment="comment" @commentEdited="updateComment" />
  </Dialog>
</template>

<script setup lang="ts">
import ContentContainer from '@/components/ContentContainer.vue'
import BBCodeRenderer from '@/components/community/BBCodeRenderer.vue'
import { timeAgo } from '@/services/helpers'
import UsernameEnriched from '../user/UsernameEnriched.vue'
import type { ConversationMessageHierarchy, EditedForumPost, ForumPostHierarchy, TitleGroupCommentHierarchy } from '@/services/api-schema'
import { useUserStore } from '@/stores/user'
import { Dialog } from 'primevue'
import EditCommentDialog from './EditCommentDialog.vue'
import { ref } from 'vue'

const props = defineProps<{
  comment: TitleGroupCommentHierarchy | ForumPostHierarchy | ConversationMessageHierarchy
  editCommentMethod?: (comment: EditedForumPost) => Promise<void>
}>()

const emit = defineEmits<{
  commentEdited: [EditedForumPost]
}>()

const userStore = useUserStore()
const editCommentDialogVisible = ref(false)
const loadingUpdatingComment = ref(false)

const updateComment = async (comment: EditedForumPost) => {
  if (!props.editCommentMethod) return
  loadingUpdatingComment.value = true
  props
    .editCommentMethod(comment)
    .then(() => {
      emit('commentEdited', comment)
      editCommentDialogVisible.value = false
    })
    .finally(() => (loadingUpdatingComment.value = false))
}
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
.actions {
  float: right;
  i {
    margin-left: 7px;
    cursor: pointer;
  }
}
.comment-body {
  padding: 7px;
}
.time-ago {
  font-size: 0.8em;
  margin-top: 5px;
}
</style>
