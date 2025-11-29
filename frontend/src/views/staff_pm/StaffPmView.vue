<template>
  <div v-if="staffPm">
    <div class="title">{{ staffPm.subject }}</div>
    <ConversationMessages :messages="staffPm.messages" />
    <Form @submit="sendMessage">
      <BBCodeEditor
        :label="t('conversation.message')"
        :emptyInput
        @valueChange="(val: string) => (newMessage.content = val)"
        @inputEmptied="emptyInput = false"
      >
        <template #buttons>
          <Button v-if="!staffPm.resolved" :label="t('staff_pm.resolve')" icon="pi pi-check" :loading="resolvingPm" @click="resolvePm" />
          <Button type="submit" :label="t('general.send')" icon="pi pi-send" :loading="sendingMessage" />
        </template>
      </BBCodeEditor>
    </Form>
  </div>
</template>

<script lang="ts" setup>
import { Form } from '@primevue/forms'
import { onMounted } from 'vue'
import { ref } from 'vue'
import { useRoute } from 'vue-router'
import ConversationMessages from '@/components/conversation/conversationMessages.vue'
import BBCodeEditor from '@/components/community/BBCodeEditor.vue'
import { Button } from 'primevue'
import { useI18n } from 'vue-i18n'
import { useUserStore } from '@/stores/user'
import { getStaffPm, type StaffPmHierarchy } from '@/services/api/staffPmService'
import type { UserCreatedStaffPmMessage } from '@/services/api/staffPmService'
import { postStaffPmMessage } from '@/services/api/staffPmService'
import { resolveStaffPm } from '@/services/api/staffPmService'
import { showToast } from '@/main'

const route = useRoute()
const { t } = useI18n()
const userStore = useUserStore()

const staffPm = ref<StaffPmHierarchy>()
const sendingMessage = ref(false)
const resolvingPm = ref(false)
const newMessage = ref<UserCreatedStaffPmMessage>({
  content: '',
  staff_pm_id: 0,
})
const emptyInput = ref(false)
const siteName = import.meta.env.VITE_SITE_NAME

const fetchConversation = async (staffPmId: number) => {
  getStaffPm(staffPmId).then((c) => {
    staffPm.value = c
    document.title = `${c.subject} - ${siteName}`
  })
}

const resolvePm = async () => {
  resolvingPm.value = true
  resolveStaffPm(parseInt(route.params.id as string))
    .then(() => {
      if (staffPm.value) {
        staffPm.value.resolved = true
      }
      showToast('', t('staff_pm.resolved_successfully'), 'success', 3000, true, 'tr')
    })
    .finally(() => {
      resolvingPm.value = false
    })
}

const sendMessage = async () => {
  sendingMessage.value = true
  newMessage.value.staff_pm_id = parseInt(route.params.id as string)
  postStaffPmMessage(newMessage.value)
    .then((message) => {
      staffPm.value?.messages.push({ ...message, created_by: userStore })
      emptyInput.value = true
    })
    .finally(() => {
      sendingMessage.value = false
    })
}

onMounted(() => {
  fetchConversation(parseInt(route.params.id as string))
})
</script>
<style scoped>
.messages {
  margin-bottom: 15px;
}
</style>
