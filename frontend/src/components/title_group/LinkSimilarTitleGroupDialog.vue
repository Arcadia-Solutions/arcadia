<template>
  <Dialog :visible="visible" @update:visible="$emit('update:visible', $event)" modal :header="t('title_group.link_similar_title')" :style="{ width: '30em' }">
    <div class="fields">
      <!-- hide suggestions overlay when entering a url (no search is performed) -->
      <AutoComplete
        v-model="searchInput"
        :suggestions="searchSuggestions"
        @complete="search"
        @option-select="onTitleGroupSelected($event.value)"
        optionLabel="name"
        :placeholder="t('title_group.search_title_to_link')"
        :overlayStyle="{ display: searchInput.startsWith('http') ? 'none' : null }"
        size="small"
        fluid
      >
        <template #option="slotProps">
          <TitleGroupSlimHeader :titleGroup="slotProps.option" :series="slotProps.option.series" :affiliatedArtists="[]" />
        </template>
      </AutoComplete>
      <InputText v-model="note" :placeholder="t('title_group.similar_title_note')" size="small" fluid />
      <div class="wrapper-center">
        <Button :label="t('title_group.link_similar_title')" size="small" :loading @click="link" />
      </div>
    </div>
  </Dialog>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { AutoComplete, Button, Dialog, InputText } from 'primevue'
import TitleGroupSlimHeader from './TitleGroupSlimHeader.vue'
import { getHostname } from '@/services/helpers'
import { getTitleGroupInfoLite, linkSimilarTitleGroups, searchTitleGroupInfo, type SimilarTitleGroupLite, type TitleGroupLite } from '@/services/api-schema'
import { showToast } from '@/main'

const props = defineProps<{
  visible: boolean
  titleGroupId: number
  linkedTitleGroupIds: number[]
}>()

const emit = defineEmits<{
  'update:visible': [boolean]
  linked: [SimilarTitleGroupLite]
}>()

const { t } = useI18n()

const searchInput = ref('')
const searchSuggestions = ref<TitleGroupLite[]>([])
const note = ref('')
const loading = ref(false)

const search = () => {
  if (searchInput.value.startsWith('http') || searchInput.value === '') {
    searchSuggestions.value = []
    return
  }
  searchTitleGroupInfo({ name: searchInput.value, content_type: null }).then((titleGroups) => {
    searchSuggestions.value = titleGroups.filter((tg) => tg.id !== props.titleGroupId && !props.linkedTitleGroupIds.includes(tg.id))
  })
}

const onTitleGroupSelected = (titleGroup: TitleGroupLite) => {
  searchInput.value = `https://${getHostname()}/title-group/${titleGroup.id}`
}

const link = () => {
  const id = parseInt(searchInput.value.split('/').pop() ?? '')
  if (isNaN(id) || id === props.titleGroupId || props.linkedTitleGroupIds.includes(id)) {
    return
  }
  loading.value = true
  getTitleGroupInfoLite(id)
    .then((titleGroups) => {
      const titleGroup = titleGroups[0]
      if (!titleGroup) return
      return linkSimilarTitleGroups({
        group_1: props.titleGroupId,
        group_2: id,
        note: note.value || null,
      }).then(() => {
        showToast('Success', t('title_group.similar_title_linked'), 'success', 4000)
        emit('linked', {
          id: titleGroup.id,
          name: titleGroup.name,
          cover: titleGroup.covers[0] ?? null,
          note: note.value || null,
          original_release_date: titleGroup.original_release_date,
        })
        emit('update:visible', false)
        searchInput.value = ''
        searchSuggestions.value = []
        note.value = ''
      })
    })
    .finally(() => (loading.value = false))
}
</script>

<style scoped>
.fields {
  display: flex;
  flex-direction: column;
  gap: 10px;
}
</style>
