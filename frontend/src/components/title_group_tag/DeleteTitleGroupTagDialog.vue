<template>
  <div class="delete-tag">
    <div>
      Are you sure you want to delete this tag ?
      <br />
      <br />
      <span class="bold">name:</span> {{ tag.name }}
      <br />
      <span class="bold">synonyms:</span> {{ tag.synonyms.join(', ') }}
      <br />
      <br />
      <br />
    </div>

    <Button :label="t('general.delete')" size="small" :loading="loading" @click="sendDeletion()" />
  </div>
</template>

<script setup lang="ts">
import { deleteTitleGroupTag, type EditedTitleGroupTag } from '@/services/api-schema'
import Button from 'primevue/button'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const loading = ref(false)

const props = defineProps<{
  tag: EditedTitleGroupTag
}>()

const emit = defineEmits<{
  deleted: []
}>()

const sendDeletion = () => {
  loading.value = true
  deleteTitleGroupTag({ id: props.tag.id }).then(() => {
    loading.value = false
    emit('deleted')
  })
}
</script>

<style scoped>
.delete-tag {
  display: flex;
  flex-direction: column;
  justify-content: center;
}
</style>
