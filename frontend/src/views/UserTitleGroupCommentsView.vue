<template>
  <UserCommentsList
    :searchComments="searchUserTitleGroupComments"
    titleKey="title_group.comments_of_user"
    :editSuccessMessage="t('title_group.comment_edited_success')"
  >
    <template #location="{ comment }">
      <RouterLink :to="`/title-group/${comment.title_group_id}`">{{ comment.title_group_name }}</RouterLink>
      <RouterLink :to="`/title-group/${comment.title_group_id}?post_id=${comment.id}#post-${comment.id}`">
        <i class="pi pi-arrow-right" />
      </RouterLink>
    </template>
    <template #default="{ comment, commentEdited, commentDeleted }">
      <GeneralComment
        :comment="comment"
        :editCommentMethod="(editedComment: EditedTitleGroupComment) => editTitleGroupComment({ EditedTitleGroupComment: editedComment, id: comment.id })"
        :deleteCommentMethod="deleteTitleGroupComment"
        @commentEdited="commentEdited(comment.id, $event as EditedTitleGroupComment)"
        @commentDeleted="commentDeleted"
        :hasEditPermission="userStore.permissions.includes('edit_title_group_comment') || (comment.created_by.id === userStore.id && !comment.locked)"
        :hasDeletePermission="userStore.permissions.includes('delete_title_group_comment')"
      />
    </template>
  </UserCommentsList>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import GeneralComment from '@/components/community/GeneralComment.vue'
import UserCommentsList from '@/components/community/UserCommentsList.vue'
import { deleteTitleGroupComment, editTitleGroupComment, searchUserTitleGroupComments, type EditedTitleGroupComment } from '@/services/api-schema'
import { useUserStore } from '@/stores/user'

const userStore = useUserStore()
const { t } = useI18n()
</script>
