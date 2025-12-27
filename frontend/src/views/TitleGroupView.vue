<template>
  <!-- TODO: use skeletons while the data is loading -->
  <div v-if="titleGroupAndAssociatedData" id="title-group-view" class="with-sidebar">
    <div
      :class="{
        main: true,
      }"
    >
      <TitleGroupSlimHeader :titleGroup="titleGroupAndAssociatedData.title_group" :series="titleGroupAndAssociatedData.series" class="slim-header title" />
      <div class="actions">
        <div>
          <i v-if="togglingSubscription" class="pi pi-hourglass" />
          <i
            v-else
            v-tooltip.top="t(`general.${titleGroupAndAssociatedData.is_subscribed ? 'un' : ''}subscribe`)"
            @click="toggleSubscribtion"
            :class="`pi pi-bell${titleGroupAndAssociatedData.is_subscribed ? '-slash' : ''}`"
          />
          <i v-tooltip.top="t('general.bookmark')" class="pi pi-bookmark" />
        </div>
        <div>
          <i
            v-if="titleGroupAndAssociatedData.title_group.created_by_id === userStore.id || userStore.permissions.includes('edit_title_group')"
            v-tooltip.top="t('general.edit')"
            class="pi pi-pen-to-square"
            @click="editTitleGroupDialogVisible = true"
          />
          <i @click="uploadTorrent" v-tooltip.top="t('torrent.add_format')" class="pi pi-upload" />
          <i @click="requestTorrent" v-tooltip.top="t('torrent.request_format')" class="pi pi-shopping-cart" />
          <i @click="addCollagesDialogVisible = true" v-tooltip.top="t('collage.add_collage_to_entry', 2)" class="pi pi-folder-plus" />
        </div>
        <FloatLabel class="sort-by-select" variant="on">
          <Select v-model="sortBy" inputId="sort_by" :options="selectableSortingOptions" class="select" size="small">
            <template #option="slotProps">
              <span>{{ t(`torrent.${slotProps.option}`) }}</span>
            </template>
            <template #value="slotProps">
              <span>{{ t(`torrent.${slotProps.value}`) }}</span>
            </template>
          </Select>
          <label for="sort_by">{{ t('general.sort_by') }}</label>
        </FloatLabel>
      </div>
      <TitleGroupTable
        :showActionBtns="true"
        :title_group="titleGroupAndAssociatedData.title_group"
        :editionGroups="titleGroupAndAssociatedData.edition_groups"
        :sortBy
        :preview="false"
      />
      <ContentContainer :container-title="t('general.screenshots')" class="screenshots" v-if="titleGroupAndAssociatedData.title_group.screenshots.length !== 0">
        <CustomGalleria :images="titleGroupAndAssociatedData.title_group.screenshots" />
      </ContentContainer>
      <Accordion v-if="titleGroupAndAssociatedData.collages.length != 0" value="0" class="dense-accordion">
        <AccordionPanel value="0">
          <AccordionHeader> {{ t('collage.collage', 2) }} ({{ titleGroupAndAssociatedData.collages.length }}) </AccordionHeader>
          <AccordionContent>
            <CollagesTable :collages="titleGroupAndAssociatedData.collages" />
          </AccordionContent>
        </AccordionPanel>
      </Accordion>
      <Accordion v-if="titleGroupAndAssociatedData.torrent_requests.length != 0" value="0" class="dense-accordion">
        <AccordionPanel value="0">
          <AccordionHeader> {{ t('torrent.requests') }} ({{ titleGroupAndAssociatedData.torrent_requests.length }}) </AccordionHeader>
          <AccordionContent>
            <TorrentRequestsTable
              :torrentRequests="titleGroupAndAssociatedData.torrent_requests"
              :contentType="titleGroupAndAssociatedData.title_group.content_type"
            />
          </AccordionContent>
        </AccordionPanel>
      </Accordion>
      <EmbeddedLinks
        class="embedded-links"
        v-if="Object.keys(titleGroupAndAssociatedData.title_group.trailers).length > 0"
        :links="titleGroupAndAssociatedData.title_group.trailers"
      />
      <ContentContainer class="description" :container-title="t('title_group.description')">
        <div class="title-group-description">
          <BBCodeRenderer :content="titleGroupAndAssociatedData.title_group.description" />
        </div>
        <div v-for="edition_group in titleGroupAndAssociatedData.edition_groups" :key="edition_group.id">
          <div v-if="edition_group.description" class="edition-description">
            <div class="edition-group-slug">{{ getEditionGroupSlug(edition_group) }}</div>
            <BBCodeRenderer :content="edition_group.description" />
          </div>
        </div>
      </ContentContainer>
      <TitleGroupRatings
        v-if="titleGroupAndAssociatedData.title_group.public_ratings.length > 0"
        :publicRatings="titleGroupAndAssociatedData.title_group.public_ratings"
        class="ratings"
      />
      <TitleGroupComments :comments="titleGroupAndAssociatedData.title_group_comments" @newComment="newComment" @commentEdited="commentEdited" />
    </div>
    <div class="sidebar">
      <TitleGroupSidebar
        :title_group="titleGroupAndAssociatedData.title_group"
        :inSameMasterGroup="titleGroupAndAssociatedData.in_same_master_group"
        :affiliatedArtists="titleGroupAndAssociatedData.affiliated_artists"
        :affiliatedEntities="titleGroupAndAssociatedData.affiliated_entities"
        :series="titleGroupAndAssociatedData.series"
        editAffiliationBtns
        @edit-affiliated-artists-clicked="editAffiliatedArtistsDialogVisible = true"
        @tag-applied="titleGroupAndAssociatedData.title_group.tags.push($event)"
        @tag-removed="titleGroupAndAssociatedData.title_group.tags = titleGroupAndAssociatedData.title_group.tags.filter((tag_name) => $event != tag_name)"
      />
    </div>
    <Dialog modal :header="t('title_group.edit_affiliated_artists')" v-model:visible="editAffiliatedArtistsDialogVisible">
      <EditArtistsModal
        :artists-affiliations="
          titleGroupAndAssociatedData.affiliated_artists.length === 0
            ? [{ artist_id: 0, nickname: null, roles: [], title_group_id: 0 }]
            : titleGroupAndAssociatedData.affiliated_artists
        "
        :content-type="titleGroupAndAssociatedData.title_group.content_type"
        :title-group-id="titleGroupAndAssociatedData.title_group.id"
        @cancelled="editAffiliatedArtistsDialogVisible = false"
        @done="affiliatedArtistsEdited"
      />
    </Dialog>
    <Dialog closeOnEscape modal :header="t('title_group.edit_title_group')" v-model:visible="editTitleGroupDialogVisible">
      <CreateOrEditTitleGroup
        class="edit-title-group"
        v-if="titleGroupAndAssociatedData"
        :initialTitleGroup="titleGroupAndAssociatedData.title_group"
        editMode
        @done="titleGroupEdited"
      />
    </Dialog>
    <Dialog modal :header="t('collage.add_collage_to_entry', 2)" v-model:visible="addCollagesDialogVisible">
      <AddCollagesToEntryDialog :titleGroupId="titleGroupAndAssociatedData.title_group.id" @addedEntries="router.go(0)" />
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useUserStore } from '@/stores/user'
import BBCodeRenderer from '@/components/community/BBCodeRenderer.vue'
import TitleGroupComments from '@/components/title_group/TitleGroupComments.vue'
import TitleGroupSidebar from '@/components/title_group/TitleGroupSidebar.vue'
import ContentContainer from '@/components/ContentContainer.vue'
import TitleGroupTable from '@/components/title_group/TitleGroupTable.vue'
import TorrentRequestsTable from '@/components/torrent_request/TorrentRequestsTable.vue'
import Accordion from 'primevue/accordion'
import AccordionPanel from 'primevue/accordionpanel'
import AccordionHeader from 'primevue/accordionheader'
import AccordionContent from 'primevue/accordioncontent'
import TitleGroupSlimHeader from '@/components/title_group/TitleGroupSlimHeader.vue'
import { useTitleGroupStore } from '@/stores/titleGroup'
import TitleGroupRatings from '@/components/title_group/TitleGroupRatings.vue'
import FloatLabel from 'primevue/floatlabel'
import Select from 'primevue/select'
import CustomGalleria from '@/components/CustomGalleria.vue'
import { useRoute, useRouter } from 'vue-router'
import { getEditionGroupSlug } from '@/services/helpers'
import { useI18n } from 'vue-i18n'
import { showToast } from '@/main'
import EditArtistsModal from '@/components/artist/EditArtistsModal.vue'
import { Dialog } from 'primevue'
import EmbeddedLinks from '@/components/title_group/EmbeddedLinks.vue'
import CreateOrEditTitleGroup from '@/components/title_group/CreateOrEditTitleGroup.vue'
import { useEditionGroupStore } from '@/stores/editionGroup'
import { onBeforeRouteLeave } from 'vue-router'
import AddCollagesToEntryDialog from '@/components/collage/AddCollagesToEntryDialog.vue'
import CollagesTable from '@/components/collage/CollagesTable.vue'
import {
  getTitleGroup,
  removeTitleGroupTorrentsSubscription,
  type AffiliatedArtistHierarchy,
  type EditedTitleGroupComment,
  type TitleGroup,
  type TitleGroupAndAssociatedData,
  type TitleGroupCommentHierarchy,
} from '@/services/api-schema'

