<template>
  <ContentContainer>
    <Form>
      <FloatLabel>
        <InputText v-model="form.name" name="name" size="small" style="width: 40%" />
        <label for="name">{{ t('general.name') }}</label>
      </FloatLabel>
      <div class="line">
        <FloatLabel>
          <Select v-model="form.category" inputId="category" :options="getCollageCategories()" class="select" size="small" name="category" />
          <label for="category">{{ t('general.category') }}</label>
        </FloatLabel>
      </div>
      <!-- <TitleGroupTagsInput v-model="form.tags" /> -->
      <FloatLabel>
        <Textarea v-model="form.description" name="description" rows="5" style="width: 100%" />
        <label for="description">{{ t('general.description') }}</label>
      </FloatLabel>
      <FloatLabel>
        <InputText v-model="form.cover" name="cover" size="small" style="width: 40%" />
        <label for="cover">{{ t('collage.cover_url') }}</label>
      </FloatLabel>
      <div class="wrapper-center" style="margin-top: 15px">
        <Button :label="t('collage.create')" :loading @click="sendCollage" />
      </div>
    </Form>
  </ContentContainer>
</template>
<script setup lang="ts">
import ContentContainer from '@/components/ContentContainer.vue'
import { InputText, Textarea, Button, FloatLabel, Select } from 'primevue'
import { Form } from '@primevue/forms'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import { getCollageCategories } from '@/services/helpers'
import { createCollage, type UserCreatedCollage } from '@/services/api-schema'

const { t } = useI18n()
const router = useRouter()

const form = ref<UserCreatedCollage>({
  category: 'Personal',
  name: '',
  tags: [],
  description: '',
  cover: null,
})
const loading = ref(false)

const sendCollage = async () => {
  // TODO: form validation
  loading.value = true
  createCollage(form.value)
    .then((createdCollage) => {
      router.push(`/collage/${createdCollage.id}`)
    })
    .finally(() => (loading.value = false))
}
</script>
<style scoped></style>
