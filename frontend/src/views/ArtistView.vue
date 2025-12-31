<template>
  <div v-if="artist" id="artist-view" class="with-sidebar">
    <div class="main">
      <ArtistSlimHeader class="slim-header" :artist @artistEdited="artist = $event" @artistDeleted="onArtistDeleted" />
      <ContentContainer v-if="title_group_preview_mode == 'cover-only'">
        <div class="title-groups">
          <TitleGroupPreviewCoverOnly v-for="title_group in title_groups" :key="title_group.id" :titleGroup="title_group" />
        </div>
      </ContentContainer>
      <div v-if="title_group_preview_mode == 'table'">
        <TitleGroupPreviewTable v-for="title_group in title_groups" :key="title_group.id" :title_group="title_group" class="preview-table" />
      </div>
    </div>
    <ArtistSidebar :artist class="sidebar" />
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import ContentContainer from '@/components/ContentContainer.vue'
import ArtistSidebar from '@/components/artist/ArtistSidebar.vue'
import TitleGroupPreviewCoverOnly from '@/components/title_group/TitleGroupPreviewCoverOnly.vue'
import TitleGroupPreviewTable from '@/components/title_group/TitleGroupPreviewTable.vue'
import ArtistSlimHeader from '@/components/artist/ArtistSlimHeader.vue'
import { getArtistPublications, type Artist, type TitleGroupHierarchyLite } from '@/services/api-schema'

const route = useRoute()
const router = useRouter()

const artist = ref<Artist>()
const title_groups = ref<TitleGroupHierarchyLite[]>([])
const title_group_preview_mode = ref<'table' | 'cover-only'>('table')
const siteName = import.meta.env.VITE_SITE_NAME

const fetchArtist = async () => {
  const artistData = await getArtistPublications(parseInt(route.params.id.toString()))

  artist.value = artistData.artist
  title_groups.value = artistData.title_groups

  document.title = `${artistData.artist.name} - ${siteName}`
}

const onArtistDeleted = () => {
  router.push('/artists')
}

watch(() => route.params.id, fetchArtist, { immediate: true })
</script>

<style scoped>
.main {
  width: 75%;
}
.sidebar {
  width: 25%;
}
.title-groups {
  display: flex;
  align-items: center;
  justify-content: space-around;
  flex-wrap: wrap;
}
.preview-table {
  margin-bottom: 15px;
}
</style>
