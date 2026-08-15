<template>
  <UserCommentsList :searchComments="searchUserTorrentRequestComments" titleKey="torrent_request.comments_of_user">
    <template #location="{ comment }">
      <RouterLink :to="`/title-group/${comment.title_group_id}`">{{ comment.title_group_name }}</RouterLink>
      <RouterLink :to="`/torrent-request/${comment.torrent_request_id}?post_id=${comment.id}#post-${comment.id}`">
        <i class="pi pi-arrow-right" />
      </RouterLink>
    </template>
    <template #default="{ comment, commentDeleted }">
      <GeneralComment
        :comment="comment"
        :deleteCommentMethod="deleteTorrentRequestComment"
        @commentDeleted="commentDeleted"
        :hasEditPermission="false"
        :hasDeletePermission="userStore.permissions.includes('delete_torrent_request_comment')"
      />
    </template>
  </UserCommentsList>
</template>

<script setup lang="ts">
import GeneralComment from '@/components/community/GeneralComment.vue'
import UserCommentsList from '@/components/community/UserCommentsList.vue'
import { deleteTorrentRequestComment, searchUserTorrentRequestComments } from '@/services/api-schema'
import { useUserStore } from '@/stores/user'

const userStore = useUserStore()
</script>
