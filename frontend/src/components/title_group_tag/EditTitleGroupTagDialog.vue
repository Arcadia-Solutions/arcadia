<template>
  <div class="edit-tag">
    <FloatLabel>
      <InputText name="name" v-model="editedTag.name" />
      <label for="name">{{ t('general.name') }}</label>
    </FloatLabel>
    <div class="synonyms-container">
      <FloatLabel>
        <InputText name="synonyms" v-model="synonymsInput" @keydown.enter.prevent="addSynonym" />
        <label for="synonyms">{{ t('title_group.add_synonym', 2) }}</label>
      </FloatLabel>
      <div v-if="editedTag.synonyms.length > 0" class="synonyms-list">
        <Chip v-for="(synonym, index) in editedTag.synonyms" :key="index">
          {{ synonym }}
          <i class="pi pi-times-circle cursor-pointer" style="font-size: 0.8rem" @click="removeSynonym(index)" />
        </Chip>
      </div>
    </div>
    <div class="wrapper-center">
      <Button :label="t('general.confirm')" size="small" :loading="loading" @click="sendEdits()" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { editTitleGroupTag, type EditedTitleGroupTag } from '@/services/api-schema'
import { FloatLabel, InputText, Chip } from 'primevue'
import Button from 'primevue/button'
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps<{
  initialTag: EditedTitleGroupTag
}>()

const editedTag = ref<EditedTitleGroupTag>({
  id: 0,
  name: '',
  synonyms: [],
})
const synonymsInput = ref('')
const loading = ref(false)

const emit = defineEmits<{
  done: [EditedTitleGroupTag]
}>()

const addSynonym = () => {
  const trimmed = synonymsInput.value.trim()
  if (trimmed && !editedTag.value.synonyms.includes(trimmed)) {
    editedTag.value.synonyms.push(trimmed)
    synonymsInput.value = ''
  }
}

const removeSynonym = (index: number) => {
  editedTag.value.synonyms.splice(index, 1)
}

const sendEdits = () => {
  loading.value = true
  editTitleGroupTag(editedTag.value).then(() => {
    loading.value = false
    emit('done', editedTag.value)
  })
}

onMounted(() => {
  Object.assign(editedTag.value, props.initialTag)
})
</script>

<style scoped>
.edit-tag {
  width: 30em;
}
.synonyms-container {
  margin-top: 20px;
  margin-bottom: 20px;
}
.synonyms-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-top: 10px;
}
</style>
