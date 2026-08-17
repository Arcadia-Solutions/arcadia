<template>
  <div class="merge-tag">
    <p>{{ t('title_group.merge_tag_description') }}</p>
    <div>
      <span class="bold">{{ t('title_group.source') }}:</span> {{ tag.name }}
      <br />
      <span class="bold">{{ t('title_group.target') }}:</span>
      <span v-if="targetTag"> {{ targetTag.name }}</span>
    </div>
    <TitleGroupTagSearchBar :hideTags="[tag.name]" :placeholder="t('title_group.merge_tag_target')" @tag-selected="targetTag = $event" />
    <Button :label="t('general.confirm')" severity="danger" size="small" :loading :disabled="!targetTag" @click="sendMerge()" />
  </div>
</template>

<script setup lang="ts">
import { mergeTitleGroupTags, type EditedTitleGroupTag, type TitleGroupTagLite } from '@/services/api-schema'
import TitleGroupTagSearchBar from '@/components/title_group/TitleGroupTagSearchBar.vue'
import Button from 'primevue/button'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps<{
  tag: EditedTitleGroupTag
}>()

const emit = defineEmits<{
  merged: []
}>()

const targetTag = ref<TitleGroupTagLite | null>(null)
const loading = ref(false)

const sendMerge = () => {
  if (!targetTag.value) {
    return
  }
  loading.value = true
  mergeTitleGroupTags({ source_tag_id: props.tag.id, target_tag_id: targetTag.value.id })
    .then(() => {
      emit('merged')
    })
    .finally(() => {
      loading.value = false
    })
}
</script>

<style scoped>
.merge-tag {
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 1rem;
  width: 40em;
}
</style>
