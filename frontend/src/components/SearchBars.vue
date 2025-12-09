<template>
  <div id="search-bars">
    <InputText
      type="text"
      :placeholder="t('torrent.torrent', 2)"
      v-model="searchForm.torrents"
      size="small"
      @keyup.enter="
        () => {
          router.push(`/torrents?title_group_name=${searchForm.torrents}`)
          searchForm.torrents = ''
        }
      "
    />
    <ArtistSearchBar :placeholder="t('artist.artist', 2)" @artistSelected="artistSelected" :clearInputOnSelect="true" v-model="searchForm.artists" />
    <InputText type="text" :placeholder="t('series.series')" v-model="searchForm.series" size="small" />
    <InputText type="text" :placeholder="t('forum.forum', 2)" v-model="searchForm.forums" size="small" />
    <InputText type="text" :placeholder="t('user.user', 2)" v-model="searchForm.users" size="small" />
  </div>
</template>

<script setup lang="ts">
import InputText from 'primevue/inputtext'
import ArtistSearchBar from './artist/ArtistSearchBar.vue'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import type { ArtistLite } from '@/services/api-schema'

const { t } = useI18n()
const router = useRouter()

const searchForm = ref({
  torrents: '',
  artists: '',
  series: '',
  requests: '',
  forums: '',
  users: '',
})

const artistSelected = (artist: ArtistLite) => {
  router.push(`/artist/${artist.id}`)
}
</script>

<style scoped>
#search-bars {
  display: flex;
  justify-content: center;
  flex-wrap: wrap;
  gap: 5px;
  width: 100%;
}
</style>
