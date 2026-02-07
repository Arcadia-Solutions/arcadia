<template>
  <div id="series-view" v-if="series" class="with-sidebar">
    <div class="main">
      <div class="slim-header">
        <SeriesSlimHeader :series />
        <div class="actions">
          <i
            v-if="series.created_by_id === userStore.id || userStore.permissions.includes('edit_series')"
            v-tooltip.top="t('general.edit')"
            class="pi pi-pen-to-square cursor-pointer"
            @click="editSeriesDialogVisible = true"
          />
          <i
            v-if="userStore.permissions.includes('delete_series')"
            v-tooltip.top="t('general.delete')"
            class="pi pi-trash cursor-pointer"
            @click="deleteSeriesDialogVisible = true"
          />
          <i v-tooltip.top="t('general.bookmark')" class="pi pi-bookmark" />
          <i @click="addTitleGroupModalVisible = true" v-tooltip.top="t('series.add_title_group_to_series')" class="pi pi-plus cursor-pointer" />
        </div>
      </div>
      <PaginatedResults v-if="entries" :totalPages :initialPage :totalItems="entries.total_items" :pageSize @change-page="changePage($event.page)">
        <TitleGroupList :titleGroups="entries.results" :titleGroupPreview />
      </PaginatedResults>
    </div>
    <SeriesSidebar :series class="sidebar" />
    <Dialog modal :header="t('series.add_title_group_to_series')" v-model:visible="addTitleGroupModalVisible">
      <AddTitleGroupToSeriesDialog :seriesId="series.id" @titleGroupAdded="titleGroupAdded" />
    </Dialog>
    <Dialog modal :header="t('series.edit_series')" v-model:visible="editSeriesDialogVisible">
      <CreateOrEditSeriesView :initialSeriesForm="series" @done="seriesEdited" />
    </Dialog>
    <Dialog modal :header="t('series.delete_series')" v-model:visible="deleteSeriesDialogVisible">
      <DeleteSeriesDialog :seriesId="series.id" @deleted="onSeriesDeleted" />
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, toRaw, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { Dialog } from 'primevue'
import SeriesSlimHeader from '@/components/series/SeriesSlimHeader.vue'
import SeriesSidebar from '@/components/series/SeriesSidebar.vue'
import AddTitleGroupToSeriesDialog from '@/components/series/AddTitleGroupToSeriesDialog.vue'
import DeleteSeriesDialog from '@/components/series/DeleteSeriesDialog.vue'
import CreateOrEditSeriesView from '@/views/series/CreateOrEditSeriesView.vue'
import TitleGroupList, { type titleGroupPreviewMode } from '@/components/title_group/TitleGroupList.vue'
import PaginatedResults from '@/components/PaginatedResults.vue'
import {
  getSeries,
  getSeriesEntries,
  type Series,
  type PaginatedResultsTitleGroupHierarchyLite,
  TorrentSearchOrderByColumn,
  OrderByDirection,
} from '@/services/api-schema'
import { useUserStore } from '@/stores/user'
import { showToast } from '@/main'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()
const userStore = useUserStore()

const series = ref<Series>()
const entries = ref<PaginatedResultsTitleGroupHierarchyLite>()
const titleGroupPreview = ref<titleGroupPreviewMode>('table') // TODO: make a select button to switch from cover-only to table
const pageSize = ref(25)
const totalPages = computed(() => (entries.value ? Math.ceil(entries.value.total_items / pageSize.value) : 0))
let initialPage: number | null = null

const siteName = import.meta.env.VITE_SITE_NAME
const addTitleGroupModalVisible = ref(false)
const editSeriesDialogVisible = ref(false)
const deleteSeriesDialogVisible = ref(false)

const fetchSeriesEntries = () => {
  const page = route.query.page ? parseInt(route.query.page as string) : 1
  if (!initialPage) {
    initialPage = page
  }
  getSeriesEntries({
    series_id: parseInt(route.params.id.toString()),
    page,
    page_size: pageSize.value,
    title_group_include_empty_groups: false,
    title_group_content_type: [],
    title_group_category: [],
    edition_group_source: [],
    torrent_video_resolution: [],
    torrent_language: [],
    order_by_column: TorrentSearchOrderByColumn.TitleGroupOriginalReleaseDate,
    order_by_direction: OrderByDirection.Desc,
  }).then((data) => {
    entries.value = data
  })
}

const fetchSeries = () => {
  const id = Number(route.params.id)
  if (!Number.isNaN(id)) {
    Promise.all([getSeries(id), fetchSeriesEntries()]).then(([seriesData]) => {
      series.value = seriesData
      document.title = `${series.value?.name} - ${siteName}`
    })
  }
}

const changePage = (page: number) => {
  router.push({ query: { page } })
}

const titleGroupAdded = () => {
  addTitleGroupModalVisible.value = false
  fetchSeriesEntries()
  showToast('', t('series.title_group_added_successfully'), 'success', 3000)
}

const seriesEdited = (updatedSeries: Series) => {
  series.value = structuredClone(toRaw(updatedSeries))
  editSeriesDialogVisible.value = false
  showToast('', t('series.series_edited_success'), 'success', 2000)
}

const onSeriesDeleted = () => {
  deleteSeriesDialogVisible.value = false
  router.push('/')
}

onMounted(() => {
  fetchSeries()
})

watch(
  () => route.query,
  () => {
    fetchSeriesEntries()
  },
  { deep: true },
)
</script>

<style scoped>
.main {
  width: 75%;
}
.slim-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
}
.sidebar {
  width: 25%;
}
.actions {
  display: flex;
  justify-content: flex-end;
  margin-bottom: 5px;
  i {
    margin-left: 5px;
  }
}
</style>
<style>
#series-view .series-covers img {
  border-radius: 7px;
}
</style>
