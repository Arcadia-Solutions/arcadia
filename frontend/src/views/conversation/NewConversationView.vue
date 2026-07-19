<template>
  <div class="title">{{ isMassConversation ? t('conversation.mass_message') : t('conversation.start_conversation', [username]) }}</div>
  <Message v-if="isMassConversation" severity="info" size="small" class="mass-message-hint">
    {{ t('conversation.mass_message_placeholder_hint') }}
  </Message>
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
          <Button type="submit" v-tooltip.top="'Post'" icon="pi pi-send" :loading="sendingConversation" />
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
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { useRoute } from 'vue-router'
import { onMounted } from 'vue'
import { createConversation, createMassConversation, type UserCreatedConversation } from '@/services/api-schema'
import { showToast } from '@/main'

const { t } = useI18n()
const router = useRouter()
const route = useRoute()

// When opened from the user search "message all matching users" action, the page sends
// a single message to every user matching the search filter instead of a single receiver.
const isMassConversation = computed(() => route.query.mass === 'true')

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
  if (!valid) {
    return
  }
  sendingConversation.value = true

  if (isMassConversation.value) {
    createMassConversation({
      username: (route.query.username as string) || undefined,
      registered_after: (route.query.registered_after as string) || undefined,
      registered_before: (route.query.registered_before as string) || undefined,
      subject: newConversation.value.subject,
      message: newConversation.value.first_message.content,
    })
      .then((result) => {
        showToast(t('conversation.mass_message'), t('user.mass_pm_success', [result.messages_sent]), 'success')
        router.push('/users')
      })
      .finally(() => {
        sendingConversation.value = false
      })
    return
  }

  newConversation.value.receiver_id = parseInt(route.query.receiverId as string)
  createConversation(newConversation.value)
    .then((createdConversation) => {
      router.push(`/conversation/${createdConversation.id}`)
    })
    .finally(() => {
      sendingConversation.value = false
    })
}
onMounted(() => {
  username.value = route.query.username as string
})
</script>

<style scoped>
.title {
  margin-bottom: 10px;
}
.mass-message-hint {
  margin-bottom: 15px;
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
