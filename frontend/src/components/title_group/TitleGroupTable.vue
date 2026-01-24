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
    :pt="{ rowGroupHeaderCell: { colspan: 9 } }"
    class="title-group-table"
    :showHeaders="false"
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
    <Column class="torrent-slug">
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
    <Column style="width: 7em; padding: 0">
      <template #body="slotProps">
        {{ timeAgo(slotProps.data.created_at) }}
      </template>
    </Column>
    <Column class="actions" style="width: 12em; padding: 0">
      <template #body="slotProps">
        <i
          v-if="userStore.permissions.includes('download_torrent')"
          v-tooltip.top="t('torrent.download')"
          class="action pi pi-download"
          @click="downloadTorrent(slotProps.data, title_group.name)"
        />
        <i v-tooltip.top="t('general.report')" class="action pi pi-flag" @click="reportTorrent(slotProps.data.id)" />
        <RouterLink :to="`/title-group/${title_group.id}?torrentId=${slotProps.data.id}`" style="color: white">
          <i v-tooltip.top="t('torrent.permalink')" class="action pi pi-link" />
        </RouterLink>
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
        <i
          v-if="showActionBtns && user.permissions.includes('edit_torrent_up_down_factors')"
          v-tooltip.top="t('torrent.edit_factors')"
          @click="editTorrentFactors(slotProps.data)"
          class="action pi pi-percentage"
        />
      </template>
    </Column>
    <Column style="width: 7em; padding: 0">
      <template #body="slotProps"> {{ bytesToReadable(slotProps.data.size) }} </template>
    </Column>
    <Column style="width: 7em; padding: 0; color: yellow">
      <template #body="slotProps">
        <span v-tooltip.top="t('torrent.bonus_points_snatch_cost_hint')">
          {{ slotProps.data.bonus_points_snatch_cost }}
        </span>
      </template>
    </Column>
    <Column style="width: 2em" class="tracker-stats">
      <template #body="slotProps">
        <span v-tooltip.top="t('torrent.times_completed', 2)">
          {{ slotProps.data.times_completed }}
        </span>
      </template>
    </Column>
    <Column style="width: 2em" class="tracker-stats">
      <template #body="slotProps">
        <span style="color: green" v-tooltip.top="t('torrent.seeder', 2)">{{ slotProps.data.seeders }}</span>
      </template>
    </Column>
    <Column style="width: 2em" class="tracker-stats">
      <template #body="slotProps">
        <span v-tooltip.top="t('torrent.leecher', 2)">
          {{ slotProps.data.leechers }}
        </span>
      </template>
    </Column>
    <template #groupheader="slotProps" v-if="groupBy !== undefined">
      <div class="edition-group-header">
        <template v-if="groupBy === 'edition_group_id'">
          {{ getEditionGroupSlugById(slotProps.data.edition_group_id) }}
          <i
            v-if="
              showActionBtns &&
              (user.permissions.includes('edit_edition_group') || getEditionGroupCreatorIdById(slotProps.data.edition_group_id) === userStore.id)
            "
            v-tooltip.top="t('edition_group.edit_edition_group')"
            @click="editEditionGroup(slotProps.data.edition_group_id)"
            class="action pi pi-pen-to-square"
            style="color: white; margin-left: 3px; font-size: 0.8em"
          />
        </template>
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
      <Accordion v-model:value="activeAccordionPanels[slotProps.data.id]" multiple class="dense-accordion">
        <AccordionPanel value="5" v-if="slotProps.data.trumpable">
          <AccordionHeader>{{ t('torrent.trump_reason') }}</AccordionHeader>
          <AccordionContent>
            {{ slotProps.data.trumpable }}
          </AccordionContent>
        </AccordionPanel>
        <AccordionPanel value="0" v-if="slotProps.data.reports.length > 0">
          <AccordionHeader>Report information</AccordionHeader>
          <AccordionContent>
            <TorrentReportsList :reports="slotProps.data.reports" @deleted="(reportId) => reportDeleted(slotProps.data, reportId)" />
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
        <AccordionPanel value="6" v-if="userStore.permissions.includes('view_torrent_peers')">
          <AccordionHeader>{{ t('torrent.peers') }}</AccordionHeader>
          <AccordionContent>
            <!-- Only load the component when the accordion panel is open -->
            <TorrentPeerTable v-if="activeAccordionPanels[slotProps.data.id]?.includes('6')" :torrentId="slotProps.data.id" />
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
  <Dialog closeOnEscape modal :header="t('edition_group.edit_edition_group')" v-model:visible="editEditionGroupDialogVisible">
    <CreateOrEditEditionGroup
      v-if="editionGroupBeingEdited !== null"
      :titleGroup="title_group"
      :initialEditionGroupForm="editionGroupBeingEdited"
      :sendingEditionGroup="sendingEditionGroup"
      @validated="editionGroupEdited"
    />
  </Dialog>
  <Dialog closeOnEscape modal :header="t('torrent.edit_factors')" v-model:visible="editFactorsDialogVisible">
    <EditTorrentFactorsDialog
      v-if="torrentBeingEditedFactors !== null"
      :torrentId="torrentBeingEditedFactors.id"
      :initialUploadFactor="torrentBeingEditedFactors.upload_factor"
      :initialDownloadFactor="torrentBeingEditedFactors.download_factor"
      @done="torrentFactorsEdited"
    />
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
import CreateOrEditEditionGroup from '../edition_group/CreateOrEditEditionGroup.vue'
import { useUserStore } from '@/stores/user'
import { useEditionGroupStore } from '@/stores/editionGroup'
import ImagePreview from '../ImagePreview.vue'
import MediaInfoPreview from '@/components/mediainfo/MediaInfoPreview.vue'
import {
  editEditionGroup as editEditionGroupApi,
  setTorrentStaffChecked,
  type EditedTorrent,
  type EditionGroupHierarchy,
  type EditionGroupHierarchyLite,
  type EditionGroupInfoLite,
  type TitleGroup,
  type TitleGroupHierarchyLite,
  type TorrentHierarchyLite,
  type TorrentReport,
  type UserCreatedEditionGroup,
} from '@/services/api-schema'
import UsernameEnriched from '../user/UsernameEnriched.vue'
import TorrentPeerTable from '../torrent/TorrentPeerTable.vue'
import EditTorrentFactorsDialog from '../torrent/EditTorrentFactorsDialog.vue'
import TorrentReportsList from '../torrent/TorrentReportsList.vue'

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
const editEditionGroupDialogVisible = ref(false)
const editFactorsDialogVisible = ref(false)
const torrentBeingEditedFactors = ref<{ id: number; upload_factor: number; download_factor: number } | null>(null)
const torrentBeingEdited = ref<EditedTorrent | null>(null)
const editionGroupBeingEdited = ref<UserCreatedEditionGroup | null>(null)
const editionGroupIdBeingEdited = ref<number | null>(null)
const sendingEditionGroup = ref(false)
const expandedRows = ref<TorrentHierarchyLite[]>([])
const torrentIdBeingReported = ref(0)
const torrentIdBeingDeleted = ref(0)
const route = useRoute()
const user = useUserStore()
const activeAccordionPanels = ref<Record<number, string[]>>({})

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

