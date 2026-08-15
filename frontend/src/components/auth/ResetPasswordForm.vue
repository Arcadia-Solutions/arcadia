<template>
  <Form :initialValues="form" :resolver @submit="submit" validateOnSubmit :validateOnValueUpdate="false" validateOnBlur class="form" v-slot="$form">
    <Message v-if="!token" severity="error" size="small">{{ t('auth.reset_password_token_missing') }}</Message>
    <template v-else>
      <Password class="form-item" name="new_password" v-model="form.new_password" :placeholder="t('user.new_password')" toggleMask />
      <Message v-if="$form.new_password?.invalid" severity="error" size="small" variant="simple">
        {{ $form.new_password.error?.message }}
      </Message>
      <Password
        class="form-item"
        name="new_password_verify"
        v-model="form.new_password_verify"
        :placeholder="t('user.new_password_verify')"
        :feedback="false"
        toggleMask
      />
      <Message v-if="$form.new_password_verify?.invalid" severity="error" size="small" variant="simple">
        {{ $form.new_password_verify.error?.message }}
      </Message>
      <Button class="form-item w-full" type="submit" severity="secondary" size="small" :label="t('auth.reset_password')" :loading />
    </template>
  </Form>
</template>
<script setup lang="ts">
import { Message, Password } from 'primevue'
import { Form, type FormResolverOptions, type FormSubmitEvent } from '@primevue/forms'
import Button from 'primevue/button'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute, useRouter } from 'vue-router'
import { showToast } from '@/main'
import { resetPassword } from '@/services/api-schema'
import { validatePasswordStrength } from '@/services/helpers'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()

const token = String(route.query.token ?? '')
const form = ref({
  new_password: '',
  new_password_verify: '',
})
const loading = ref(false)

const resolver = ({ values }: FormResolverOptions) => {
  const errors: Record<string, { message: string }[]> = {}

  const passwordValidation = validatePasswordStrength(values.new_password, t)
  if (!passwordValidation.isValid) {
    errors.new_password = [{ message: passwordValidation.message }]
  }

  if (values.new_password !== values.new_password_verify) {
    errors.new_password_verify = [{ message: t('auth_validation.password_mismatch') }]
  }

  return { errors }
}

const submit = ({ valid }: FormSubmitEvent) => {
  if (!valid) {
    return
  }
  loading.value = true
  resetPassword({
    token,
    new_password: form.value.new_password,
    new_password_verify: form.value.new_password_verify,
  })
    .then(() => {
      showToast('', t('auth.password_reset_success'), 'success', 4000)
      router.push('/login')
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
}
</style>