const router = useRouter()
const route = useRoute()
const { t } = useI18n()

const editAffiliatedArtistsDialogVisible = ref(false)
const addCollagesDialogVisible = ref(false)
const userStore = useUserStore()
const titleGroupStore = useTitleGroupStore()
const editTitleGroupDialogVisible = ref(false)

// TODO: add by extras
const selectableSortingOptions = ['edition', 'size', 'seeders', 'times_completed', 'created_at']

const titleGroupAndAssociatedData = ref<TitleGroupAndAssociatedData>()
const sortBy = ref('edition')
const togglingSubscription = ref(false)
const siteName = import.meta.env.VITE_SITE_NAME

const commentEdited = (editedComment: EditedTitleGroupComment, commentId: number) => {
  if (!titleGroupAndAssociatedData.value) return
  const index = titleGroupAndAssociatedData.value.title_group_comments.findIndex((comment) => comment.id === commentId)
  if (index !== -1) {
    titleGroupAndAssociatedData.value.title_group_comments[index] = { ...titleGroupAndAssociatedData.value.title_group_comments[index], ...editedComment }
    showToast('', t('title_group.comment_edited_success'), 'success', 2000)
  }
}

onMounted(async () => {
  await fetchTitleGroup()
})

const fetchTitleGroup = async () => {
  titleGroupAndAssociatedData.value = await getTitleGroup(parseInt(route.params.id.toString()))

  // add audio_codec to sorting options
  const audioCodecInSortingOptions = selectableSortingOptions.includes('audio_codec')
  const contentTypeShouldHaveAudioCodec = ['tv_show', 'movie', 'music'].includes(titleGroupAndAssociatedData.value.title_group.content_type)
  if (contentTypeShouldHaveAudioCodec && !audioCodecInSortingOptions) selectableSortingOptions.unshift('audio_codec')
  else if (!contentTypeShouldHaveAudioCodec && audioCodecInSortingOptions) selectableSortingOptions.splice(selectableSortingOptions.indexOf('audio_codec'), 1)

  // add video_resolution to sorting options
  const resolutionInSortingOptions = selectableSortingOptions.includes('video_resolution')
  const contentTypeShouldHaveResolution = ['tv_show', 'movie'].includes(titleGroupAndAssociatedData.value.title_group.content_type)
  if (contentTypeShouldHaveResolution && !resolutionInSortingOptions) selectableSortingOptions.unshift('video_resolution')
  else if (!contentTypeShouldHaveResolution && resolutionInSortingOptions) selectableSortingOptions.splice(selectableSortingOptions.indexOf('resolution'), 1)
  /*
    For series, the title group name just holds the season name (i.e. 'Season 1')
    so we want to show the series name itself in the document title as well.
  */
  document.title = titleGroupAndAssociatedData.value.series.name
    ? `${titleGroupAndAssociatedData.value.title_group.name} (${titleGroupAndAssociatedData.value.series.name}) - ${siteName}`
    : `${titleGroupAndAssociatedData.value.title_group.name} - ${siteName}`

  populateTitleGroupStore()
}

