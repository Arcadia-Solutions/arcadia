<template>
  <DataTable
    v-model:expandedRows="expandedRows"
    :value="sortedTorrents"
    rowGroupMode="subheader"
    :groupRowsBy="groupBy"
    sortMode="single"
    :sortField="['edition', 'video_resolution', 'audio_codec'].includes(sortBy) ? '' : sortBy"
    :sortOrder="1"
    tableStyle="min-width: 35rem"
    size="small"
    :pt="{ rowGroupHeaderCell: { colspan: 8 } }"
    class="title-group-table"
  >
    <!-- <Column expander style="width: 1em" v-if="!preview" class="expander" />
    <Column style="width: 1em" v-else /> -->
    <Column style="width: 1em">
      <template #body="slotProps">
        <i
          class="pi pi-verified"
          :style="{ color: slotProps.data.staff_checked ? 'green' : 'grey', 'margin-top': '0.3em' }"
          v-tooltip.top="slotProps.data.staff_checked ? t('torrent.staff_check_present') : t('torrent.staff_check_missing')"
        />
      </template>
    </Column>
    <Column :header="t('torrent.properties')" style="min-width: 15em" class="torrent-slug">
      <template #body="slotProps">
        <div class="cursor-pointer">
          <RouterLink v-if="preview" :to="`/title-group/${title_group.id}?torrentId=${slotProps.data.id}`">
            <TorrentSlug
              :contentType="title_group.content_type"
              :torrent="slotProps.data"
              :editionGroup="getEditionGroupById(slotProps.data.edition_group_id)"
              :sortedBy="sortBy"
            />
          </RouterLink>
          <a v-else @click="toggleRow(slotProps.data)">
            <TorrentSlug
              :contentType="title_group.content_type"
              :torrent="slotProps.data"
              :editionGroup="getEditionGroupById(slotProps.data.edition_group_id)"
              :sortedBy="sortBy"
            />
          </a>
        </div>
      </template>
    </Column>
    <Column :header="t('general.uploaded')" style="width: 7em; padding: 0">
      <template #body="slotProps">
        {{ timeAgo(slotProps.data.created_at) }}
      </template>
    </Column>
    <Column header="" class="actions" style="width: 12em; padding: 0">
      <template #body="slotProps">
        <i
          v-if="userStore.permissions.includes('download_torrent')"
          v-tooltip.top="t('torrent.download')"
          class="action pi pi-download"
          @click="downloadTorrent(slotProps.data, title_group.name)"
        />
        <i v-tooltip.top="t('general.report')" class="action pi pi-flag" @click="reportTorrent(slotProps.data.id)" />
        <i v-tooltip.top="t('torrent.copy_permalink')" class="action pi pi-link" />
        <i
          v-tooltip.top="t('general.delete')"
          class="action pi pi-trash"
          v-if="showActionBtns && (user.id === slotProps.data.created_by_id || user.permissions.includes('delete_torrent'))"
          @click="deleteTorrent(slotProps.data.id)"
        />
        <i
          v-if="showActionBtns && (user.id === slotProps.data.created_by_id || user.permissions.includes('edit_torrent'))"
          v-tooltip.top="t('general.edit')"
          @click="editTorrent(slotProps.data)"
          class="action pi pi-pen-to-square"
        />
        <i
          v-if="showActionBtns && user.permissions.includes('set_torrent_staff_checked')"
          v-tooltip.top="t(`torrent.${slotProps.data.staff_checked ? 'unset_staff_checked' : 'set_staff_checked'}`)"
          @click="toggleTorrentStaffChecked({ torrent_id: slotProps.data.id, staff_checked: !slotProps.data.staff_checked })"
          :class="{
            action: true,
            pi: true,
            'pi-verified': settingTorrentIdStaffChecked !== slotProps.data.id,
            'pi-hourglass': settingTorrentIdStaffChecked === slotProps.data.id,
          }"
          :style="`color: ${slotProps.data.staff_checked ? 'green' : 'white'}`"
        />
      </template>
    </Column>
    <Column :header="t('torrent.size')" style="width: 7em; padding: 0">
      <template #body="slotProps"> {{ bytesToReadable(slotProps.data.size) }} </template>
    </Column>
    <!-- TODO: replace with real data from the tracker -->
    <Column style="width: 2em" class="tracker-stats">
      <template #header>
        <div style="text-align: center">
          <i class="pi pi-replay" v-tooltip.top="t('torrent.times_completed')" />
        </div>
      </template>
      <template #body="slotProps">{{ slotProps.data.times_completed }}</template>
    </Column>
    <Column style="width: 2em" class="tracker-stats">
      <template #header>
        <i class="pi pi-arrow-up" v-tooltip.top="t('torrent.seeder', 2)" />
      </template>
      <template #body="slotProps">
        <span style="color: green">{{ slotProps.data.seeders }}</span>
      </template>
    </Column>
    <Column style="width: 2em" class="tracker-stats">
      <template #header>
        <i class="pi pi-arrow-down" v-tooltip.top="t('torrent.leecher', 2)" />
      </template>
      <template #body="slotProps">{{ slotProps.data.leechers }}</template>
    </Column>
    <template #groupheader="slotProps" v-if="groupBy !== undefined">
      <div class="edition-group-header">
        <template v-if="groupBy === 'edition_group_id'">{{ getEditionGroupSlugById(slotProps.data.edition_group_id) }}</template>
        <template v-else-if="groupBy === 'video_resolution'">{{ slotProps.data.video_resolution }}</template>
        <template v-else-if="groupBy === 'audio_codec'">{{ slotProps.data.audio_codec }}</template>
      </div>
    </template>
    <template #expansion="slotProps" v-if="!preview">
      <div class="pre-style release-name">{{ slotProps.data.release_name }}</div>
      <div class="uploader">
        <span>{{ t('torrent.uploaded_by') }}</span>
        <UsernameEnriched v-if="slotProps.data.created_by" :user="slotProps.data.created_by" />
        <span v-else>{{ t('general.anonymous') }}</span>
      </div>
      <Accordion :value="[]" multiple class="dense-accordion">
        <AccordionPanel value="5" v-if="slotProps.data.trumpable">
          <AccordionHeader>{{ t('torrent.trump_reason') }}</AccordionHeader>
          <AccordionContent>
            {{ slotProps.data.trumpable }}
          </AccordionContent>
        </AccordionPanel>
        <AccordionPanel value="0" v-if="slotProps.data.reports.length > 0">
          <AccordionHeader>Report information</AccordionHeader>
          <AccordionContent>
            <div class="report" v-for="report in slotProps.data.reports" :key="report.id">
              <span class="bold">{{ timeAgo(report.reported_at) }}</span
              >: {{ report.description }}
            </div>
          </AccordionContent>
        </AccordionPanel>
        <AccordionPanel v-if="slotProps.data.description" value="2">
          <AccordionHeader>{{ t('general.description') }}</AccordionHeader>
          <AccordionContent>
            <BBCodeRenderer :content="slotProps.data.description" />
          </AccordionContent>
        </AccordionPanel>
        <AccordionPanel value="1" v-if="slotProps.data.mediainfo !== null">
          <AccordionHeader class="aa">
            <div class="header-text">{{ t('torrent.mediainfo') }}</div>
          </AccordionHeader>
          <AccordionContent>
            <MediaInfoPreview :mediainfo="slotProps.data.mediainfo" />
          </AccordionContent>
        </AccordionPanel>
        <AccordionPanel v-if="slotProps.data.screenshots && slotProps.data.screenshots.length > 0" value="3">
          <AccordionHeader>{{ t('general.screenshots') }}</AccordionHeader>
          <AccordionContent>
            <div class="screenshots-container">
              <div v-for="(screenshot, index) in slotProps.data.screenshots" :key="index" class="screenshot">
                <ImagePreview class="screenshot-image" :imageLink="screenshot" />
              </div>
            </div>
          </AccordionContent>
        </AccordionPanel>
        <AccordionPanel value="4">
          <AccordionHeader>{{ t('torrent.file_list') }}</AccordionHeader>
          <AccordionContent>
            <DataTable :value="slotProps.data.file_list.files" tableStyle="min-width: 50rem">
              <Column field="name" :header="(slotProps.data.file_list.parent_folder ?? '') + '/'"></Column>
              <Column field="size" :header="t('torrent.size')">
                <template #body="slotProps">
                  {{ bytesToReadable(slotProps.data.size) }}
                </template>
              </Column>
            </DataTable>
          </AccordionContent>
        </AccordionPanel>
      </Accordion>
    </template>
  </DataTable>
  <Dialog closeOnEscape modal :header="t('torrent.report_torrent')" v-model:visible="reportTorrentDialogVisible">
    <ReportTorrentDialog :torrentId="torrentIdBeingReported" @reported="torrentReported" />
  </Dialog>
  <Dialog closeOnEscape modal :header="t('torrent.delete_torrent')" v-model:visible="deleteTorrentDialogVisible">
    <DeleteTorrentDialog :torrentId="torrentIdBeingDeleted" @deleted="torrentDeleted" />
  </Dialog>
  <Dialog closeOnEscape modal :header="t('torrent.edit_torrent')" v-model:visible="editTorrentDialogVisible">
    <CreateOrEditTorrent v-if="torrentBeingEdited !== null" :initialTorrent="torrentBeingEdited" @done="torrentEdited" />
  </Dialog>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import DataTable from 'primevue/datatable'
