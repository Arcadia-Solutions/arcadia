<template>
  <div v-if="collage" id="collage-view">
    <div class="main-content">
      <div class="title">{{ collage.name }}</div>
      <div class="actions">
        <div>
          <i v-tooltip.top="t('general.bookmark')" class="pi pi-bookmark" />
        </div>
        <div>
          <i @click="addEntriesModalVisible = true" v-tooltip.top="t('collage.add_entry_to_collage', 2)" class="pi pi-plus cursor-pointer" />
        </div>
      </div>
      <PaginatedResults v-if="entries.length > 0" :totalPages :initialPage :totalItems @change-page="changePage($event.page)" :page-size="pageSize">
        <TitleGroupList
          v-if="collage.collage_type === 'TitleGroup'"
          :titleGroups="entries.map((entry) => entry.title_group as TitleGroupHierarchyLite)"
          :titleGroupPreview
        />
        <!-- TODO: display Artists, Entities and Master Groups -->
      </PaginatedResults>
      <div v-else-if="!loading" class="empty-message">
        {{ t('collage.no_entries') }}
      </div>
    </div>
    <CollageSidebar :collage />
    <Dialog modal :header="t('collage.add_entry_to_collage', 2)" v-model:visible="addEntriesModalVisible">
      <AddEntriesToCollageDialog :collageId="collage.id" :collageType="collage.collage_type" @addedEntries="onEntriesAdded" />
    </Dialog>
  </div>
</template>
<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import CollageSidebar from '@/components/collage/CollageSidebar.vue'
import TitleGroupList, { type titleGroupPreviewMode } from '@/components/title_group/TitleGroupList.vue'
import PaginatedResults from '@/components/PaginatedResults.vue'
import { Dialog } from 'primevue'
import AddEntriesToCollageDialog from '@/components/collage/AddEntriesToCollageDialog.vue'
import { useI18n } from 'vue-i18n'
import { getCollage, getCollageEntries, type Collage, type CollageEntryHierarchy, type TitleGroupHierarchyLite } from '@/services/api-schema'

const { t } = useI18n()

const route = useRoute()
const router = useRouter()
const siteName = import.meta.env.VITE_SITE_NAME

const collage = ref<Collage>()
const entries = ref<CollageEntryHierarchy[]>([])
const totalItems = ref(0)
const pageSize = ref(20)
const totalPages = computed(() => Math.ceil(totalItems.value / pageSize.value))
const currentPage = ref(1)
let initialPage: number | null = null
const loading = ref(true)

const titleGroupPreview = ref<titleGroupPreviewMode>('table')
const addEntriesModalVisible = ref(false)

onMounted(async () => {
  await Promise.all([fetchCollage(), fetchEntriesFromUrl()])
  loading.value = false
})

const fetchCollage = async () => {
  collage.value = await getCollage(parseInt(route.params.id.toString()))
  document.title = `${collage.value.name} - ${siteName}`
}

const fetchEntriesFromUrl = async () => {
  let page = 1
  if (route.query.page) {
    page = parseInt(route.query.page as string)
    initialPage = page
  }
  await fetchEntries(page)
}

const fetchEntries = async (page: number) => {
  const collageId = parseInt(route.params.id.toString())
  const result = await getCollageEntries({
    collage_id: collageId,
    page: page,
    page_size: pageSize.value,
  })

  entries.value = result.results
  totalItems.value = result.total_items
  currentPage.value = result.page
}

const changePage = (page: number) => {
  currentPage.value = page
  router.push({ query: { page } })
}

const onEntriesAdded = async () => {
  await fetchEntries(currentPage.value)
}

watch(
  () => route.query,
  () => {
    fetchEntriesFromUrl()
  },
  { deep: true },
)
</script>
<style scoped>
#collage-view {
  display: flex;
}
.main-content {
  width: 80%;
  margin-right: 10px;
}
.actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 5px;
}
.empty-message {
  text-align: center;
  padding: 20px;
  color: var(--p-text-muted-color);
}
</style>
