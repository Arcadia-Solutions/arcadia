<template>
  <div v-if="collageAndAssociatedData" id="collage-view">
    <div class="main-content">
      <div class="title">{{ collageAndAssociatedData.collage.name }}</div>
      <div class="actions">
        <div>
          <!-- <i v-if="togglingSubscription" class="pi pi-hourglass" /> -->
          <!-- <i
            v-else
            v-tooltip.top="t(`general.${titleGroupAndAssociatedData.is_subscribed ? 'un' : ''}subscribe`)"
            @click="toggleSubscribtion"
            :class="`pi pi-bell${titleGroupAndAssociatedData.is_subscribed ? '-slash' : ''}`"
          /> -->
          <i v-tooltip.top="t('general.bookmark')" class="pi pi-bookmark" />
        </div>
        <div>
          <!-- <i
            v-if="titleGroupAndAssociatedData.title_group.created_by_id === userStore.id || userStore.class === 'staff'"
            v-tooltip.top="t('general.edit')"
            class="pi pi-pen-to-square"
            @click="editTitleGroupDialogVisible = true"
          /> -->
          <i @click="addEntriesModalVisible = true" v-tooltip.top="t('collage.add_entry_to_collage', 2)" class="pi pi-plus cursor-pointer" />
          <!-- <i @click="requestTorrent" v-tooltip.top="t('torrent.request_format')" class="pi pi-shopping-cart" /> -->
        </div>
      </div>
      <TitleGroupList
        v-if="collageAndAssociatedData.collage.collage_type === 'TitleGroup'"
        :titleGroups="collageAndAssociatedData.entries.map((entry) => entry.title_group as TitleGroupHierarchyLite)"
        :titleGroupPreview
      />
      <!-- TODO: display Artists, Entities and Master Groups -->
    </div>
    <CollageSidebar :collage="collageAndAssociatedData.collage" />
    <Dialog modal :header="t('collage.add_entry_to_collage', 2)" v-model:visible="addEntriesModalVisible">
      <AddEntriesToCollageDialog
        :collageId="collageAndAssociatedData.collage.id"
        :collageType="collageAndAssociatedData.collage.collage_type"
        @addedEntries="router.go(0)"
      />
    </Dialog>
  </div>
</template>
<script setup lang="ts">
import { ref } from 'vue'
import { onMounted } from 'vue'
import { useRoute } from 'vue-router'
import CollageSidebar from '@/components/collage/CollageSidebar.vue'
import TitleGroupList, { type titleGroupPreviewMode } from '@/components/title_group/TitleGroupList.vue'
import { Dialog } from 'primevue'
import AddEntriesToCollageDialog from '@/components/collage/AddEntriesToCollageDialog.vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import { getCollage, type CollageAndAssociatedData, type TitleGroupHierarchyLite } from '@/services/api-schema'

const { t } = useI18n()

const route = useRoute()
const router = useRouter()
const siteName = import.meta.env.VITE_SITE_NAME
const collageAndAssociatedData = ref<CollageAndAssociatedData>()
const titleGroupPreview = ref<titleGroupPreviewMode>('table') // TODO: make a select button to switch from cover-only to table

onMounted(async () => {
  await fetchCollage()
})

const addEntriesModalVisible = ref(false)

const fetchCollage = async () => {
  collageAndAssociatedData.value = await getCollage(parseInt(route.params.id.toString()))
  document.title = `${collageAndAssociatedData.value.collage.name} - ${siteName}`
}
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
</style>
