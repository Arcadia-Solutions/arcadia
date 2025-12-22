<template>
  <div class="arcadia-settings" v-if="settings">
    <Form v-slot="$form" :initialValues="settings" :resolver @submit="saveSettings" validateOnSubmit :validateOnValueUpdate="false">
      <FloatLabel>
        <Select
          v-model="settings.default_css_sheet_name"
          :options="cssSheets"
          optionLabel="name"
          optionValue="name"
          name="default_css_sheet_name"
          size="small"
        />
        <label>{{ t('arcadia_settings.default_css_sheet_name') }}</label>
      </FloatLabel>
      <Message v-if="$form.default_css_sheet_name?.invalid" severity="error" size="small" variant="simple">
        {{ $form.default_css_sheet_name.error.message }}
      </Message>

      <FloatLabel>
        <Select
          v-model="settings.user_class_name_on_signup"
          :options="userClasses"
          optionLabel="name"
          optionValue="name"
          name="user_class_name_on_signup"
          size="small"
        />
        <label>{{ t('arcadia_settings.user_class_name_on_signup') }}</label>
      </FloatLabel>
      <Message v-if="$form.user_class_name_on_signup?.invalid" severity="error" size="small" variant="simple">
        {{ $form.user_class_name_on_signup.error.message }}
      </Message>

      <FloatLabel>
        <InputNumber v-model="settings.global_download_factor" name="global_download_factor" :min="0" :step="1" size="small" />
        <label>{{ t('arcadia_settings.global_download_factor') }}</label>
      </FloatLabel>
      <Message v-if="$form.global_download_factor?.invalid" severity="error" size="small" variant="simple">
        {{ $form.global_download_factor.error.message }}
      </Message>

      <FloatLabel>
        <InputNumber v-model="settings.global_upload_factor" name="global_upload_factor" :min="0" :step="1" size="small" />
        <label>{{ t('arcadia_settings.global_upload_factor') }}</label>
      </FloatLabel>
      <Message v-if="$form.global_upload_factor?.invalid" severity="error" size="small" variant="simple">
        {{ $form.global_upload_factor.error.message }}
      </Message>

      <FloatLabel>
        <InputText v-model="settings.logo_subtitle" name="logo_subtitle" :min="0" :step="1" size="small" />
        <label>{{ t('arcadia_settings.logo_subtitle') }}</label>
      </FloatLabel>

      <Checkbox v-model="settings.open_signups" name="open_signups" :binary="true" inputId="open_signups" style="margin-top: 20px; margin-right: 5px" />
      <label for="open_signups">{{ t('arcadia_settings.open_signups') }}</label>

      <div class="form-actions" style="margin-top: 20px">
        <Button type="submit" :label="t('general.save')" :loading="saving" />
      </div>
    </Form>
  </div>
</template>

<script setup lang="ts">
import { FloatLabel, InputNumber, Checkbox, Button, Message, Select, InputText } from 'primevue'
import { Form, type FormResolverOptions, type FormSubmitEvent } from '@primevue/forms'
import { useI18n } from 'vue-i18n'
import { ref, onMounted } from 'vue'
import {
  getArcadiaSettings,
  updateArcadiaSettings,
  type ArcadiaSettings,
  getCSSSheets,
  getAllUserClasses,
  type CssSheet,
  type UserClass,
} from '@/services/api-schema'
import { showToast } from '@/main'

const { t } = useI18n()

const settings = ref<ArcadiaSettings>()
const cssSheets = ref<CssSheet[]>([])
const userClasses = ref<UserClass[]>([])

const saving = ref(false)

const resolver = ({ values }: FormResolverOptions) => {
  const errors = {
    default_css_sheet_name: {},
    user_class_name_on_signup: {},
    global_download_factor: {},
    global_upload_factor: {},
  }

  if (!values.default_css_sheet_name || values.default_css_sheet_name.trim().length === 0) {
    errors.default_css_sheet_name = [{ message: t('error.field_required') }]
  }

  if (!values.user_class_name_on_signup || values.user_class_name_on_signup.trim().length === 0) {
    errors.user_class_name_on_signup = [{ message: t('error.field_required') }]
  }

  if (values.global_download_factor < 0) {
    errors.global_download_factor = [{ message: t('error.field_required') }]
  }

  if (values.global_upload_factor < 0) {
    errors.global_upload_factor = [{ message: t('error.field_required') }]
  }

  return { errors }
}

const saveSettings = async ({ valid }: FormSubmitEvent) => {
  if (!settings.value) return
  if (valid) {
    saving.value = true
    if (settings.value.logo_subtitle?.trim() === '') settings.value.logo_subtitle = null
    updateArcadiaSettings(settings.value)
      .then(() => {
        showToast('Success', t('arcadia_settings.settings_updated'), 'success', 4000)
      })
      .finally(() => {
        saving.value = false
      })
  }
}

onMounted(async () => {
  const [arcadiaSettings, cssData, userClassesData] = await Promise.all([getArcadiaSettings(), getCSSSheets(), getAllUserClasses()])

  settings.value = arcadiaSettings
  cssSheets.value = cssData.css_sheets
  userClasses.value = userClassesData
})
</script>

<style scoped></style>
