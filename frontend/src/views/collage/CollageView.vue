<template>
  <div v-if="collage" id="collage-view">
    <div class="main-content">
      <div class="top">
        <div class="title">{{ collage.name }}</div>
        <div class="actions">
          <!-- <i v-if="togglingSubscription" class="pi pi-hourglass" /> -->
          <!-- <i
            v-else
            v-tooltip.top="t(`general.${titleGroupAndAssociatedData.is_subscribed ? 'un' : ''}subscribe`)"
            @click="toggleSubscribtion"
            :class="`pi pi-bell${titleGroupAndAssociatedData.is_subscribed ? '-slash' : ''}`"
          /> -->
          <!-- <i v-tooltip.top="t('general.bookmark')" class="pi pi-bookmark" /> -->
          <i
            v-if="collage.created_by_id === userStore.id || userStore.permissions.includes('edit_collage')"
            v-tooltip.top="t('general.edit')"
            class="pi pi-pen-to-square cursor-pointer"
            @click="editCollageDialogVisible = true"
          />
          <i
            v-if="userStore.permissions.includes('delete_collage')"
            v-tooltip.top="t('general.delete')"
            class="pi pi-trash cursor-pointer"
            @click="deleteCollageDialogVisible = true"
          />
          <i @click="addEntriesModalVisible = true" v-tooltip.top="t('collage.add_entry_to_collage', 2)" class="pi pi-plus cursor-pointer" />
        </div>
      </div>
      <PaginatedResults v-if="entries" :totalPages :initialPage :totalItems="entries.total_items" :pageSize @change-page="changePage($event.page)">
        <TitleGroupList :titleGroups="entries.results" :titleGroupPreview />
      </PaginatedResults>
      <!-- TODO: display Artists, Entities and Master Groups -->
    </div>
    <CollageSidebar :collage="collage" class="sidebar" />
    <Dialog modal :header="t('collage.add_entry_to_collage', 2)" v-model:visible="addEntriesModalVisible">
      <AddEntriesToCollageDialog :collageId="collage.id" @addedEntries="router.go(0)" />
    </Dialog>
    <Dialog modal :header="t('collage.edit_collage')" v-model:visible="editCollageDialogVisible">
      <EditCollageDialog :initialCollage="collage" @done="onCollageEdited" />
    </Dialog>
    <Dialog modal :header="t('collage.delete_collage')" v-model:visible="deleteCollageDialogVisible">
      <DeleteCollageDialog :collageId="collage.id" @deleted="onCollageDeleted" />
    </Dialog>
  </div>
</template>
<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import CollageSidebar from '@/components/collage/CollageSidebar.vue'
import TitleGroupList, { type titleGroupPreviewMode } from '@/components/title_group/TitleGroupList.vue'
import { Dialog } from 'primevue'
import AddEntriesToCollageDialog from '@/components/collage/AddEntriesToCollageDialog.vue'
import EditCollageDialog from '@/components/collage/EditCollageDialog.vue'
import DeleteCollageDialog from '@/components/collage/DeleteCollageDialog.vue'
import { useI18n } from 'vue-i18n'
import PaginatedResults from '@/components/PaginatedResults.vue'
import { useUserStore } from '@/stores/user'
import { showToast } from '@/main'
import {
  getCollage,
  getCollageEntries,
  type Collage,
  type PaginatedResultsTitleGroupHierarchyLite,
  TorrentSearchOrderByColumn,
  OrderByDirection,
} from '@/services/api-schema'

const { t } = useI18n()
const userStore = useUserStore()

const route = useRoute()
const router = useRouter()
const siteName = import.meta.env.VITE_SITE_NAME
const collage = ref<Collage>()
const entries = ref<PaginatedResultsTitleGroupHierarchyLite>()
const titleGroupPreview = ref<titleGroupPreviewMode>('table') // TODO: make a select button to switch from cover-only to table
const pageSize = ref(10)
const totalPages = computed(() => (entries.value ? Math.ceil(entries.value.total_items / pageSize.value) : 0))
let initialPage: number | null = null

const addEntriesModalVisible = ref(false)
const editCollageDialogVisible = ref(false)
const deleteCollageDialogVisible = ref(false)

const onCollageEdited = (editedCollage: Collage) => {
  collage.value = editedCollage
  editCollageDialogVisible.value = false
  showToast('', t('collage.collage_edited_success'), 'success', 2000)
}

const onCollageDeleted = () => {
  deleteCollageDialogVisible.value = false
  router.push('/collages')
}

const fetchCollageEntries = async () => {
  const page = route.query.page ? parseInt(route.query.page as string) : 1
  if (!initialPage) {
    initialPage = page
  }
  entries.value = await getCollageEntries({
    collage_id: parseInt(route.params.id.toString()),
    page,
    page_size: pageSize.value,
    title_group_include_empty_groups: false,
    title_group_content_type: [],
    title_group_category: [],
    edition_group_source: [],
    torrent_video_resolution: [],
    torrent_language: [],
    order_by_column: TorrentSearchOrderByColumn.TorrentCreatedAt,
    order_by_direction: OrderByDirection.Desc,
  })
}

const fetchCollage = async () => {
  ;[collage.value] = await Promise.all([getCollage(parseInt(route.params.id.toString())), fetchCollageEntries()])
  document.title = collage.value ? `${collage.value.name} - ${siteName}` : `Collage - ${siteName}`
}

const changePage = (page: number) => {
  router.push({ query: { page } })
}

onMounted(async () => {
  await fetchCollage()
})

watch(
  () => route.query,
  () => {
    fetchCollageEntries()
  },
  { deep: true },
)
</script>
<style scoped>
#collage-view {
  display: flex;
}
.main-content {
  width: 75%;
  margin-right: 10px;
}
.sidebar {
  width: 25%;
}
.top {
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
}
.actions {
  i {
    margin-left: 5px;
  }
}
</style>
