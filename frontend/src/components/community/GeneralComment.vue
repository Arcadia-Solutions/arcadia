<template>
  <ContentContainer class="comment-container" :id="`post-${comment.id}`">
    <div class="comment-actions">
      <i v-if="canEdit" class="pi pi-pencil action-icon" @click="startEditing" v-tooltip.top="t('general.edit')" />
      <RouterLink
        :to="{
          query: { post_id: comment.id },
          hash: `#post-${comment.id}`,
        }"
      >
        <i class="pi pi-link action-icon" />
      </RouterLink>
    </div>
    <div class="comment">
      <div class="user">
        <img class="avatar" :src="comment.created_by.avatar ?? '/default_user_avatar.jpg'" :alt="comment.created_by.username + '\'s avatar'" />
        <UsernameEnriched :user="comment.created_by" />
        <span class="time-ago">
          {{ timeAgo(comment.created_at) }}
        </span>
        <span v-if="isEdited && localUpdatedAt" class="time-ago edited"> ({{ t('general.edited') }} {{ timeAgo(localUpdatedAt) }}) </span>
      </div>
      <div class="comment-body">
        <template v-if="!isEditing">
          <BBCodeRenderer :content="localContent" />
        </template>
        <template v-else>
          <BBCodeEditor :label="t('general.edit')" :initialValue="editedContent" @valueChange="(val: string) => (editedContent = val)">
            <template #buttons>
              <Button :label="t('general.cancel')" icon="pi pi-times" severity="secondary" @click="cancelEditing" />
              <Button :label="t('general.save')" icon="pi pi-check" :loading="isSaving" @click="saveEdit" />
            </template>
          </BBCodeEditor>
        </template>
      </div>
    </div>
  </ContentContainer>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import ContentContainer from '@/components/ContentContainer.vue'
import BBCodeRenderer from '@/components/community/BBCodeRenderer.vue'
import BBCodeEditor from '@/components/community/BBCodeEditor.vue'
import { Button } from 'primevue'
import { timeAgo } from '@/services/helpers'
import UsernameEnriched from '../user/UsernameEnriched.vue'
import { useUserStore } from '@/stores/user'
import { editTitleGroupComment, type ConversationMessageHierarchy, type ForumPostHierarchy, type TitleGroupCommentHierarchy } from '@/services/api-schema'

const props = defineProps<{
  comment: TitleGroupCommentHierarchy | ForumPostHierarchy | ConversationMessageHierarchy
}>()

const emit = defineEmits<{
  commentUpdated: [{ id: number | bigint; content: string }]
}>()

const { t } = useI18n()
const userStore = useUserStore()

const isTitleGroupComment = (comment: typeof props.comment): comment is TitleGroupCommentHierarchy => {
  return 'title_group_id' in comment
}

const isEditing = ref(false)
const editedContent = ref('')
const isSaving = ref(false)
const localContent = ref(props.comment.content)
const localUpdatedAt = ref(isTitleGroupComment(props.comment) ? props.comment.updated_at : null)

const EDIT_TIME_LIMIT_MS = 15 * 60 * 1000 // 15 minutes

const canEdit = computed(() => {
  if (!isTitleGroupComment(props.comment)) return false

  if (userStore.class === 'staff') return true

  const isOwner = userStore.id === props.comment.created_by.id
  if (!isOwner) return false

  const createdAt = new Date(props.comment.created_at).getTime()
  const now = Date.now()
  return now - createdAt <= EDIT_TIME_LIMIT_MS
})

const isEdited = computed(() => {
  if (!localUpdatedAt.value) return false
  return localUpdatedAt.value !== props.comment.created_at
})

const startEditing = () => {
  editedContent.value = localContent.value
  isEditing.value = true
}

const cancelEditing = () => {
  isEditing.value = false
  editedContent.value = ''
}

const saveEdit = async () => {
  isSaving.value = true
  try {
    if (isTitleGroupComment(props.comment)) {
      await editTitleGroupComment({
        id: Number(props.comment.id),
        EditedTitleGroupComment: { content: editedContent.value },
      })
      localContent.value = editedContent.value
      localUpdatedAt.value = new Date().toISOString()
    }
    emit('commentUpdated', { id: props.comment.id, content: editedContent.value })
    isEditing.value = false
  } finally {
    isSaving.value = false
  }
}
</script>

<style scoped>
.comment-container {
  margin-top: 10px;
}
.comment-actions {
  float: right;
  display: flex;
  gap: 8px;
}
.action-icon {
  cursor: pointer;
  opacity: 0.7;
  transition: opacity 0.2s;
}
.action-icon:hover {
  opacity: 1;
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
  flex: 1;
}
.time-ago {
  font-size: 0.8em;
  margin-top: 5px;
}
.time-ago.edited {
  font-style: italic;
  opacity: 0.8;
}
</style>
