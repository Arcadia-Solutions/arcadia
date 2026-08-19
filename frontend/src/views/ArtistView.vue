<template>
  <div v-if="artist" id="artist-view" class="with-sidebar">
    <div class="main">
      <ArtistSlimHeader class="slim-header" :artist @artistEdited="artist = $event" @artistDeleted="onArtistDeleted" />
      <PaginatedResults :totalItems="totalTitleGroups" :pageSize="TITLE_GROUPS_PAGE_SIZE" :totalPages :initialPage="page" @changePage="goToPage($event.page)">
        <ContentContainer v-if="title_group_preview_mode == 'cover-only'">
          <div class="title-groups">
            <TitleGroupPreviewCoverOnly v-for="title_group in title_groups" :key="title_group.id" :titleGroup="title_group" />
          </div>
        </ContentContainer>
        <div v-if="title_group_preview_mode == 'table'">
          <TitleGroupPreviewTable v-for="title_group in title_groups" :key="title_group.id" :title_group="title_group" class="preview-table" />
        </div>
      </PaginatedResults>
    </div>
    <ArtistSidebar :artist :tags v-model:relatedThreads="relatedThreads" class="sidebar" />
  </div>
</template>

<script setup lang="ts">
import { config } from '@/config'
import { computed, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import ContentContainer from '@/components/ContentContainer.vue'
import PaginatedResults from '@/components/PaginatedResults.vue'
import ArtistSidebar from '@/components/artist/ArtistSidebar.vue'
import TitleGroupPreviewCoverOnly from '@/components/title_group/TitleGroupPreviewCoverOnly.vue'
import TitleGroupPreviewTable from '@/components/title_group/TitleGroupPreviewTable.vue'
import ArtistSlimHeader from '@/components/artist/ArtistSlimHeader.vue'
import {
  getArtist,
  searchTorrents,
  type Artist,
  type RelatedForumThread,
  type TitleGroupHierarchyLite,
  TorrentSearchOrderByColumn,
  OrderByDirection,
} from '@/services/api-schema'

const route = useRoute()
const router = useRouter()

const TITLE_GROUPS_PAGE_SIZE = 100

const artist = ref<Artist>()
const tags = ref<{ [key: string]: number }>({})
const relatedThreads = ref<RelatedForumThread[]>([])
const title_groups = ref<TitleGroupHierarchyLite[]>([])
const title_group_preview_mode = ref<'table' | 'cover-only'>('table')
const page = ref(1)
const totalTitleGroups = ref(0)
const totalPages = computed(() => Math.ceil(totalTitleGroups.value / TITLE_GROUPS_PAGE_SIZE))
const siteName = config.site_name

const searchTitleGroups = (artistId: number) => {
  searchTorrents({
    artist_id: artistId,
    page: page.value,
    page_size: TITLE_GROUPS_PAGE_SIZE,
    title_group_include_empty_groups: false,
    title_group_content_type: [],
    title_group_category: [],
    edition_group_source: [],
    torrent_video_resolution: [],
    torrent_language: [],
    order_by_column: TorrentSearchOrderByColumn.TitleGroupOriginalReleaseDate,
    order_by_direction: OrderByDirection.Desc,
  }).then((data) => {
    title_groups.value = data.results
    totalTitleGroups.value = data.total_items
  })
}

const goToPage = (newPage: number) => {
  page.value = newPage
  searchTitleGroups(parseInt(route.params.id.toString()))
}

const fetchArtist = () => {
  const id = parseInt(route.params.id.toString())

  getArtist(id).then((data) => {
    artist.value = data.artist
    tags.value = data.tags
    relatedThreads.value = data.related_threads ?? []
    document.title = `${data.artist.name} - ${siteName}`
  })

  page.value = 1
  searchTitleGroups(id)
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
