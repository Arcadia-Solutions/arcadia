<template>
  <ContentContainer :title="isEditMode ? t('forum.edit_category') : t('forum.create_category')">
    <Form v-slot="$form" :initialValues="formData" :resolver @submit="onFormSubmit">
      <FloatLabel variant="in">
        <InputText v-model="formData.name" name="name" :class="{ 'p-invalid': $form.name?.invalid }" />
        <label for="name">{{ t('forum.category_name') }}</label>
      </FloatLabel>
      <Message v-if="$form.name?.invalid" severity="error">
        {{ $form.name.error?.message }}
      </Message>

      <div class="actions">
        <Button type="submit" :label="isEditMode ? t('general.save') : t('general.create')" :loading="loading" />
      </div>
    </Form>
  </ContentContainer>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { Button, FloatLabel, InputText, Message } from 'primevue'
import { Form, type FormResolverOptions, type FormSubmitEvent } from '@primevue/forms'
import ContentContainer from '@/components/ContentContainer.vue'
import { createForumCategory, editForumCategory, getForum, type EditedForumCategory, type UserCreatedForumCategory } from '@/services/api-schema/api'
import { showToast } from '@/main'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()

const isEditMode = computed(() => route.path.includes('/edit'))
const loading = ref(false)
const formData = ref<UserCreatedForumCategory | EditedForumCategory>({
  name: '',
})

const resolver = ({ values }: FormResolverOptions) => {
  const errors: Partial<Record<keyof UserCreatedForumCategory, { message: string }[]>> = {}

  if (!values.name || values.name.trim().length === 0) {
    errors.name = [{ message: t('error.field_required') }]
  } else if (values.name.trim().length < 2) {
    errors.name = [{ message: t('error.write_more_than_x_chars', [1]) }]
  }

  return { errors }
}

const onFormSubmit = async ({ valid }: FormSubmitEvent) => {
  if (!valid) return
  loading.value = true
  try {
    if (isEditMode.value) {
      await editForumCategory(formData.value as EditedForumCategory)
    } else {
      await createForumCategory(formData.value as UserCreatedForumCategory)
    }
    router.push('/forum')
  } catch {
    loading.value = false
  }
}

onMounted(async () => {
  if (isEditMode.value) {
    const categoryId = Number(route.params.id)

    // Fetch forum overview to get the category data
    const forumOverview = await getForum()
    const category = forumOverview.forum_categories.find((cat) => cat.id === categoryId)

    if (category) {
      formData.value = {
        id: category.id,
        name: category.name,
      }
    } else {
      showToast('', 'forum category not found', 'error', 2000)
      router.push('/forum')
    }
  }
})
</script>

<style scoped>
.actions {
  margin-top: 20px;
  display: flex;
  gap: 10px;
}
</style>
