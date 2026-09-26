<template>
  <div class="staff-note-dialog">
    <BBCodeEditor :initialValue="content" :label="t('user_staff_note.note')" :rows="6" @valueChange="(value: string) => (content = value)" />
    <div class="wrapper-center">
      <Button :label="t('general.confirm')" size="small" :loading="submitting" :disabled="!content.trim()" @click="submit" />
    </div>
  </div>
</template>

<script setup lang="ts">
import Button from 'primevue/button'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { showToast } from '@/main'
import { createUserStaffNote, editUserStaffNote } from '@/services/api-schema'
import BBCodeEditor from '@/components/community/BBCodeEditor.vue'

const { t } = useI18n()

const props = defineProps<{
  userId: number
  // only set when an existing note is edited, the dialog then edits it instead of creating one
  staffNoteId?: number
  initialContent?: string
}>()

const emit = defineEmits<{
  saved: []
}>()

const content = ref(props.initialContent ?? '')
const submitting = ref(false)

const submit = () => {
  submitting.value = true

  const request = props.staffNoteId
    ? editUserStaffNote({
        id: props.userId,
        staff_note_id: props.staffNoteId,
        EditedUserStaffNote: { content: content.value },
      })
    : createUserStaffNote({
        id: props.userId,
        UserCreatedStaffNote: { content: content.value },
      })

  request
    .then(() => {
      showToast('', props.staffNoteId ? t('user_staff_note.note_edited_success') : t('user_staff_note.note_added_success'), 'success', 2000)
      emit('saved')
    })
    .finally(() => {
      submitting.value = false
    })
}
</script>

<style scoped>
.staff-note-dialog {
  width: 40vw;
  max-width: 500px;
  display: flex;
  flex-direction: column;
  gap: 20px;
}
</style>
