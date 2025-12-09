<template>
  <div v-if="cssSheet">
    <div class="title">{{ isEditMode ? t('css_sheet.edit_sheet') : t('css_sheet.create_sheet') }}</div>
    <div class="line">
      <FloatLabel class="css-name" variant="in">
        <InputText v-model="cssSheet.name" name="name" :format="false" />
        <label for="name">{{ t('general.name') }}</label>
      </FloatLabel>
      <FloatLabel class="css-preview-url" variant="in">
        <InputText v-model="cssSheet.preview_image_url" name="preview_image_url" :format="false" />
        <label for="preview_image_url">{{ t('css_sheet.preview_image_url') }}</label>
      </FloatLabel>
    </div>
    <FloatLabel class="css-content" variant="in">
      <Textarea v-model="cssSheet.css" name="css" class="textarea pre-style" rows="30" />
      <label for="css">{{ t('css_sheet.css_content') }}</label>
    </FloatLabel>
    <div class="wrapper-center">
      <Button :label="isEditMode ? t('css_sheet.save') : t('css_sheet.create_sheet')" :loading @click="submitSheet" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { Button, FloatLabel, InputText, Textarea } from 'primevue'
import { createCSSSheet, editCSSSheet, getCSSSheet, type EditedCssSheet, type UserCreatedCssSheet } from '@/services/api-schema'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()
const isEditMode = computed(() => route.path.endsWith('/edit'))

const loading = ref(false)
const cssSheet = ref<EditedCssSheet | UserCreatedCssSheet>()

const submitSheet = () => {
  if (!cssSheet.value) return
  loading.value = true
  if (isEditMode.value) {
    editCSSSheet(cssSheet.value as EditedCssSheet)
      .then(() => {
        router.push(router.options.history.state.back ? router.options.history.state.back.toString() : '/')
      })
      .catch(() => (loading.value = false))
  } else {
    const newSheet = cssSheet.value as UserCreatedCssSheet
    createCSSSheet(newSheet)
      .then(() => {
        router.push(router.options.history.state.back ? router.options.history.state.back.toString() : '/')
      })
      .catch(() => (loading.value = false))
  }
}

onMounted(async () => {
  if (isEditMode.value) {
    const name = (route.params as { name: string }).name
    cssSheet.value = { old_name: '', ...(await getCSSSheet(name)) }
    cssSheet.value.old_name = cssSheet.value.name
  } else {
    cssSheet.value = {
      name: '',
      css: '',
      preview_image_url: '',
    }
  }
})
</script>

<style scoped>
.css-name,
.css-preview-url {
  margin-bottom: 20px;
  input {
    width: 100%;
  }
}
.css-content {
  margin-bottom: 20px;
  textarea {
    width: 100%;
  }
}
.actions {
  margin-top: 20px;
}
</style>