onBeforeRouteLeave((to, from, next) => {
  if (to.name !== 'UploadTorrent' && to.name !== 'NewTorrentRequest') {
    titleGroupStore.$reset()
    useEditionGroupStore().$reset()
  }
  // we must call `next()` to continue navigation
  next()
})

const populateTitleGroupStore = () => {
  if (titleGroupAndAssociatedData.value) {
    titleGroupStore.id = titleGroupAndAssociatedData.value.title_group.id
    titleGroupStore.original_release_date = titleGroupAndAssociatedData.value.title_group.original_release_date
    titleGroupStore.name = titleGroupAndAssociatedData.value.title_group.name
    titleGroupStore.edition_groups = titleGroupAndAssociatedData.value.edition_groups
    titleGroupStore.content_type = titleGroupAndAssociatedData.value.title_group.content_type
  }
}

const uploadTorrent = async () => {
  router.push({ path: '/upload' })
}

const requestTorrent = () => {
  router.push({ path: '/new-torrent-request' })
}

const toggleSubscribtion = async () => {
  if (titleGroupAndAssociatedData.value) {
    togglingSubscription.value = true
    if (titleGroupAndAssociatedData.value.is_subscribed) {
      await removeTitleGroupTorrentsSubscription(parseInt(route.params.id.toString()))
    } else {
      await removeTitleGroupTorrentsSubscription(parseInt(route.params.id.toString()))
    }
    titleGroupAndAssociatedData.value.is_subscribed = !titleGroupAndAssociatedData.value.is_subscribed
    showToast(
      'Success',
      t(`title_group.${titleGroupAndAssociatedData.value.is_subscribed ? 'subscription_successful' : 'unsubscription_successful'}`),
      'success',
      3000,
    )
    togglingSubscription.value = false
  }
}

