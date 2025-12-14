<template>
  <BBCodeEditor :label="''" :initialValue="initialComment.content" @valueChange="editedComment.content = $event">
    <template #belowInput>
      <Checkbox v-model="editedComment.locked" binary inputId="locked" name="locked" />
      <label for="locked"> Locked </label>
    </template>
    <template #buttons>
      <Button :label="t('general.edit')" @click="emit('commentEdited', editedComment)" />
    </template>
  </BBCodeEditor>
</template>

<script setup lang="ts">
import type { EditedForumPost, ForumPostHierarchy, TitleGroupCommentHierarchy } from '@/services/api-schema'
import BBCodeEditor from './BBCodeEditor.vue'
import { Button, Checkbox } from 'primevue'
import { useI18n } from 'vue-i18n'
import { ref } from 'vue'

const props = defineProps<{
  initialComment: ForumPostHierarchy | TitleGroupCommentHierarchy
}>()

const emit = defineEmits<{
  commentEdited: [EditedForumPost]
}>()

const { t } = useI18n()
const editedComment = ref<EditedForumPost>({
  id: props.initialComment.id,
  sticky: 'sticky' in props.initialComment ? props.initialComment.sticky : false,
  locked: props.initialComment.locked,
  content: props.initialComment.content,
})
</script>

<style scoped></style>
