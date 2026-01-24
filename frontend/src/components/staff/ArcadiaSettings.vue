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
        <InputNumber v-model="settings.bonus_points_given_on_upload" name="bonus_points_given_on_upload" :min="0" :step="1" size="small" />
        <label>{{ t('arcadia_settings.bonus_points_given_on_upload') }}</label>
      </FloatLabel>
      <Message v-if="$form.bonus_points_given_on_upload?.invalid" severity="error" size="small" variant="simple">
        {{ $form.bonus_points_given_on_upload.error.message }}
      </Message>

      <FloatLabel>
        <InputNumber v-model="settings.default_torrent_bonus_points_cost" name="default_torrent_bonus_points_cost" :min="0" :step="1" size="small" />
        <label>{{ t('arcadia_settings.default_torrent_bonus_points_cost') }}</label>
      </FloatLabel>

      <div>
        <Checkbox
          v-model="settings.allow_uploader_set_torrent_bonus_points_cost"
          name="allow_uploader_set_torrent_bonus_points_cost"
          :binary="true"
          inputId="allow_uploader_set_torrent_bonus_points_cost"
          style="margin-top: 10px; margin-right: 5px"
        />
        <label for="allow_uploader_set_torrent_bonus_points_cost">{{ t('arcadia_settings.allow_uploader_set_torrent_bonus_points_cost') }}</label>
      </div>

      <FloatLabel>
        <InputText v-model="settings.logo_subtitle" name="logo_subtitle" :min="0" :step="1" size="small" />
        <label>{{ t('arcadia_settings.logo_subtitle') }}</label>
      </FloatLabel>

      <FloatLabel>
        <Chips v-model="settings.approved_image_hosts" name="approved_image_hosts" separator="," size="small" style="width: 40em" />
        <label>{{ t('arcadia_settings.approved_image_hosts') }} {{ t('arcadia_settings.approved_image_hosts_hint') }}</label>
      </FloatLabel>

      <BBCodeEditor
        :label="t('arcadia_settings.upload_page_top_text')"
        :initialValue="settings.upload_page_top_text ?? ''"
        :rows="4"
        @valueChange="(val) => (settings!.upload_page_top_text = val || null)"
        style="margin-top: 15px"
      />

      <Checkbox v-model="settings.open_signups" name="open_signups" :binary="true" inputId="open_signups" style="margin-top: 20px; margin-right: 5px" />
      <label for="open_signups">{{ t('arcadia_settings.open_signups') }}</label>

      <BBCodeEditor
        :label="t('arcadia_settings.automated_message_on_signup')"
        :initialValue="settings.automated_message_on_signup ?? ''"
        :rows="4"
        @valueChange="(val) => (settings!.automated_message_on_signup = val || null)"
        style="margin-top: 15px"
      />
      <FloatLabel>
        <InputText
          v-model="settings.automated_message_on_signup_conversation_name"
          name="automated_message_on_signup_conversation_name"
          size="small"
          style="width: 20em"
        />
        <label>{{ t('arcadia_settings.automated_message_on_signup_conversation_name') }}</label>
      </FloatLabel>
      <Message v-if="$form.automated_message_fields?.invalid" severity="error" size="small" variant="simple">
        {{ $form.automated_message_fields.error?.message }}
      </Message>

      <FloatLabel>
        <InputNumber v-model="settings.automated_message_on_signup_sender_id" name="automated_message_on_signup_sender_id" :min="1" :step="1" size="small" />
        <label>{{ t('arcadia_settings.automated_message_on_signup_sender_id') }}</label>
      </FloatLabel>

      <div>
        <Checkbox
          v-model="settings.automated_message_on_signup_locked"
          name="automated_message_on_signup_locked"
          :binary="true"
          inputId="automated_message_on_signup_locked"
          style="margin-top: 10px; margin-right: 5px"
        />
        <label for="automated_message_on_signup_locked">{{ t('arcadia_settings.automated_message_on_signup_locked') }}</label>
      </div>

      <div class="form-actions" style="margin-top: 20px">
        <Button type="submit" :label="t('general.save')" :loading="saving" />
      </div>
    </Form>
  </div>
</template>

<script setup lang="ts">
import { FloatLabel, InputNumber, Checkbox, Button, Message, Select, InputText, Chips } from 'primevue'
import BBCodeEditor from '@/components/community/BBCodeEditor.vue'
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
  const errors: Record<string, { message: string }[]> = {}

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

    // set those values to null so they're not an empty string or a boolean that should be null
    if (!settings.value.automated_message_on_signup_conversation_name?.trim()) {
      settings.value.automated_message_on_signup_conversation_name = null
      settings.value.automated_message_on_signup_locked = null
    } else {
      // if the user tried to submit an incomplete form once, or never checked this box, it'll remain null
      // but visually, it looks like it's just unchecked, and therefore false. so we just set it to false
      if (!settings.value.automated_message_on_signup_locked) {
        settings.value.automated_message_on_signup_locked = false
      }
    }
    if (!settings.value.automated_message_on_signup?.trim()) {
      settings.value.automated_message_on_signup = null
    }

    updateArcadiaSettings(settings.value)
      .then(() => {
        showToast('Success', t('arcadia_settings.settings_updated'), 'success', 4000)
      })
      .finally(() => {
        saving.value = false
      })
  }
}

onMounted(() => {
  Promise.all([getArcadiaSettings(), getCSSSheets(), getAllUserClasses()]).then(([arcadiaSettings, cssData, userClassesData]) => {
    settings.value = arcadiaSettings
    cssSheets.value = cssData.css_sheets
    userClasses.value = userClassesData
  })
})
</script>
