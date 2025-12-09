<template>
  <div v-if="torrentRequestAndAssociatedData" id="title-group-view" class="with-sidebar">
    <div class="main">
      <TitleGroupSlimHeader
        :titleGroup="torrentRequestAndAssociatedData.title_group"
        :series="torrentRequestAndAssociatedData.series"
        :nameLink="`/title-group/${torrentRequestAndAssociatedData.title_group.id}`"
        class="slim-header title"
      />
      <div class="actions">
        <div>
          <i v-if="togglingSubscription" class="pi pi-hourglass" />
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
          <i @click="uploadTorrent" v-tooltip.top="t('torrent.upload_torrent')" class="pi pi-upload" />
          <!-- <i @click="requestTorrent" v-tooltip.top="t('torrent.request_format')" class="pi pi-shopping-cart" /> -->
        </div>
      </div>
      <TorrentRequestDetails
        :torrentRequest="torrentRequestAndAssociatedData.torrent_request"
        :votes="torrentRequestAndAssociatedData.votes"
        :contentType="torrentRequestAndAssociatedData.title_group.content_type"
        @voted="voted"
      />
      <TorrentRequestVotesTable class="votes-table" :votes="torrentRequestAndAssociatedData.votes" />
      <!-- <ContentContainer :container-title="t('general.screenshots')" class="screenshots" v-if="titleGroupAndAssociatedData.title_group.screenshots.length !== 0">
        <CustomGalleria :images="titleGroupAndAssociatedData.title_group.screenshots" />
      </ContentContainer> -->
      <!-- <Accordion v-if="titleGroupAndAssociatedData.torrent_requests.length != 0" value="0" class="torrent-requests dense-accordion">
        <AccordionPanel value="0">
          <AccordionHeader> {{ t('torrent.requests') }} ({{ titleGroupAndAssociatedData.torrent_requests.length }}) </AccordionHeader>
          <AccordionContent>
            <TorrentRequestsTable
              :torrentRequests="titleGroupAndAssociatedData.torrent_requests"
              :contentType="titleGroupAndAssociatedData.title_group.content_type"
            />
          </AccordionContent>
        </AccordionPanel>
      </Accordion> -->
      <!-- <EmbeddedLinks
        class="embedded-links"
        v-if="Object.keys(titleGroupAndAssociatedData.title_group.embedded_links).length > 0"
        :links="titleGroupAndAssociatedData.title_group.embedded_links"
      /> -->
      <ContentContainer class="description" :container-title="t('title_group.description')">
        <BBCodeRenderer :content="torrentRequestAndAssociatedData.title_group.description" />
      </ContentContainer>
      <ContentContainer class="description" :container-title="t('torrent_request.description')">
        <BBCodeRenderer :content="torrentRequestAndAssociatedData.torrent_request.description" />
      </ContentContainer>
      <!-- <TitleGroupComments :comments="titleGroupAndAssociatedData.title_group_comments" @newComment="newComment" /> -->
    </div>
    <div class="sidebar">
      <TitleGroupSidebar
        :title_group="torrentRequestAndAssociatedData.title_group"
        :affiliatedArtists="torrentRequestAndAssociatedData.affiliated_artists"
        :series="torrentRequestAndAssociatedData.series"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import BBCodeRenderer from '@/components/community/BBCodeRenderer.vue'
