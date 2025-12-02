<template>
  <div class="artists input-list" v-if="componentReady">
    <div v-for="(_link, index) in affiliated_artists_names" :key="index">
      <ArtistSearchBar
        :placeholder="t('artist.name')"
        :clearInputOnSelect="false"
        v-model="affiliated_artists_names[index]"
        @artistSelected="(event) => artistSelected(event, index)"
        :disabled="disableInputsExistingArtists && 'id' in affiliated_artists[index]"
      />
      <InputText
        size="small"
        v-model="affiliated_artists[index].nickname"
        :placeholder="t('artist.nickname')"
        class="artist"
        v-tooltip.top="t('artist.nickname_explanation')"
        v-if="['movie', 'tv_show'].includes(contentType)"
        :disabled="disableInputsExistingArtists && 'id' in affiliated_artists[index]"
      />
      <MultiSelect
        v-model="affiliated_artists[index].roles"
        :options="getArtistRoles(contentType)"
        size="small"
        class="select"
        :placeholder="t('artist.role.role', 2)"
        :disabled="disableInputsExistingArtists && 'id' in affiliated_artists[index]"
      />
      <!-- <span v-if="titleGroupForm.affiliated_artists[index].artist_id !== 0" class="artist-creation-hint existing">
            {{ t('artist.existing_artist') }}
          </span> -->
      <!-- <span v-else-if="affiliated_artists_names[index] !== ''" class="artist-creation-hint new">{{ t('artist.new_artist') }}</span> -->
      <Button v-if="index == 0" @click="addAffiliatedArtist" icon="pi pi-plus" size="small" />
      <Button v-if="index != 0 || affiliated_artists_names.length > 1" @click="removeAffiliatedArtist(index)" icon="pi pi-minus" size="small" />
      <!-- <Message v-if="($form.affiliated_artists as unknown as FormFieldState[])?.[index]?.invalid" severity="error" size="small" variant="simple">
      {{ ($form.affiliated_artists as unknown as FormFieldState[])[index].error?.message }}
    </Message> -->
    </div>
  </div>
</template>
<script lang="ts" setup>
import { Button, InputText, MultiSelect } from 'primevue'
import ArtistSearchBar from './ArtistSearchBar.vue'
import { getArtistRoles } from '@/services/helpers'
import { ref } from 'vue'
import type { AffiliatedArtistHierarchy, ArtistLite, UserCreatedAffiliatedArtist } from '@/services/api/artistService'
import { useI18n } from 'vue-i18n'
import type { ContentType } from '@/services/api/titleGroupService'
import { onMounted } from 'vue'
import { toRaw } from 'vue'
import { type UserCreatedArtist, createArtists } from '@/services/api/artistService'

const { t } = useI18n()

const affiliated_artists_names = ref<string[]>([])
const affiliated_artists = ref<(UserCreatedAffiliatedArtist | AffiliatedArtistHierarchy)[]>([])
const removedExistingAffiliatedArtistsIds: number[] = []

const props = defineProps<{
  contentType: ContentType
  initialArtistsAffiliations?: AffiliatedArtistHierarchy[] | UserCreatedAffiliatedArtist[]
  disableInputsExistingArtists?: boolean
}>()

const componentReady = ref(false)

const artistSelected = (artist: ArtistLite, index: number) => {
  affiliated_artists.value[index].artist_id = artist.id
}
const addAffiliatedArtist = () => {
  affiliated_artists_names.value.push('')
  affiliated_artists.value.push({
    artist_id: 0,
    nickname: null,
    roles: [],
    title_group_id: 0,
  })
}
const removeAffiliatedArtist = (index: number) => {
  affiliated_artists_names.value.splice(index, 1)
  if ('id' in affiliated_artists.value[index]) {
    removedExistingAffiliatedArtistsIds.push(affiliated_artists.value[index].id)
  }
  affiliated_artists.value.splice(index, 1)
}
const createInexistingArtists = async () => {
  const artistsToCreate: UserCreatedArtist[] = []
  affiliated_artists.value.forEach((artist: UserCreatedAffiliatedArtist, index: number) => {
    if (artist.artist_id === 0 && affiliated_artists_names.value[index] !== '') {
      artistsToCreate.push({
        name: affiliated_artists_names.value[index],
        pictures: [],
        description: '',
      })
    }
  })
  if (artistsToCreate.length !== 0) {
    const createdArtists = await createArtists(artistsToCreate)
    // add the artist IDs returned after creation
    affiliated_artists.value.forEach((artist: UserCreatedAffiliatedArtist) => {
      if (artist.artist_id === 0) {
        artist.artist_id = createdArtists[0].id
        createdArtists.shift()
      }
    })
  }
  // removing the artists that are empty inputs and keeping the ones that need to be added as affiliated
  affiliated_artists.value = affiliated_artists.value.filter((aa: UserCreatedAffiliatedArtist) => aa.artist_id !== 0)
}
defineExpose({
  affiliated_artists,
  affiliated_artists_names,
  createInexistingArtists,
  removedExistingAffiliatedArtistsIds,
})

onMounted(() => {
  if (props.initialArtistsAffiliations) {
    props.initialArtistsAffiliations.forEach((aa: AffiliatedArtistHierarchy | UserCreatedAffiliatedArtist) => {
      affiliated_artists_names.value.push('artist' in aa ? aa.artist.name : '')
    })
    affiliated_artists.value = structuredClone(toRaw(props.initialArtistsAffiliations))
  }
  componentReady.value = true
})
</script>
<style scoped>
.artist {
  width: 230px;
}
.artist-creation-hint {
  margin-right: 5px;
  &.new {
    color: green;
  }
}
.select {
  width: 200px;
}
</style>