import TorrentSlug from '../torrent/TorrentSlug.vue'
import Column from 'primevue/column'
import BBCodeRenderer from '@/components/community/BBCodeRenderer.vue'
import Accordion from 'primevue/accordion'
import AccordionPanel from 'primevue/accordionpanel'
import AccordionHeader from 'primevue/accordionheader'
import AccordionContent from 'primevue/accordioncontent'
import ReportTorrentDialog from '../torrent/ReportTorrentDialog.vue'
import DeleteTorrentDialog from '../torrent/DeleteTorrentDialog.vue'
import Dialog from 'primevue/dialog'
import { downloadTorrent } from '@/services/api/torrentService'
import { useRoute } from 'vue-router'
import { bytesToReadable, getEditionGroupSlug, timeAgo } from '@/services/helpers'
import { useI18n } from 'vue-i18n'
import { RouterLink } from 'vue-router'
import CreateOrEditTorrent from '../torrent/CreateOrEditTorrent.vue'
import { useUserStore } from '@/stores/user'
import { useEditionGroupStore } from '@/stores/editionGroup'
import ImagePreview from '../ImagePreview.vue'
import MediaInfoPreview from '@/components/mediainfo/MediaInfoPreview.vue'
import {
  setTorrentStaffChecked,
  type EditedTorrent,
  type EditionGroupHierarchy,
  type EditionGroupHierarchyLite,
  type EditionGroupInfoLite,
  type TitleGroup,
  type TitleGroupHierarchyLite,
  type TorrentHierarchyLite,
  type TorrentReport,
} from '@/services/api-schema'
import UsernameEnriched from '../user/UsernameEnriched.vue'

