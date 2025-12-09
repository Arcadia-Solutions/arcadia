<template>
  <AutoComplete
    v-model="name"
    :suggestions="foundArtists"
    @complete="search"
    size="small"
    :placeholder
    optionLabel="name"
    @option-select="artistSelected"
    @input="onInput"
  >
    <template #option="slotProps">
      <div>{{ slotProps.option.name }}</div>
    </template>
  </AutoComplete>
</template>

<script lang="ts" setup>
import { ref, watch } from 'vue'
import { AutoComplete, type AutoCompleteOptionSelectEvent } from 'primevue'
import { searchArtists, type ArtistLite } from '@/services/api-schema'

const props = defineProps<{
  placeholder: string
  clearInputOnSelect: boolean
  modelValue: string
}>()

const emit = defineEmits<{
  'update:modelValue': [string]
  artistSelected: [ArtistLite]
}>()

const name = ref('')

watch(
  () => props.modelValue,
  (newValue) => {
    name.value = newValue
  },
  { immediate: true },
)

const foundArtists = ref<ArtistLite[]>()

const artistSelected = (event: AutoCompleteOptionSelectEvent) => {
  const selectedArtistName = (event.value as ArtistLite).name
  emit('artistSelected', event.value)
  emit('update:modelValue', selectedArtistName)
  if (props.clearInputOnSelect) {
    name.value = ''
  } else {
    name.value = selectedArtistName
  }
}

const onInput = () => {
  emit('update:modelValue', name.value)
}

const search = () => {
  if (name.value !== '') {
    searchArtists(name.value).then((artists) => {
      foundArtists.value = artists
    })
  } else {
    foundArtists.value = []
  }
}
</script>
