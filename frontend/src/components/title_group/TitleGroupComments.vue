<template>
  <div class="comments">
    <GeneralComment
      v-for="comment in comments"
      :key="comment.id"
      :comment="comment"
      @commentEdited="commentEdited($event, comment.id)"
      :editCommentMethod="(post: EditedTitleGroupComment) => editTitleGroupComment({ EditedTitleGroupComment: post, id: comment.id })"
    />
  </div>
  <Form v-slot="$form" :initialValues="new_comment" :resolver @submit="onFormSubmit" validateOnSubmit :validateOnValueUpdate="false">
    <div class="new-comment">
      <BBCodeEditor
        :empty-input="bbcodeEditorEmptyInput"
        @value-change="newCommentUpdated"
        @input-emptied="bbcodeEditorEmptyInput = false"
        :label="t('community.new_comment')"
      >
        <template #buttons>
          <Button type="submit" label="Post" icon="pi pi-send" :loading="sending_comment" class="post-button" />
        </template>
      </BBCodeEditor>
      <Message v-if="$form.content?.invalid" severity="error" size="small" variant="simple">
        {{ $form.content.error?.message }}
      </Message>
    </div>
  </Form>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { ref } from 'vue'
import GeneralComment from '../community/GeneralComment.vue'
import { Button } from 'primevue'
import BBCodeEditor from '../community/BBCodeEditor.vue'
import { Form, type FormResolverOptions, type FormSubmitEvent } from '@primevue/forms'
import Message from 'primevue/message'
import { useUserStore } from '@/stores/user'
import { useRoute } from 'vue-router'
import {
  createTitleGroupComment,
  editTitleGroupComment,
  type EditedTitleGroupComment,
  type TitleGroupCommentHierarchy,
  type UserCreatedTitleGroupComment,
} from '@/services/api-schema'

defineProps<{
  comments: TitleGroupCommentHierarchy[]
}>()

const emit = defineEmits<{
  newComment: [TitleGroupCommentHierarchy]
  commentEdited: [EditedTitleGroupComment, number]
}>()

const { t } = useI18n()

const route = useRoute()

const new_comment = ref<UserCreatedTitleGroupComment>({
  content: '',
  refers_to_torrent_id: null,
  answers_to_comment_id: null,
  title_group_id: 0,
})
const sending_comment = ref(false)
const bbcodeEditorEmptyInput = ref(false)

const commentEdited = (editedComment: EditedTitleGroupComment, commentId: number) => {
  emit('commentEdited', editedComment, commentId)
}

const resolver = ({ values }: FormResolverOptions) => {
  const errors: Partial<Record<keyof UserCreatedTitleGroupComment, { message: string }[]>> = {}

  if (values.content.length < 5) {
    errors.content = [{ message: t('error.write_more_than_x_chars', [5]) }]
  }

  return {
    errors,
  }
}

const onFormSubmit = ({ valid }: FormSubmitEvent) => {
  if (valid) {
    sendComment()
  }
}

const newCommentUpdated = (content: string) => {
  new_comment.value.content = content
}

const sendComment = async () => {
  sending_comment.value = true
  new_comment.value.title_group_id = parseInt(route.params.id as string)
  const createdComment: TitleGroupCommentHierarchy = {
    ...(await createTitleGroupComment(new_comment.value)),
    created_by: useUserStore(),
  }
  new_comment.value.content = ''
  new_comment.value.refers_to_torrent_id = null
  new_comment.value.answers_to_comment_id = null
  emit('newComment', createdComment)
  bbcodeEditorEmptyInput.value = true
  sending_comment.value = false
}
</script>
<style scoped>
.new-comment {
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