interface Props {
  title_group: TitleGroup | TitleGroupHierarchyLite
  editionGroups: EditionGroupHierarchyLite[] | EditionGroupHierarchy[]
  preview: boolean
  sortBy?: string
  showActionBtns?: boolean
}
const { title_group, editionGroups, preview = false, sortBy = 'edition' } = defineProps<Props>()

const { t } = useI18n()
const userStore = useUserStore()

const settingTorrentIdStaffChecked = ref<number | null>(null)
const reportTorrentDialogVisible = ref(false)
const deleteTorrentDialogVisible = ref(false)
const editTorrentDialogVisible = ref(false)
const torrentBeingEdited = ref<EditedTorrent | null>(null)
const expandedRows = ref<TorrentHierarchyLite[]>([])
const torrentIdBeingReported = ref(0)
const torrentIdBeingDeleted = ref(0)
const route = useRoute()
const user = useUserStore()

const toggleTorrentStaffChecked = async ({ torrent_id, staff_checked }: { torrent_id: number; staff_checked: boolean }) => {
  settingTorrentIdStaffChecked.value = torrent_id
  setTorrentStaffChecked({ torrent_id, staff_checked })
    .then(() => {
      editionGroups.forEach((edition_group) => {
        edition_group.torrents.forEach((torrent) => {
          if (torrent.id === torrent_id) {
            torrent.staff_checked = staff_checked
          }
        })
      })
    })
    .finally(() => {
      settingTorrentIdStaffChecked.value = null
    })
}