const reportDeleted = (torrent: TorrentHierarchyLite, reportId: number) => {
  torrent.reports = torrent.reports.filter((r) => r.id !== reportId)
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
  useEditionGroupStore().source = getEditionGroupById(torrent.edition_group_id).source
  editTorrentDialogVisible.value = true
}
const editEditionGroup = (editionGroupId: number) => {
  const eg = editionGroups.find((g) => g.id === editionGroupId) as EditionGroupHierarchy
  editionGroupIdBeingEdited.value = editionGroupId
  editionGroupBeingEdited.value = {
    name: eg.name,
    description: eg.description,
    external_links: eg.external_links,
    covers: eg.covers,
    release_date: eg.release_date,
    title_group_id: eg.title_group_id,
    source: eg.source,
    distributor: eg.distributor,
    additional_information: eg.additional_information,
  }
  editEditionGroupDialogVisible.value = true
}
const editionGroupEdited = async (updatedEditionGroup: UserCreatedEditionGroup) => {
  if (editionGroupIdBeingEdited.value === null) return
  const eg = editionGroups.find((g) => g.id === editionGroupIdBeingEdited.value)
  if (!eg) return
  sendingEditionGroup.value = true
  editEditionGroupApi({
    ...updatedEditionGroup,
    id: editionGroupIdBeingEdited.value,
  })
    .then((result) => {
      Object.assign(eg, result)
      editEditionGroupDialogVisible.value = false
    })
    .finally(() => (sendingEditionGroup.value = false))
}
const deleteTorrent = (torrentId: number) => {
  torrentIdBeingDeleted.value = torrentId
  deleteTorrentDialogVisible.value = true
}
const editTorrentFactors = (torrent: TorrentHierarchyLite) => {
  torrentBeingEditedFactors.value = {
    id: torrent.id,
    upload_factor: torrent.upload_factor,
    download_factor: torrent.download_factor,
  }
  editFactorsDialogVisible.value = true
}
const torrentFactorsEdited = (uploadFactor: number, downloadFactor: number) => {
  if (torrentBeingEditedFactors.value === null) return
  editionGroups.forEach((eg) => {
    const torrent = eg.torrents.find((t) => t.id === torrentBeingEditedFactors.value!.id)
    if (torrent) {
      torrent.upload_factor = uploadFactor
      torrent.download_factor = downloadFactor
    }
  })
  editFactorsDialogVisible.value = false
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
const getEditionGroupCreatorIdById = (editionGroupId: number): number => {
  return (editionGroups as EditionGroupHierarchy[]).find((group) => group.id === editionGroupId)!.created_by_id
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
    min-width: 10em;
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
