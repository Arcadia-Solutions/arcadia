<template>
  <div class="title">{{ t('conversation.start_conversation', [username]) }}</div>
  <Form v-slot="$form" :initialValues="newConversation" :resolver @submit="sendConversation" validateOnSubmit :validateOnValueUpdate="false" validateOnBlur>
    <FloatLabel class="conversation-subject" variant="in">
      <InputText v-model="newConversation.subject" name="subject" :format="false" />
      <label for="master_group_id">{{ t('conversation.subject') }}</label>
    </FloatLabel>
    <Message v-if="$form.subject?.invalid" severity="error" size="small" variant="simple">
      {{ $form.subject.error.message }}
    </Message>
    <div class="bbcode-editor">
      <BBCodeEditor :label="t('conversation.message')" :emptyInput="false" @valueChange="(val) => (newConversation.first_message.content = val)">
        <template #message>
          <Message v-if="$form.content?.invalid" severity="error" size="small" variant="simple">
            {{ $form.content.error.message }}
          </Message>
        </template>
        <template #buttons>
          <Button type="submit" label="Post" icon="pi pi-send" :loading="sendingConversation" />
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
import { onMounted } from 'vue'
import { createConversation, type UserCreatedConversation } from '@/services/api-schema'

const { t } = useI18n()
const router = useRouter()
const route = useRoute()

const username = ref('')
const newConversation = ref<UserCreatedConversation>({
  first_message: { content: '', conversation_id: 0 },
  subject: '',
  receiver_id: 0,
})
const sendingConversation = ref(false)

const resolver = ({ values }: FormResolverOptions) => {
  const errors = { subject: {}, content: {} }

  if (values.subject.length < 5) {
    errors.subject = [{ message: t('error.write_more_than_x_chars', [5]) }]
  }
  if (newConversation.value.first_message.content.length < 5) {
    errors.content = [{ message: t('error.write_more_than_x_chars', [5]) }]
  }

  return {
    errors,
  }
}

const sendConversation = async ({ valid }: FormSubmitEvent) => {
  if (valid) {
    sendingConversation.value = true
    newConversation.value.receiver_id = parseInt(route.query.receiverId as string)
    createConversation(newConversation.value)
      .then((createdConversation) => {
        router.push(`/conversation/${createdConversation.id}`)
      })
      .finally(() => {
        sendingConversation.value = false
      })
  }
}
onMounted(() => {
  username.value = route.query.username as string
})
</script>

<style scoped>
.title {
  margin-bottom: 10px;
}
.conversation-subject {
  .p-inputtext {
    width: 100%;
  }
}
.bbcode-editor {
  margin-top: 15px;
}
</style>
