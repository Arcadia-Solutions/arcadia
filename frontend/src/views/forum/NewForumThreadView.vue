<template>
  <div class="title">{{ t('forum.new_thread') }}</div>
  <Form v-slot="$form" :initialValues="newThread" :resolver @submit="sendThread" validateOnSubmit :validateOnValueUpdate="false" validateOnBlur>
    <FloatLabel class="thread-name" variant="in">
      <InputText v-model="newThread.name" name="name" :format="false" />
      <label for="master_group_id">{{ t('forum.thread_name') }}</label>
    </FloatLabel>
    <Message v-if="$form.name?.invalid" severity="error" size="small" variant="simple">
      {{ $form.name.error.message }}
    </Message>
    <div class="bbcode-editor">
      <BBCodeEditor :label="t('forum.new_post')" :emptyInput="false" @valueChange="(val) => (newThread.first_post.content = val)">
        <template #message>
          <Message v-if="$form.content?.invalid" severity="error" size="small" variant="simple">
            {{ $form.content.error.message }}
          </Message>
        </template>
        <template #buttons>
          <Button type="submit" label="Post" icon="pi pi-send" :loading="sendingThread" />
        </template>
      </BBCodeEditor>
    </div>
  </Form>
</template>

<script setup lang="ts">
import { FloatLabel, InputText, Button, Message } from 'primevue'
import { Form, type FormResolverOptions, type FormSubmitEvent } from '@primevue/forms'
import BBCodeEditor from '@/components/community/BBCodeEditor.vue'
import { useI18n } from 'vue-i18n'
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useRoute } from 'vue-router'
import { createForumThread, type UserCreatedForumThread } from '@/services/api-schema'

const { t } = useI18n()
const router = useRouter()
const route = useRoute()

const newThread = ref<UserCreatedForumThread>({
  first_post: { content: '', forum_thread_id: 0 },
  name: '',
  forum_sub_category_id: 0,
})
const sendingThread = ref(false)

const resolver = ({ values }: FormResolverOptions) => {
  const errors = { name: {}, content: {} }

  if (values.name.length < 5) {
    errors.name = [{ message: t('error.write_more_than_x_chars', [5]) }]
  }
  if (newThread.value.first_post.content.length < 5) {
    errors.content = [{ message: t('error.write_more_than_x_chars', [5]) }]
  }

  return {
    errors,
  }
}

const sendThread = async ({ valid }: FormSubmitEvent) => {
  if (valid) {
    sendingThread.value = true
    newThread.value.forum_sub_category_id = parseInt(route.query.subCategoryId as string)
    createForumThread(newThread.value)
      .then((createdThread) => {
        router.push(`/forum/thread/${createdThread.id}`)
      })
      .finally(() => {
        sendingThread.value = false
      })
  }
}
</script>

<style scoped>
.title {
  margin-bottom: 10px;
}
.thread-name {
  .p-inputtext {
    width: 100%;
  }
}
.bbcode-editor {
  margin-top: 15px;
}
</style>
