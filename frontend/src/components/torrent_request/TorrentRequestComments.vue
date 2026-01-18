<template>
  <div class="comments">
    <GeneralComment v-for="comment in comments" :key="comment.id" :comment="comment" :hasEditPermission="false" />
  </div>
  <Form v-slot="$form" :initialValues="new_comment" :resolver @submit="onFormSubmit" validateOnSubmit :validateOnValueUpdate="false">
    <div class="new-comment">
      <BBCodeEditor
        :empty-input="bbcodeEditorEmptyInput"
        @value-change="newCommentUpdated"
        @input-emptied="bbcodeEditorEmptyInput = false"
        :label="t('community.new_comment')"
        name="content"
      >
        <template #buttons>
          <Button type="submit" label="Post" icon="pi pi-send" :loading="sending_comment" class="post-button" />
        </template>
        <template #message>
          <Message v-if="$form.content?.invalid" severity="error" size="small" variant="simple">
            {{ $form.content.error?.message }}
          </Message>
        </template>
      </BBCodeEditor>
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
import { createTorrentRequestComment, type TorrentRequestCommentHierarchy, type UserCreatedTorrentRequestComment } from '@/services/api-schema'

defineProps<{
  comments: TorrentRequestCommentHierarchy[]
}>()

const emit = defineEmits<{
  newComment: [TorrentRequestCommentHierarchy]
}>()

const { t } = useI18n()

const route = useRoute()

const new_comment = ref<UserCreatedTorrentRequestComment>({
  content: '',
  torrent_request_id: 0,
})
const sending_comment = ref(false)
const bbcodeEditorEmptyInput = ref(false)

const resolver = ({ values }: FormResolverOptions) => {
  const errors: Partial<Record<keyof UserCreatedTorrentRequestComment, { message: string }[]>> = {}

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

const sendComment = () => {
  sending_comment.value = true
  new_comment.value.torrent_request_id = parseInt(route.params.id as string)
  createTorrentRequestComment(new_comment.value)
    .then((createdComment) => {
      const commentHierarchy: TorrentRequestCommentHierarchy = {
        ...createdComment,
        created_by: useUserStore(),
      }
      new_comment.value.content = ''
      emit('newComment', commentHierarchy)
      bbcodeEditorEmptyInput.value = true
    })
    .finally(() => (sending_comment.value = false))
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
