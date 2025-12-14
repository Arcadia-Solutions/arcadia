<template>
  <div id="search-bars">
    <InputText type="text" :placeholder="t('torrent.torrent', 2)" v-model="searchForm.torrents" size="small" @keyup.enter="searchTorrents" />
    <ArtistSearchBar :placeholder="t('artist.artist', 2)" :clickableSeriesLink="true" :clearInputOnSelect="true" v-model="searchForm.artists" />
    <SeriesSearchBar :placeholder="t('series.series')" :clickableSeriesLink="true" :clearInputOnSelect="true" v-model="searchForm.series" />
    <InputText type="text" :placeholder="t('forum.forum', 2)" v-model="searchForm.forums" size="small" @keyup.enter="searchForums" />
    <InputText type="text" :placeholder="t('user.user', 2)" v-model="searchForm.users" size="small" />
  </div>
</template>

<script setup lang="ts">
import InputText from 'primevue/inputtext'
import ArtistSearchBar from './artist/ArtistSearchBar.vue'
import SeriesSearchBar from './series/SeriesSearchBar.vue'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'

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

const searchTorrents = () => {
  router.push({
    path: '/torrents',
    query: { title_group_name: searchForm.value.torrents },
  })
  searchForm.value.torrents = ''
}

const searchForums = () => {
  router.push({
    path: '/forum/search',
    query: { thread_name: searchForm.value.forums },
  })
  searchForm.value.forums = ''
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
