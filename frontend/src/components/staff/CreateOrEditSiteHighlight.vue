<template>
  <Form class="site-highlight-dialog" v-slot="$form" :initialValues="form" :resolver="resolver" @submit="save">
    <FloatLabel>
      <InputText v-model="form.alias" name="alias" size="small" />
      <label>{{ t('site_highlights.alias') }}</label>
    </FloatLabel>
    <Message v-if="$form.alias?.invalid" severity="error" size="small" variant="simple">
      {{ $form.alias.error?.message }}
    </Message>

    <FloatLabel>
      <Select v-model="form.item_type" :options="Object.values(SiteHighlightItemType)" name="item_type" size="small" />
      <label>{{ t('site_highlights.item_type') }}</label>
    </FloatLabel>

    <FloatLabel>
      <InputNumber v-model="form.item_id" :min="1" :step="1" name="item_id" size="small" :useGrouping="false" />
      <label>{{ t('site_highlights.item_id') }}</label>
    </FloatLabel>
    <Message v-if="$form.item_id?.invalid" severity="error" size="small" variant="simple">
      {{ $form.item_id.error?.message }}
    </Message>

    <FloatLabel>
      <InputNumber v-model="form.forum_thread_id" :min="1" :step="1" name="forum_thread_id" size="small" :useGrouping="false" />
      <label>{{ t('site_highlights.forum_thread_id') }}</label>
    </FloatLabel>
    <Message v-if="$form.forum_thread_id?.invalid" severity="error" size="small" variant="simple">
      {{ $form.forum_thread_id.error?.message }}
    </Message>

    <FloatLabel>
      <InputNumber v-model="form.position" :min="0" :step="1" name="position" size="small" :useGrouping="false" />
      <label>{{ t('site_highlights.position') }}</label>
    </FloatLabel>

    <div class="enabled-row">
      <Checkbox v-model="form.enabled" :binary="true" inputId="site_highlight_enabled" />
      <label for="site_highlight_enabled">{{ t('site_highlights.enabled') }}</label>
    </div>

    <div v-if="isEditMode" class="enabled-row">
      <Checkbox v-model="removePreviousRelatedThread" :binary="true" inputId="site_highlight_remove_prev" />
      <label for="site_highlight_remove_prev">{{ t('site_highlights.remove_previous_related_thread') }}</label>
    </div>

    <div class="wrapper-center" style="margin-top: 20px">
      <Button :label="t('general.confirm')" size="small" :loading="loading" type="submit" />
    </div>
  </Form>
</template>

<script setup lang="ts">
import { FloatLabel, InputNumber, InputText, Checkbox, Button, Select, Message } from 'primevue'
import { Form, type FormResolverOptions, type FormSubmitEvent } from '@primevue/forms'
import { ref, computed, reactive } from 'vue'
import { useI18n } from 'vue-i18n'
import { createSiteHighlight, editSiteHighlight, SiteHighlightItemType, type SiteHighlight, type CreateSiteHighlight } from '@/services/api-schema'
import { showToast } from '@/main'

const { t } = useI18n()

const props = defineProps<{
  initialHighlight?: SiteHighlight
}>()

const emit = defineEmits<{
  created: [SiteHighlight]
  edited: [SiteHighlight]
}>()

const form = reactive<CreateSiteHighlight>(
  props.initialHighlight
    ? {
        alias: props.initialHighlight.alias,
        item_type: props.initialHighlight.item_type,
        item_id: props.initialHighlight.item_id,
        forum_thread_id: props.initialHighlight.forum_thread_id,
        enabled: props.initialHighlight.enabled,
        position: props.initialHighlight.position,
      }
    : { alias: '', item_type: SiteHighlightItemType.TitleGroup, item_id: 1, forum_thread_id: 1, enabled: true, position: 0 },
)

const loading = ref(false)
const removePreviousRelatedThread = ref(false)
const isEditMode = computed(() => !!props.initialHighlight)

const resolver = ({ values }: FormResolverOptions) => {
  const errors: Partial<Record<keyof CreateSiteHighlight, { message: string }[]>> = {}

  if (!values.alias || values.alias.trim().length === 0) {
    errors.alias = [{ message: t('site_highlights.alias_required') }]
  }
  if (!values.item_id || values.item_id < 1) {
    errors.item_id = [{ message: t('error.field_required') }]
  }
  if (!values.forum_thread_id || values.forum_thread_id < 1) {
    errors.forum_thread_id = [{ message: t('error.field_required') }]
  }

  return { errors }
}

const save = ({ valid }: FormSubmitEvent) => {
  if (!valid) return
  loading.value = true

  if (isEditMode.value && props.initialHighlight) {
    editSiteHighlight({
      id: props.initialHighlight.id,
      EditSiteHighlight: {
        alias: form.alias,
        item_type: form.item_type,
        item_id: form.item_id,
        forum_thread_id: form.forum_thread_id,
        enabled: form.enabled,
        position: form.position,
        remove_previous_related_thread: removePreviousRelatedThread.value,
      },
    })
      .then((data) => {
        showToast('Success', t('site_highlights.saved'), 'success', 2000)
        emit('edited', data)
      })
      .finally(() => {
        loading.value = false
      })
  } else {
    createSiteHighlight({ ...form })
      .then((data) => {
        showToast('Success', t('site_highlights.created'), 'success', 2000)
        emit('created', data)
      })
      .finally(() => {
        loading.value = false
      })
  }
}
</script>

<style scoped>
.site-highlight-dialog {
  display: flex;
  flex-direction: column;
  gap: 20px;
  min-width: 25em;
}
.enabled-row {
  display: flex;
  align-items: center;
  gap: 8px;
}
</style>
