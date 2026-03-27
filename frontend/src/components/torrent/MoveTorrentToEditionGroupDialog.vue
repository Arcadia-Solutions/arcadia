<template>
  <div class="move-torrent">
    <FloatLabel>
      <Select
        v-model="selectedEditionGroupId"
        :options="availableEditionGroups"
        optionLabel="label"
        optionValue="value"
        inputId="target-edition-group"
        size="small"
        class="select-edition-group"
      />
      <label for="target-edition-group">{{ t('torrent.target_edition_group') }}</label>
    </FloatLabel>
    <Button :label="t('general.confirm')" size="small" :loading="loading" :disabled="!selectedEditionGroupId" @click="moveTorrent()" />
  </div>
</template>

<script setup lang="ts">
import { moveTorrentToEditionGroup } from '@/services/api-schema'
import { getEditionGroupSlug } from '@/services/helpers'
import FloatLabel from 'primevue/floatlabel'
import Select from 'primevue/select'
import Button from 'primevue/button'
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import type { EditionGroupInfoLite } from '@/services/api-schema'

const { t } = useI18n()

const props = defineProps<{
  torrentId: number
  currentEditionGroupId: number
  editionGroups: EditionGroupInfoLite[]
}>()

const emit = defineEmits<{
  done: [targetEditionGroupId: number]
}>()

const selectedEditionGroupId = ref<number | null>(null)
const loading = ref(false)

const availableEditionGroups = computed(() =>
  props.editionGroups
    .filter((eg) => eg.id !== props.currentEditionGroupId)
    .map((eg) => ({
      label: getEditionGroupSlug(eg),
      value: eg.id,
    })),
)

const moveTorrent = () => {
  if (!selectedEditionGroupId.value) return
  loading.value = true
  moveTorrentToEditionGroup({
    torrent_id: props.torrentId,
    target_edition_group_id: selectedEditionGroupId.value,
  })
    .then(() => {
      emit('done', selectedEditionGroupId.value!)
    })
    .finally(() => {
      loading.value = false
    })
}
</script>

<style scoped>
.move-torrent {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  min-width: 300px;
}
.select-edition-group {
  width: 100%;
}
</style>
