<template>
  <div class="bookmark-title-group">
    <FloatLabel>
      <Textarea class="description" name="description" v-model="bookmark.description" rows="5" />
      <label for="description">{{ t('general.description') }}</label>
    </FloatLabel>
    <Button :label="t('general.bookmark')" size="small" :loading @click="sendBookmark()" />
  </div>
</template>

<script setup lang="ts">
import { createTitleGroupBookmark, type TitleGroupBookmark, type UserCreatedTitleGroupBookmark } from '@/services/api-schema'
import { Textarea, FloatLabel } from 'primevue'
import Button from 'primevue/button'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const loading = ref(false)

const props = defineProps<{
  titleGroupId: number
}>()

const bookmark = ref<UserCreatedTitleGroupBookmark>({ description: '', title_group_id: props.titleGroupId })

const emit = defineEmits<{
  bookmarked: [bookmark: TitleGroupBookmark]
}>()

const sendBookmark = () => {
  loading.value = true
  createTitleGroupBookmark(bookmark.value)
    .then((data: TitleGroupBookmark) => {
      emit('bookmarked', data)
    })
    .finally(() => {
      loading.value = false
    })
}
</script>

<style scoped>
.bookmark-title-group {
  padding-top: 20px;
  width: 30em !important;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}
.description {
  width: 25em;
  margin-bottom: 20px;
}
</style>
