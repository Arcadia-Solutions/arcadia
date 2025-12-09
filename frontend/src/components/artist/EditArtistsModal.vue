<template>
  <EditAffiliatedArtists
    ref="editAffiliatedArtistsRef"
    :initialArtistsAffiliations="artistsAffiliations"
    :contentType
    :disable-inputs-existing-artists="true"
  />
  <div class="wrapper-center">
    <Button :label="t('title_group.update_artists')" @click="sendEdits" :loading size="small" />
    <Button :label="t('general.cancel')" @click="emit('cancelled')" size="small" />
  </div>
</template>
<script lang="ts" setup>
import EditAffiliatedArtists from './EditAffiliatedArtists.vue'
import { Button } from 'primevue'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import type { VNodeRef } from 'vue'
import {
  createArtistAffiliation,
  deleteArtistAffiliation,
  type AffiliatedArtistHierarchy,
  type ContentType,
  type UserCreatedAffiliatedArtist,
} from '@/services/api-schema'

const { t } = useI18n()
const editAffiliatedArtistsRef = ref<VNodeRef | null>(null)

const props = defineProps<{
  artistsAffiliations?: AffiliatedArtistHierarchy[] | UserCreatedAffiliatedArtist[]
  contentType: ContentType
  titleGroupId: number
}>()

const emit = defineEmits<{
  done: [AffiliatedArtistHierarchy[], number[]]
  cancelled: []
}>()

const loading = ref<boolean>(false)

const sendEdits = async () => {
  loading.value = true
  await editAffiliatedArtistsRef.value.createInexistingArtists()
  if (editAffiliatedArtistsRef.value.removedExistingAffiliatedArtistsIds.length > 0) {
    // await removeArtistAffiliations()

    const params = new URLSearchParams()
    editAffiliatedArtistsRef.value.removedExistingAffiliatedArtistsIds.forEach((id: number) => {
      params.append('affiliation_ids', id.toString())
    })
    await deleteArtistAffiliation({ params })
  }
  const affiliationsToCreate = editAffiliatedArtistsRef.value.affiliated_artists
    .filter((aa: UserCreatedAffiliatedArtist | AffiliatedArtistHierarchy) => !('id' in aa))
    .map((aa: UserCreatedAffiliatedArtist) => {
      aa.title_group_id = props.titleGroupId
      return aa
    })
  let newAffiliatedArtists: AffiliatedArtistHierarchy[] = []
  if (affiliationsToCreate.length !== 0) {
    newAffiliatedArtists = await createArtistAffiliation(affiliationsToCreate)
  }
  emit('done', newAffiliatedArtists, editAffiliatedArtistsRef.value.removedExistingAffiliatedArtistsIds)
}
</script>
<style scoped>
.wrapper-center {
  margin-top: 10px;
  .p-button {
    margin: 0 10px;
  }
}
</style>