const torrentReported = (torrentReport: TorrentReport) => {
  reportTorrentDialogVisible.value = false
  const reportedTorrent = editionGroups
    .flatMap((edition_group) => edition_group.torrents)
    .find((torrent: TorrentHierarchyLite) => torrent.id == torrentReport.reported_torrent_id)
  if (reportedTorrent) {
    if (reportedTorrent.reports) {
      reportedTorrent.reports.push(torrentReport)
    } else {
      reportedTorrent.reports = [torrentReport]
    }
  } else {
    console.error('torrent to report not found !')
  }
}
const torrentDeleted = () => {
  editionGroups.forEach((edition_group) => {
    edition_group.torrents = edition_group.torrents.filter((torrent) => torrent.id !== torrentIdBeingDeleted.value)
  })
  deleteTorrentDialogVisible.value = false
}
const reportTorrent = (id: number) => {
  torrentIdBeingReported.value = id
  reportTorrentDialogVisible.value = true
}
const editTorrent = (torrent: EditedTorrent) => {
  torrentBeingEdited.value = torrent
  useEditionGroupStore().additional_information = getEditionGroupById(torrent.edition_group_id).additional_information
  editTorrentDialogVisible.value = true
}
const deleteTorrent = (torrentId: number) => {
  torrentIdBeingDeleted.value = torrentId
  deleteTorrentDialogVisible.value = true
}
const toggleRow = (torrent: TorrentHierarchyLite) => {
  if (!expandedRows.value.some((expandedTorrent) => expandedTorrent.id === torrent.id)) {
    expandedRows.value = [...expandedRows.value, torrent]
  } else {
    expandedRows.value = expandedRows.value.filter((t) => t.id !== torrent.id)
  }
}

const getEditionGroupById = (editionGroupId: number): EditionGroupInfoLite => {
  return editionGroups.find((group) => group.id === editionGroupId) as EditionGroupInfoLite
}
const getEditionGroupSlugById = (editionGroupId: number): string => {
  const editionGroup = getEditionGroupById(editionGroupId)
  return editionGroup ? getEditionGroupSlug(editionGroup) : ''
}

onMounted(() => {
  const torrentIdParam = route.query.torrentId?.toString()
  if (torrentIdParam) {
    const torrentId = parseInt(torrentIdParam)
    const matchingTorrent = editionGroups.flatMap((edition_group) => edition_group.torrents).find((torrent) => torrent.id === torrentId)

    if (matchingTorrent) {
      toggleRow(matchingTorrent)
    }
  }
})
const sortedTorrents = computed(() => {
  const flatTorrents = editionGroups.flatMap((edition_group: EditionGroupHierarchyLite) => edition_group.torrents)

  switch (sortBy) {
    case 'video_resolution': {
      const resolutionOrder = ['SD', '720p', '1080p', '1440p', '2160p']
      return flatTorrents.sort((a, b) => {
        const aIndex = resolutionOrder.indexOf(a.video_resolution!)
        const bIndex = resolutionOrder.indexOf(b.video_resolution!)
        return aIndex - bIndex
      })
    }

    case 'audio_codec': {
      const codecOrder = ['flac', 'true-hd', 'aac', 'ac3', 'dts', 'mp3', 'opus', 'mp2', 'pcm', 'dsd']
      return flatTorrents.sort((a, b) => {
        const aIndex = codecOrder.indexOf(a.audio_codec!)
        const bIndex = codecOrder.indexOf(b.audio_codec!)
        return aIndex - bIndex
      })
    }

    default:
      return flatTorrents
  }

  return flatTorrents
})
const torrentEdited = (editedTorrent: EditedTorrent) => {
  editionGroups.forEach((eg) => {
    const index = eg.torrents.findIndex((t) => t.id === editedTorrent.id)
    if (index !== -1) {
      eg.torrents[index] = { ...eg.torrents[index], ...editedTorrent }
    }
  })
  editTorrentDialogVisible.value = false
}
const groupBy = computed(() => {
  switch (sortBy) {
    case 'edition':
      return 'edition_group_id'
    case 'video_resolution':
      return 'video_resolution'
    case 'audio_codec':
      return 'audio_codec'
    default:
      return undefined
  }
})
</script>
<style scoped>
.feature {
  font-weight: bold;
}
.action {
  margin-right: 4px;
  cursor: pointer;
}
.mediainfo {
  border: 2px dotted black;
  padding: 5px;
}
.edition-group-header {
  color: var(--color-primary);
}
.date {
  font-weight: bold;
}
.release-name {
  margin-bottom: 10px;
  margin-left: 7px;
}
.uploader {
  margin-left: 7px;
  margin-bottom: 10px;
  span {
    margin-right: 5px;
  }
}

.screenshots-container {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  margin-top: 10px;
}

.screenshot {
  max-width: 200px;
}

.screenshot-image {
  width: 100%;
  height: auto;
  border-radius: 4px;
}
</style>
<style>
.title-group-table {
  .torrent-slug {
    padding: 0 !important;
  }
  .p-datatable-header-cell {
    padding: 7px 0 !important;
  }
  .tracker-stats > .p-datatable-column-header-content {
    text-align: center;
    display: flex;
    justify-content: center;
  }
  .p-accordionheader.aa {
    align-items: baseline;
  }
}
</style>
