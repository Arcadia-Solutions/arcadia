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
import { searchTitleGroupTag, createTitleGroupTag, type TitleGroupTagSearchResult } from '@/services/api/titleGroupTagService'
import { AutoComplete } from 'primevue'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps<{
  placeholder: string
  hideTags: string[]
}>()
const emit = defineEmits<{
  tagSelected: [TitleGroupTagSearchResult]
}>()

const name = ref('')
const foundTags = ref<TitleGroupTagSearchResult[]>([])

const searchTags = async () => {
  await searchTitleGroupTag(name.value).then((tags) => {
    foundTags.value = tags.filter((tag) => !props.hideTags.includes(tag.name))
    // only show the option to create a new tag if doesn't already exist
    // and if none of the synonyms is already it
    if (!tags.some((tag) => tag.name === name.value || tag.synonyms.some((synonym) => synonym === name.value))) {
      foundTags.value.push({ name: name.value, synonyms: [], id: 0 })
    }
  })
}

const tagSelected = (event: TitleGroupTagSearchResult) => {
  if (event.id === 0) {
    createTitleGroupTag({ name: event.name }).then((tag) => {
      emit('tagSelected', tag)
    })
  } else {
    emit('tagSelected', event)
  }
}
</script>
<style scoped></style>
