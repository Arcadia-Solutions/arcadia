<template>
  <div class="site-highlights-manager">
    <div class="wrapper-center" style="color: white; margin-bottom: 20px">
      <i class="pi pi-plus cursor-pointer" v-tooltip.top="t('site_highlights.add_new')" @click="openDialog()" />
    </div>
    <DataTable :value="highlights" size="small" dataKey="id">
      <Column :header="t('site_highlights.alias')" field="alias" />
      <Column :header="t('site_highlights.item_type')" field="item_type" />
      <Column :header="t('site_highlights.item_id')">
        <template #body="slotProps">
          <RouterLink :to="siteHighlightLink(slotProps.data.item_type, itemIdOf(slotProps.data) ?? 0)">
            {{ itemIdOf(slotProps.data) }}
          </RouterLink>
        </template>
      </Column>
      <Column :header="t('site_highlights.forum_thread_id')">
        <template #body="slotProps">
          <RouterLink :to="`/forum/thread/${slotProps.data.forum_thread_id}`">
            {{ slotProps.data.forum_thread_id }}
          </RouterLink>
        </template>
      </Column>
      <Column :header="t('site_highlights.position')" field="position" />
      <Column :header="t('site_highlights.enabled')">
        <template #body="slotProps">
          <i :class="slotProps.data.enabled ? 'pi pi-check' : 'pi pi-times'" />
        </template>
      </Column>
      <Column :header="t('general.action', 2)">
        <template #body="slotProps">
          <i class="pi pi-pen-to-square cursor-pointer" v-tooltip.top="t('general.edit')" @click="openDialog(slotProps.data)" style="margin-right: 10px" />
          <i class="pi pi-trash cursor-pointer" v-tooltip.top="t('general.delete')" @click="removeHighlight(slotProps.data)" />
        </template>
      </Column>
    </DataTable>

    <Dialog closeOnEscape modal :header="highlightBeingEdited ? t('site_highlights.edit') : t('site_highlights.add_new')" v-model:visible="dialogVisible">
      <CreateOrEditSiteHighlight :initialHighlight="highlightBeingEdited" @created="onHighlightCreated" @edited="onHighlightEdited" />
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import { Dialog } from 'primevue'
import CreateOrEditSiteHighlight from '@/components/staff/CreateOrEditSiteHighlight.vue'
import { listSiteHighlights, deleteSiteHighlight, type SiteHighlight } from '@/services/api-schema'
import { siteHighlightLink } from '@/services/helpers'
import { showToast } from '@/main'

const { t } = useI18n()

const highlights = ref<SiteHighlight[]>([])
const dialogVisible = ref(false)
const highlightBeingEdited = ref<SiteHighlight | undefined>()

const fetchHighlights = () => {
  listSiteHighlights().then((data) => {
    highlights.value = data
  })
}

const openDialog = (highlight?: SiteHighlight) => {
  highlightBeingEdited.value = highlight
  dialogVisible.value = true
}

const onHighlightCreated = (created: SiteHighlight) => {
  highlights.value.push(created)
  dialogVisible.value = false
}

const onHighlightEdited = (edited: SiteHighlight) => {
  const index = highlights.value.findIndex((h) => h.id === edited.id)
  if (index !== -1) {
    highlights.value[index] = edited
  }
  dialogVisible.value = false
}

const removeHighlight = (highlight: SiteHighlight) => {
  deleteSiteHighlight(highlight.id).then(() => {
    highlights.value = highlights.value.filter((h) => h.id !== highlight.id)
    showToast('Success', t('site_highlights.deleted'), 'success', 3000)
  })
}

const itemIdOf = (h: SiteHighlight) => h.title_group_id ?? h.series_id ?? h.artist_id

onMounted(() => {
  fetchHighlights()
})
</script>

<style scoped></style>
