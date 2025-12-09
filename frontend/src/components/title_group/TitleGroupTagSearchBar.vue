<template>
  <AutoComplete
    v-model="name"
    :suggestions="foundTags"
    @complete="searchTags"
    size="small"
    :placeholder
    optionLabel="name"
    @option-select="tagSelected($event.value)"
  >
    <template #option="slotProps">
      <div
        v-tooltip.right="
          slotProps.option.id === 0
            ? t('title_group.tag_will_be_created')
            : slotProps.option.synonyms.length > 0
              ? `${t('general.synonym', slotProps.option.synonyms.length)}: ${slotProps.option.synonyms.join(', ')}`
              : null
        "
        style="width: 100%"
      >
        {{ slotProps.option.name }}
        <span v-if="slotProps.option.id === 0" style="font-weight: 300"> ({{ t('general.new') }})</span>
      </div>
    </template>
  </AutoComplete>
</template>

<script setup lang="ts">
import { createTitleGroupTag, searchTitleGroupTags, type TitleGroupTagLite } from '@/services/api-schema'
import { AutoComplete } from 'primevue'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps<{
  placeholder: string
  hideTags: string[]
}>()
const emit = defineEmits<{
  tagSelected: [TitleGroupTagLite]
}>()

const name = ref('')
const foundTags = ref<TitleGroupTagLite[]>([])

const searchTags = async () => {
  await searchTitleGroupTags({ name: name.value, page: 1, page_size: 10 }).then((tags) => {
    foundTags.value = tags.results.filter((tag) => !props.hideTags.includes(tag.name))
    // only show the option to create a new tag if doesn't already exist
    // and if none of the synonyms is already it
    if (!tags.results.some((tag) => tag.name === name.value || tag.synonyms.some((synonym) => synonym === name.value))) {
      foundTags.value.push({ name: name.value, synonyms: [], id: 0 })
    }
  })
}

const tagSelected = (event: TitleGroupTagLite) => {
  if (event.id === 0) {
    createTitleGroupTag({ name: event.name }).then((tag) => {
      emit('tagSelected', tag)
    })
  } else {
    emit('tagSelected', event)
  }
  name.value = ''
}
</script>
<style scoped></style>
