<template>
  <AutoComplete
    v-model="name"
    :suggestions="foundTitleGroups"
    @complete="search"
    size="small"
    :placeholder
    optionLabel="name"
    @option-select="titleGroupSelected"
    @input="onInput"
    fluid
  >
    <template #option="slotProps">
      <!-- <div>{{ slotProps.option.name }} ({{ slotProps.option.original_release_date.substring(0, 4) }})</div> -->
      <span v-if="typeof slotProps.option === 'string'">{{ slotProps.option }}</span>
      <TitleGroupSlimHeader v-else :titleGroup="slotProps.option" :series="slotProps.option.series" :affiliatedArtists="[]" />
    </template>
  </AutoComplete>
</template>

<script lang="ts" setup>
import { ref, watch } from 'vue'
import { AutoComplete, type AutoCompleteOptionSelectEvent } from 'primevue'
import TitleGroupSlimHeader from './TitleGroupSlimHeader.vue'
import { useI18n } from 'vue-i18n'
import { searchTitleGroupInfo, type ContentType, type TitleGroupLite } from '@/services/api-schema'

const props = defineProps<{
  placeholder: string
  clearInputOnSelect: boolean
  modelValue: string
  createOption: boolean
  contentType: ContentType | null
}>()

const emit = defineEmits<{
  'update:modelValue': [string]
  titleGroupSelected: [TitleGroupLite]
  createNew: []
}>()

const { t } = useI18n()

const name = ref('')

watch(
  () => props.modelValue,
  (newValue) => {
    name.value = newValue
  },
  { immediate: true },
)

const foundTitleGroups = ref<(TitleGroupLite | string)[]>()

const titleGroupSelected = (event: AutoCompleteOptionSelectEvent) => {
  if (typeof event.value === 'string') {
    // selected create option
    emit('createNew')
  } else {
    const selectedTitleGroupName = (event.value as TitleGroupLite).name
    emit('titleGroupSelected', event.value)
    emit('update:modelValue', selectedTitleGroupName)
    if (props.clearInputOnSelect) {
      name.value = ''
    } else {
      name.value = selectedTitleGroupName
    }
  }
}

const onInput = () => {
  emit('update:modelValue', name.value)
}

const search = () => {
  if (name.value !== '') {
    searchTitleGroupInfo({ name: name.value, content_type: props.contentType }).then((titleGroups) => {
      foundTitleGroups.value = titleGroups
      if (props.createOption) {
        foundTitleGroups.value?.push(t('general.create_new_one'))
      }
    })
  } else {
    foundTitleGroups.value = []
  }
}
</script>
<style scoped></style>
