<template>
  <div class="artist-header">
    <div class="name">{{ artist.name }}</div>
    <div class="actions">
      <i
        class="pi pi-pen-to-square"
        v-if="userStore.permissions.includes('edit_artist') || artist.created_by_id === userStore.id"
        v-tooltip.top="t('artist.edit')"
        @click="editArtist"
      />
      <i
        v-if="userStore.permissions.includes('delete_artist')"
        class="pi pi-trash"
        v-tooltip.top="t('artist.delete_artist')"
        @click="deleteArtistDialogVisible = true"
      />
      <i class="pi pi-bell" v-tooltip.top="'Not implemented yet'" />
      <i class="pi pi-bookmark" v-tooltip.top="'Not implemented yet'" />
    </div>
  </div>
  <Dialog closeOnEscape modal :header="t('artist.edit')" v-model:visible="editArtistDialogVisible">
    <EditArtistDialog v-if="artistBeingEdited" :initialArtist="artistBeingEdited" @done="artistEdited" />
  </Dialog>
  <Dialog closeOnEscape modal :header="t('artist.delete_artist')" v-model:visible="deleteArtistDialogVisible">
    <DeleteArtistDialog :artistId="artist.id" @deleted="onArtistDeleted" />
  </Dialog>
</template>
<script setup lang="ts">
import { useUserStore } from '@/stores/user'
import { useI18n } from 'vue-i18n'
import { ref } from 'vue'
import Dialog from 'primevue/dialog'
import EditArtistDialog from './EditArtistDialog.vue'
import DeleteArtistDialog from './DeleteArtistDialog.vue'
import type { Artist, EditedArtist } from '@/services/api-schema'

const { t } = useI18n()
const userStore = useUserStore()

const props = defineProps<{
  artist: Artist
}>()

const emit = defineEmits<{
  artistEdited: [Artist]
  artistDeleted: []
}>()

const editArtistDialogVisible = ref(false)
const deleteArtistDialogVisible = ref(false)
const artistBeingEdited = ref<EditedArtist | null>(null)

const editArtist = () => {
  artistBeingEdited.value = props.artist
  editArtistDialogVisible.value = true
}

const artistEdited = (artist: Artist) => {
  editArtistDialogVisible.value = false
  emit('artistEdited', artist)
}

const onArtistDeleted = () => {
  deleteArtistDialogVisible.value = false
  emit('artistDeleted')
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
