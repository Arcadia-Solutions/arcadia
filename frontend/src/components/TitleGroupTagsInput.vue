<template>
  <div class="tag-input-container">
    <TitleGroupTagSearchBar :hideTags="modelValue" :placeholder="t('title_group.add_tag')" @tag-selected="addTag($event.name)" />
    <div v-for="(tag, index) in modelValue" :key="index" class="tag-chip">
      <Chip>
        {{ tag }}
        <i class="pi pi-times-circle cursor-pointer" style="font-size: 0.8rem" @click="removeTag(index)" />
      </Chip>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { Chip } from 'primevue'
import { showToast } from '@/main'
import TitleGroupTagSearchBar from './title_group/TitleGroupTagSearchBar.vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps({
  modelValue: {
    type: Array<string>,
    default: [],
  },
})
const emit = defineEmits(['update:modelValue'])

const addTag = (newTag: string) => {
  if (props.modelValue.includes(newTag.trim())) {
    showToast('error', "You can't enter duplicate tags", 'error', 3000)
    return
  }
  if (newTag.trim() !== '') {
    const updatedTags = [...props.modelValue, newTag.trim()]
    emit('update:modelValue', updatedTags)
  }
}

const removeTag = (index: number) => {
  const updatedTags = props.modelValue.filter((_, i) => i !== index)
  emit('update:modelValue', updatedTags)
}
</script>

<style scoped>
.tag-input-container {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
  margin-top: 20px;
}
</style>
