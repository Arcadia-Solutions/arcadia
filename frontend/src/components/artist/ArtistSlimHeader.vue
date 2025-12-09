<template>
  <div class="artist-header">
    <div class="name">{{ artist.name }}</div>
    <div class="actions">
      <i
        class="pi pi-pen-to-square"
        v-if="userStore.class === 'staff' || artist.created_by_id === userStore.id"
        v-tooltip.top="t('artist.edit')"
        @click="editArtist"
      />
      <i class="pi pi-bell" v-tooltip.top="'Not implemented yet'" />
      <i class="pi pi-bookmark" v-tooltip.top="'Not implemented yet'" />
    </div>
  </div>
  <Dialog closeOnEscape modal :header="t('artist.edit')" v-model:visible="editArtistDialogVisible">
    <EditArtistDialog v-if="artistBeingEdited" :initialArtist="artistBeingEdited" @done="artistEdited" />
  </Dialog>
</template>
<script setup lang="ts">
import { useUserStore } from '@/stores/user'
import { useI18n } from 'vue-i18n'
import { ref } from 'vue'
import Dialog from 'primevue/dialog'
import EditArtistDialog from './EditArtistDialog.vue'
import type { Artist, EditedArtist } from '@/services/api-schema'

const { t } = useI18n()
const userStore = useUserStore()

const props = defineProps<{
  artist: Artist
}>()

const emit = defineEmits<{
  artistEdited: [Artist]
}>()

const editArtistDialogVisible = ref(false)
const artistBeingEdited = ref<EditedArtist | null>(null)

const editArtist = () => {
  artistBeingEdited.value = props.artist
  editArtistDialogVisible.value = true
}

const artistEdited = (artist: Artist) => {
  editArtistDialogVisible.value = false
  emit('artistEdited', artist)
}
</script>
<style scoped>
.artist-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.actions {
  i {
    margin: 0 3px;
    cursor: pointer;
  }
}
.name {
  font-weight: bold;
  font-size: 2em;
}
</style>
