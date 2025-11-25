<template>
  <!-- hide suggestions overlay when entering url (empty anyways as no search is actually performed) -->
  <AutoComplete
    v-model="collageSearchForm.name"
    :suggestions="collageSearchResults"
    @complete="search"
    size="small"
    :placeholder
    optionLabel="name"
    @option-select="collageSelected($event.value)"
    :overlayStyle="{ display: collageSearchForm.name.startsWith('http') ? 'none' : null }"
  >
    <template #option="slotProps">
      <div>{{ slotProps.option.name }}</div>
    </template>
  </AutoComplete>
</template>
<script setup lang="ts">
import { AutoComplete } from 'primevue'
import { type CollageLite, type SearchCollagesLiteQuery, searchCollagesLite } from '@/services/api/collageService'
import { ref } from 'vue'
import { getHostname } from '@/services/helpers'

const emit = defineEmits<{
  collageSelected: [CollageLite]
  urlEntered: [string]
}>()
defineProps<{
  placeholder: string
}>()

const collageSearchResults = ref<CollageLite[]>([])
const collageSearchForm = ref<SearchCollagesLiteQuery>({ name: '', results_amount: 10 })

const search = () => {
  if (collageSearchForm.value.name.startsWith('http')) {
    emit('urlEntered', collageSearchForm.value.name)
    collageSearchResults.value = []
    return
  }
  if (collageSearchForm.value.name !== '') {
    searchCollagesLite(collageSearchForm.value).then((collages) => {
      collageSearchResults.value = collages
    })
  } else {
    collageSearchResults.value = []
  }
}

const collageSelected = (collage: CollageLite) => {
  emit('collageSelected', collage)
  collageSearchForm.value.name = `https://${getHostname()}/collage/${collage.id}`
}
</script>
<style scoped></style>
