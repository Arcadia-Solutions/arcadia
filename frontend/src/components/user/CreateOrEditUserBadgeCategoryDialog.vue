<template>
  <div class="user-badge-category-dialog">
    <FloatLabel>
      <InputText name="name" v-model="category.name" style="width: 100%" />
      <label>{{ t('general.name') }}</label>
    </FloatLabel>
    <div class="wrapper-center">
      <Button :label="t('general.confirm')" size="small" :loading="loading" @click="save()" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { FloatLabel, InputText, Button } from 'primevue'
import { ref, computed, onMounted, toRaw } from 'vue'
import { useI18n } from 'vue-i18n'
import { createUserBadgeCategory, editUserBadgeCategory, type UserBadgeCategory, type UserCreatedUserBadgeCategory } from '@/services/api-schema'
import { showToast } from '@/main'

const { t } = useI18n()

const props = defineProps<{
  initialCategory?: UserBadgeCategory
}>()

const emit = defineEmits<{
  done: [UserBadgeCategory]
}>()

const loading = ref(false)
const isEditMode = computed(() => !!props.initialCategory)

const category = ref<UserCreatedUserBadgeCategory>({ name: '' })

const save = () => {
  loading.value = true
  const promise =
    isEditMode.value && props.initialCategory
      ? editUserBadgeCategory({ id: props.initialCategory.id, name: category.value.name })
      : createUserBadgeCategory(category.value)
  promise
    .then((result) => {
      showToast('', t(isEditMode.value ? 'user_badge.user_badge_category_edited_success' : 'user_badge.user_badge_category_created_success'), 'success', 2000)
      emit('done', result)
    })
    .finally(() => {
      loading.value = false
    })
}

onMounted(() => {
  if (props.initialCategory) {
    category.value = { name: toRaw(props.initialCategory).name }
  }
})
</script>

<style scoped>
.user-badge-category-dialog {
  width: 30vw;
  max-width: 400px;
  display: flex;
  flex-direction: column;
  gap: 20px;
}
</style>
