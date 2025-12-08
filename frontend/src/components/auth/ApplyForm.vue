<template>
  <Form :initialValues="form" @submit="handleLogin" class="form" v-if="!applicationSent">
    <InputText class="form-item" name="email" type="text" :placeholder="t('user.email')" v-model="form.email" />
    <Textarea class="form-item" name="body" type="text" rows="15" :placeholder="t('user.application_body')" v-model="form.body" />
    <Button class="form-item w-full" type="submit" severity="secondary" :label="t('user.apply')" :loading />
  </Form>
  <span style="margin-top: 10px" v-else>{{ t('user.application_sent') }}</span>
</template>
<script setup lang="ts">
import InputText from 'primevue/inputtext'
import { Textarea } from 'primevue'
import { Form } from '@primevue/forms'
import Button from 'primevue/button'
import { useI18n } from 'vue-i18n'
import { ref } from 'vue'
import { createUserApplication, type UserCreatedUserApplication } from '@/services/api-schema'

const form = ref<UserCreatedUserApplication>({
  email: '',
  body: '',
  referral: '',
})

const loading = ref(false)
const applicationSent = ref(false)
const { t } = useI18n()

const handleLogin = async () => {
  loading.value = true
  createUserApplication(form.value)
    .then(() => {
      form.value.email = ''
      form.value.body = ''
      applicationSent.value = true
    })
    .finally(() => {
      loading.value = false
    })
}
</script>
<style scoped>
.form {
  display: flex;
  flex-direction: column;
  width: 70%;
}
</style>