const newComment = (comment: TitleGroupCommentHierarchy) => {
  titleGroupAndAssociatedData.value?.title_group_comments.push(comment)
}

const affiliatedArtistsEdited = (newAffiliatedArtists: AffiliatedArtistHierarchy[], removedAffiliatedArtistsIds: number[]) => {
  if (titleGroupAndAssociatedData.value) {
    titleGroupAndAssociatedData.value.affiliated_artists = titleGroupAndAssociatedData.value.affiliated_artists.filter((aa: AffiliatedArtistHierarchy) => {
      // removedAffiliatedArtistsIds.indexOf(aa.id) === -1
      // return aa
      return !removedAffiliatedArtistsIds.includes(aa.id)
    })
    titleGroupAndAssociatedData.value.affiliated_artists = titleGroupAndAssociatedData.value.affiliated_artists.concat(newAffiliatedArtists)
  }
  editAffiliatedArtistsDialogVisible.value = false
}

const titleGroupEdited = (updatedTitleGroup: TitleGroup) => {
  if (titleGroupAndAssociatedData.value) {
    titleGroupAndAssociatedData.value.title_group = { ...titleGroupAndAssociatedData.value.title_group, ...updatedTitleGroup }
  }
  editTitleGroupDialogVisible.value = false
}

watch(() => route.params.id, fetchTitleGroup, { immediate: true })
</script>

<style scoped>
.main {
  width: 75%;
}
.sidebar {
  width: 25%;
}
.actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 5px;
}
.actions i {
  margin: 0px 0.5em;
  color: white;
  cursor: pointer;
}
.screenshots {
  margin-top: 20px;
}
.dense-accordion {
  margin-top: 20px;
}
.embedded-links {
  margin-top: 20px;
}
.description {
  margin-top: 20px;
}
.title-group-description {
  margin-top: 10px;
  margin-bottom: 25px;
}
.edition-description {
  margin-top: 15px;
}
.edition-description .edition-group-slug {
  color: var(--color-primary);
  margin-bottom: 5px;
}
.ratings {
  margin-top: 20px;
}
.comments {
  margin-top: 20px;
}
.edit-title-group {
  width: 60vw;
}
</style>
<style>
#title-group-view {
  .p-tabpanel {
    line-height: 0 !important;
  }
  .p-tab {
    padding: 10px !important;
  }
  .p-tabpanels {
    padding: 0 !important;
  }
}
</style>
