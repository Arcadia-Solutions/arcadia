<template>
  <div class="title">{{ t('staff_pm.new') }}</div>
  <Form v-slot="$form" :initialValues="newStaffPM" :resolver @submit="sendStaffPm" validateOnSubmit :validateOnValueUpdate="false" validateOnBlur>
    <FloatLabel class="staff-pm-subject" variant="in">
      <InputText v-model="newStaffPM.subject" name="subject" :format="false" />
      <label for="master_group_id">{{ t('staff_pm.subject') }}</label>
    </FloatLabel>
    <Message v-if="$form.subject?.invalid" severity="error" size="small" variant="simple">
      {{ $form.subject.error.message }}
    </Message>
    <div class="bbcode-editor">
      <BBCodeEditor :label="t('staff_pm.message')" :emptyInput="false" @valueChange="(val) => (newStaffPM.first_message.content = val)">
        <template #message>
          <Message v-if="$form.content?.invalid" severity="error" size="small" variant="simple">
            {{ $form.content.error.message }}
          </Message>
        </template>
        <template #buttons>
          <Button type="submit" label="Post" icon="pi pi-send" :loading="sendingStaffPM" />
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
import { createStaffPM, type UserCreatedStaffPm } from '@/services/api-schema'

const { t } = useI18n()
const router = useRouter()

const newStaffPM = ref<UserCreatedStaffPm>({
  first_message: { content: '', staff_pm_id: 0 },
  subject: '',
})
const sendingStaffPM = ref(false)

const resolver = ({ values }: FormResolverOptions) => {
  const errors = { subject: {}, content: {} }

  if (values.subject.length < 5) {
    errors.subject = [{ message: t('error.write_more_than_x_chars', [5]) }]
  }
  if (newStaffPM.value.first_message.content.length < 5) {
    errors.content = [{ message: t('error.write_more_than_x_chars', [5]) }]
  }

  return {
    errors,
  }
}

const sendStaffPm = async ({ valid }: FormSubmitEvent) => {
  if (valid) {
    sendingStaffPM.value = true
    createStaffPM(newStaffPM.value)
      .then((createdStaffPm) => {
        router.push(`/staff-pm/${createdStaffPm.id}`)
      })
      .finally(() => {
        sendingStaffPM.value = false
      })
  }
}
</script>

<style scoped>
.title {
  margin-bottom: 10px;
}
.staff-pm-subject {
  .p-inputtext {
    width: 100%;
  }
}
.bbcode-editor {
  margin-top: 15px;
}
</style>
