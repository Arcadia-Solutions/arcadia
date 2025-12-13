<template>
  <ContentContainer>
    <div class="title">{{ pageTitle }}</div>
    <Form v-slot="$form" :initialValues="formData" :resolver @submit="onFormSubmit">
      <FloatLabel>
        <InputText v-model="formData.name" name="name" :class="{ 'p-invalid': $form.name?.invalid }" />
        <label for="name">{{ t('forum.subcategory_name') }}</label>
      </FloatLabel>
      <Message v-if="$form.name?.invalid" severity="error" size="small" variant="simple">
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
import { createForumSubCategory, editForumSubCategory, getForumSubCategoryThreads } from '@/services/api-schema/api'
import { showToast } from '@/main'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()

const isEditMode = computed(() => route.path.includes('/edit'))
const loading = ref(false)
const categoryName = ref('')
const formData = ref<{
  id?: number
  name: string
  forum_category_id?: number
}>({
  name: '',
})

const pageTitle = computed(() => {
  if (isEditMode.value) {
    return t('forum.edit_subcategory')
  }
  return categoryName.value ? `${t('forum.create_sub_category')} in category "${categoryName.value}"` : t('forum.create_sub_category')
})

const resolver = ({ values }: FormResolverOptions) => {
  const errors: Record<string, { message: string }[]> = {}

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
      await editForumSubCategory({
        id: formData.value.id!,
        name: formData.value.name,
      })
    } else {
      await createForumSubCategory({
        forum_category_id: formData.value.forum_category_id!,
        name: formData.value.name,
      })
    }
    router.go(-1)
  } catch {
    loading.value = false
  }
}

onMounted(async () => {
  if (isEditMode.value) {
    const subCategoryId = Number(route.params.id)
    const subCategory = await getForumSubCategoryThreads(subCategoryId)

    formData.value = {
      id: subCategory.id,
      name: subCategory.name,
      forum_category_id: subCategory.category.id,
    }
    categoryName.value = subCategory.category.name
  } else {
    const categoryId = route.query.categoryId
    const catName = route.query.categoryName

    if (!categoryId || !catName) {
      showToast('', 'Category information missing', 'error', 2000)
      router.push('/forum')
      return
    }

    formData.value.forum_category_id = Number(categoryId)
    categoryName.value = String(catName)
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
