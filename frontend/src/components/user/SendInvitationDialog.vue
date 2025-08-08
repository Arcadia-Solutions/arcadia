<template>
  <div class="invitation-dialog">
    <div class="form" v-if="!createdInvitation">
      <FloatLabel>
        <InputText class="input" name="email" v-model="invitation.receiver_email" />
        <label for="email">{{ t('user.email') }}</label>
      </FloatLabel>
      <FloatLabel>
        <Textarea class="input" name="body" type="text" rows="7" v-model="invitation.message" />
        <label for="body">{{ t('user.invitation_message') }}</label>
      </FloatLabel>
      <div class="wrapper-center">
        <Button :label="t('user.send_invitation')" size="small" severity="success" @click="sendNewInvitation" :loading />
      </div>
    </div>
    <div v-else class="invitation-sent">
      Invitation sent! Registration link:
      <br />
      <span class="link">
        {{ `${domain}/register?invitation_key=${createdInvitation?.invitation_key}` }}
      </span>
    </div>
  </div>
</template>
<script setup lang="ts">
import { sendInvitation, type SentInvitation, type Invitation } from '@/services/api/userService'
import { Textarea } from 'primevue'
import { FloatLabel, Button } from 'primevue'
import { onMounted } from 'vue'
import { ref } from 'vue'
import { InputText } from 'primevue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps<{
  receiverEmail: string
  applicationId?: number
}>()

const emit = defineEmits<{
  invitationSent: []
}>()

const loading = ref(false)

const invitation = ref<SentInvitation>({
  message: '',
  receiver_email: '',
})

const domain = window.location.hostname

const createdInvitation = ref<Invitation>()

const sendNewInvitation = () => {
  loading.value = true
  sendInvitation(invitation.value)
    .then((data) => {
      createdInvitation.value = data
      emit('invitationSent')
    })
    .finally(() => {
      loading.value = false
    })
}

onMounted(() => {
  invitation.value.receiver_email = props.receiverEmail
  if (props.applicationId) {
    invitation.value.user_application_id = props.applicationId
  }
})
</script>

<style scoped>
.invitation-dialog {
  display: flex;
  justify-content: center;
}
.p-button {
  margin-top: 10px;
}
.p-floatlabel {
  margin-top: 25px;
}
.p-textarea {
  width: 30vw;
}
.invitation-sent {
  text-align: center;
  .link {
    font-weight: bold;
  }
}
</style>