import TitleGroupSidebar from '@/components/title_group/TitleGroupSidebar.vue'
import ContentContainer from '@/components/ContentContainer.vue'
import TorrentRequestVotesTable from '@/components/torrent_request/TorrentRequestVotesTable.vue'
// import { getTitleGroup, type TitleGroup, type TitleGroupAndAssociatedData } from '@/services/api/torrentService'
// import TitleGroupTable from '@/components/title_group/TitleGroupTable.vue'
// import TorrentRequestsTable from '@/components/torrent_request/TorrentRequestsTable.vue'
// import Accordion from 'primevue/accordion'
// import AccordionPanel from 'primevue/accordionpanel'
// import AccordionHeader from 'primevue/accordionheader'
// import AccordionContent from 'primevue/accordioncontent'
import TitleGroupSlimHeader from '@/components/title_group/TitleGroupSlimHeader.vue'
// import { subscribeToItem, unsubscribeToItem } from '@/services/api/generalService'
// import { useTitleGroupStore } from '@/stores/titleGroup'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
// import { showToast } from '@/main'
// import type { TitleGroupCommentHierarchy } from '@/services/api/commentService'
// import type { AffiliatedArtistHierarchy } from '@/services/api/artistService'
import TorrentRequestDetails from '@/components/torrent_request/TorrentRequestDetails.vue'
import { getTorrentRequest, type TorrentRequestAndAssociatedData, type TorrentRequestVoteHierarchy } from '@/services/api-schema'

const router = useRouter()
const route = useRoute()
const { t } = useI18n()

// const titleGroupStore = useTitleGroupStore()

const torrentRequestAndAssociatedData = ref<TorrentRequestAndAssociatedData>()
const togglingSubscription = ref(false)
// const siteName = import.meta.env.VITE_SITE_NAME

onMounted(async () => {
  await fetchTorrentRequest()
})

const fetchTorrentRequest = async () => {
  torrentRequestAndAssociatedData.value = await getTorrentRequest(parseInt(route.params.id.toString()))
  console.log(torrentRequestAndAssociatedData.value)

  /*
    For series, the title group name just holds the season name (i.e. 'Season 1')
    so we want to show the series name itself in the document title as well.
  */
  // document.title = titleGroupAndAssociatedData.value.series.name
  //   ? `${titleGroupAndAssociatedData.value.title_group.name} (${titleGroupAndAssociatedData.value.series.name}) - ${siteName}`
  //   : `${titleGroupAndAssociatedData.value.title_group.name} - ${siteName}`
}

// TODO: also include the edition groups in torrentRequestAndAssociatedData, or pass an argument to the upload page to fetch them
// const populateTitleGroupStore = () => {
//   if (torrentRequestAndAssociatedData.value) {
//     titleGroupStore.id = torrentRequestAndAssociatedData.value.title_group.id
//     titleGroupStore.original_release_date = torrentRequestAndAssociatedData.value.title_group.original_release_date
//     titleGroupStore.name = torrentRequestAndAssociatedData.value.title_group.name
//     // titleGroupStore.edition_groups = torrentRequestAndAssociatedData.value.edition_groups
//     titleGroupStore.content_type = torrentRequestAndAssociatedData.value.title_group.content_type
//   }
// }

const uploadTorrent = () => {
  // populateTitleGroupStore()
  router.push({ path: '/upload' })
}

// const toggleSubscribtion = async () => {
//   if (titleGroupAndAssociatedData.value) {
//     togglingSubscription.value = true
//     if (titleGroupAndAssociatedData.value.is_subscribed) {
//       await unsubscribeToItem(parseInt(route.params.id.toString()), 'title_group')
//     } else {
//       await subscribeToItem(parseInt(route.params.id.toString()), 'title_group')
//     }
//     titleGroupAndAssociatedData.value.is_subscribed = !titleGroupAndAssociatedData.value.is_subscribed
//     showToast(
//       'Success',
//       t(`title_group.${titleGroupAndAssociatedData.value.is_subscribed ? 'subscription_successful' : 'unsubscription_successful'}`),
//       'success',
//       3000,
//     )
//     togglingSubscription.value = false
//   }
// }

// const newComment = (comment: TitleGroupCommentHierarchy) => {
//   titleGroupAndAssociatedData.value?.title_group_comments.push(comment)
// }

const voted = (vote: TorrentRequestVoteHierarchy) => {
  if (torrentRequestAndAssociatedData.value) {
    torrentRequestAndAssociatedData.value.votes.push(vote)
  }
}

watch(() => route.params.id, fetchTorrentRequest, { immediate: true })
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
.votes-table {
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
