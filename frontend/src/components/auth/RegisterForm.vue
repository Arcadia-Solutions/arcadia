<template>
  <Form
    class="form"
    v-slot="$form"
    :initialValues="form"
    :resolver="resolver"
    @submit="handleRegister"
    validateOnSubmit
    :validateOnValueUpdate="false"
    validateOnBlur
    ref="formRef"
  >
    <InputText 
      class="form-item" 
      name="email" 
      type="email" 
      :placeholder="t('user.email')" 
      v-model="form.email" 
    />
    <Message v-if="$form.email?.invalid" severity="error" size="small" variant="simple">
      {{ $form.email.error?.message }}
    </Message>

    <InputText 
      class="form-item" 
      name="username" 
      type="text" 
      :placeholder="t('user.username')" 
      v-model="form.username" 
    />
    <Message v-if="$form.username?.invalid" severity="error" size="small" variant="simple">
      {{ $form.username.error?.message }}
    </Message>

    <Password 
      class="form-item" 
      name="password" 
      :placeholder="t('user.password')" 
      v-model="form.password" 
      :feedback="true" 
      toggleMask 
    />
    <Message v-if="$form.password?.invalid" severity="error" size="small" variant="simple">
      {{ $form.password.error?.message }}
    </Message>

    <Password 
      class="form-item" 
      name="password_verify" 
      :placeholder="t('user.password_verify')" 
      v-model="form.password_verify" 
      :feedback="false" 
      toggleMask 
    />
    <Message v-if="$form.password_verify?.invalid" severity="error" size="small" variant="simple">
      {{ $form.password_verify.error?.message }}
    </Message>

    <Button 
      class="form-item w-full" 
      type="submit" 
      severity="secondary" 
      :loading="loading" 
      :label="t('user.register')" 
    />
  </Form>
</template>

<script setup lang="ts">
import InputText from 'primevue/inputtext'
import Password from 'primevue/password'
import Button from 'primevue/button'
import { Form, type FormResolverOptions, type FormSubmitEvent } from '@primevue/forms'

import Message from 'primevue/message'
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { register } from '@/services/api/authService'
import { useI18n } from 'vue-i18n'
import { useRoute } from 'vue-router'

const { t } = useI18n()
const router = useRouter()
const route = useRoute()

interface RegisterForm {
  email: string
  username: string
  password: string
  password_verify: string
}

const form = ref<RegisterForm>({
  email: '',
  username: '',
  password: '',
  password_verify: '',
})

const loading = ref(false)
const formRef = ref()

// Email validation regex
const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/

// Username validation regex (alphanumeric, underscore, dash, 4-20 chars)
const usernameRegex = /^[a-zA-Z0-9_-]{4,20}$/

// Password strength validation
const validatePasswordStrength = (password: string): { isValid: boolean; message: string } => {
  if (password.length < 12) {
    return { isValid: false, message: t('validation.password_too_short') }
  }
  if (!/[A-Z]/.test(password)) {
    return { isValid: false, message: t('validation.password_no_uppercase') }
  }
  if (!/[a-z]/.test(password)) {
    return { isValid: false, message: t('validation.password_no_lowercase') }
  }
  if (!/\d/.test(password)) {
    return { isValid: false, message: t('validation.password_no_number') }
  }
  return { isValid: true, message: '' }
}

const resolver = ({ values }: FormResolverOptions) => {
  const errors: Partial<Record<keyof RegisterForm, { message: string }[]>> = {}

  // Email validation
  if (!values.email.trim()) {
    errors.email = [{ message: t('validation.email_required') }]
  } else if (!emailRegex.test(values.email)) {
    errors.email = [{ message: t('validation.email_invalid') }]
  }

  // Username validation
  if (!values.username.trim()) {
    errors.username = [{ message: t('validation.username_required') }]
  } else if (!usernameRegex.test(values.username)) {
    errors.username = [{ message: t('validation.username_invalid') }]
  }

  // Password validation
  if (!values.password) {
    errors.password = [{ message: t('validation.password_required') }]
  } else {
    const passwordValidation = validatePasswordStrength(values.password)
    if (!passwordValidation.isValid) {
      errors.password = [{ message: passwordValidation.message }]
    }
  }

  // Password verification
  if (!values.password_verify) {
    errors.password_verify = [{ message: t('validation.password_verify_required') }]
  } else if (values.password !== values.password_verify) {
    errors.password_verify = [{ message: t('validation.password_mismatch') }]
  }

  return {
    errors,
  }
}

const handleRegister = async ({ valid }: FormSubmitEvent) => {
  if (!valid) {
    return
  }
  
  loading.value = true
  try {
    await register(form.value, (route.query.invitation_key as string) ?? '')
    router.push('/login')
  } catch (error) {
    console.error('Registration failed:', error)
  }
  loading.value = false
}
</script>

<style scoped>
.form {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.form-item {
  width: 100%;
}
</style>
