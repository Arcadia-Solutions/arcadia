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
      <i v-if="togglingTitleGroupsSubscription" class="pi pi-hourglass" />
      <i
        v-else
        v-tooltip.top="t(`artist.${isSubscribedToTitleGroups ? 'unsubscribe_from_title_groups' : 'subscribe_to_title_groups'}`)"
        :class="`pi pi-bell${isSubscribedToTitleGroups ? '-slash' : ''}`"
        @click="toggleTitleGroupsSubscription"
      />
      <i class="pi pi-bookmark" v-tooltip.top="'Not implemented yet'" />
    </div>
  </div>
  <Dialog closeOnEscape modal :header="t('artist.edit')" v-model:visible="editArtistDialogVisible">
    <EditArtistDialog v-if="artistBeingEdited" :initialArtist="artistBeingEdited" @done="artistEdited" />
  </Dialog>
  <Dialog closeOnEscape modal :header="t('artist.delete_artist')" v-model:visible="deleteArtistDialogVisible">
    <DeleteDialog
      :message="t('artist.confirm_delete_artist')"
      :action="() => deleteArtist(artist.id)"
      :successMessage="t('artist.artist_deleted_success')"
      @deleted="onArtistDeleted"
    />
  </Dialog>
</template>
<script setup lang="ts">
import { useUserStore } from '@/stores/user'
import { useI18n } from 'vue-i18n'
import { ref } from 'vue'
import Dialog from 'primevue/dialog'
import EditArtistDialog from './EditArtistDialog.vue'
import DeleteDialog from '@/components/DeleteDialog.vue'
import { createArtistTitleGroupsSubscription, deleteArtist, removeArtistTitleGroupsSubscription, type Artist, type EditedArtist } from '@/services/api-schema'
import { showToast } from '@/main'

const { t } = useI18n()
const userStore = useUserStore()

const props = defineProps<{
  artist: Artist
}>()

const emit = defineEmits<{
  artistEdited: [Artist]
  artistDeleted: []
}>()

const isSubscribedToTitleGroups = defineModel<boolean>('isSubscribedToTitleGroups', { required: true })
const togglingTitleGroupsSubscription = ref(false)
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

const toggleTitleGroupsSubscription = () => {
  togglingTitleGroupsSubscription.value = true
  const request = isSubscribedToTitleGroups.value ? removeArtistTitleGroupsSubscription(props.artist.id) : createArtistTitleGroupsSubscription(props.artist.id)

  request
    .then(() => {
      isSubscribedToTitleGroups.value = !isSubscribedToTitleGroups.value
      showToast('', t(`artist.${isSubscribedToTitleGroups.value ? 'subscription_successful' : 'unsubscription_successful'}`), 'success', 3000)
    })
    .finally(() => {
      togglingTitleGroupsSubscription.value = false
    })
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
