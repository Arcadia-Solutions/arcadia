<template>
  <div class="poll-editor">
    <Button v-if="!poll" size="small" severity="secondary" icon="pi pi-chart-bar" :label="t('forum.add_poll')" @click="addPoll" />

    <ContentContainer v-else class="forum-poll-editor" :container-title="t('forum.poll')">
      <template #top-left>
        <i class="pi pi-chart-bar poll-icon" />
      </template>
      <template #top-right>
        <i class="pi pi-trash delete-poll" v-tooltip.top="t('general.delete')" @click="poll = null" />
      </template>
      <div class="poll-fields">
        <FloatLabel variant="in">
          <InputText v-model="poll.question" id="poll-question" :format="false" />
          <label for="poll-question">{{ t('forum.poll_question') }}</label>
        </FloatLabel>
        <div v-for="(_, index) in poll.options" :key="index" class="poll-option-row">
          <FloatLabel variant="in">
            <InputText v-model="poll.options[index]" :id="`poll-option-${index}`" :format="false" />
            <label :for="`poll-option-${index}`">{{ t('forum.poll_option') }} {{ index + 1 }}</label>
          </FloatLabel>
          <i v-if="poll.options.length > 2" class="pi pi-times" v-tooltip.top="t('general.delete')" @click="poll.options.splice(index, 1)" />
        </div>
        <Button size="small" severity="secondary" icon="pi pi-plus" :label="t('forum.add_option')" @click="poll.options.push('')" />
      </div>
    </ContentContainer>
  </div>
</template>

<script setup lang="ts">
import { FloatLabel, InputText, Button } from 'primevue'
import { useI18n } from 'vue-i18n'
import ContentContainer from '@/components/ContentContainer.vue'

const { t } = useI18n()

const poll = defineModel<{ question: string; options: string[] } | null>({ required: true })

const addPoll = () => {
  poll.value = { question: '', options: ['', ''] }
}
</script>

<style scoped>
.poll-editor {
  margin-top: 15px;
}
.forum-poll-editor :deep(.top > div:first-child) {
  display: flex;
  align-items: center;
  gap: 8px;
}
.forum-poll-editor :deep(.container-title) {
  margin-bottom: 0;
}
.poll-icon {
  color: var(--color-primary);
}
.delete-poll {
  cursor: pointer;
}
.poll-fields {
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.poll-fields :deep(.p-inputtext) {
  width: 100%;
}
.poll-fields .p-floatlabel {
  width: 100%;
}
.poll-option-row {
  display: flex;
  align-items: center;
  gap: 10px;
}
.poll-option-row i {
  cursor: pointer;
}
</style>
