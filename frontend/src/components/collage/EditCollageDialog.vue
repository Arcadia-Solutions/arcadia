<template>
  <div class="edit-collage">
    <FloatLabel style="margin-bottom: 30px">
      <InputText name="name" v-model="editedCollage.name" style="width: 100%" />
      <label for="name">{{ t('general.name') }}</label>
    </FloatLabel>
    <FloatLabel style="margin-bottom: 30px">
      <Select v-model="editedCollage.category" inputId="category" :options="getCollageCategories()" style="width: 100%" size="small" />
      <label for="category">{{ t('general.category') }}</label>
    </FloatLabel>
    <BBCodeEditor
      :initialValue="initialCollage.description"
      :label="t('general.description')"
      @valueChange="(val: string) => (editedCollage.description = val)"
    />
    <FloatLabel style="margin-top: 20px; margin-bottom: 20px">
      <InputText name="cover" v-model="editedCollage.cover" style="width: 100%" />
      <label for="cover">{{ t('collage.cover_url') }}</label>
    </FloatLabel>
    <div class="wrapper-center">
      <Button :label="t('general.confirm')" size="small" :loading="loading" @click="sendEdits()" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { FloatLabel, InputText, Select } from 'primevue'
import Button from 'primevue/button'
import { ref, onMounted, toRaw } from 'vue'
import { useI18n } from 'vue-i18n'
import BBCodeEditor from '../community/BBCodeEditor.vue'
import { editCollage, type Collage, type EditedCollage } from '@/services/api-schema'
import { getCollageCategories } from '@/services/helpers'

const { t } = useI18n()

const props = defineProps<{
  initialCollage: EditedCollage
}>()

const editedCollage = ref<EditedCollage>({
  id: 0,
  name: '',
  description: '',
  category: 'Personal',
  cover: null,
  tags: [],
})
const loading = ref(false)

const emit = defineEmits<{
  done: [Collage]
}>()

const sendEdits = () => {
  loading.value = true
  editCollage(editedCollage.value).then((newCollage) => {
    loading.value = false
    emit('done', newCollage)
  })
}

onMounted(() => {
  editedCollage.value = structuredClone(toRaw(props.initialCollage))
})
</script>

<style scoped>
.edit-collage {
  width: 50vw;
}
</style>
