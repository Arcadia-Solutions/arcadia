<template>
  <div id="title-group-sidebar">
    <Galleria
      :value="title_group.covers"
      :numVisible="1"
      :circular="true"
      :showItemNavigators="false"
      :showThumbnails="false"
      :showIndicators="true"
      class="carousel"
    >
      <template #item="slotProps">
        <ImagePreview :imageLink="slotProps.item" />
      </template>
    </Galleria>
    <ContentContainer :container-title="t('general.link', 2)">
      <div class="external-links links">
        <ExternalLink v-for="link in title_group.external_links" :key="link" :link="link" />
      </div>
    </ContentContainer>
    <ContentContainer :container-title="t('artist.artist', 2)">
      <template v-if="editAffiliationBtns" #top-right><i class="pi pi-pen-to-square cursor-pointer" @click="emit('editAffiliatedArtistsClicked')" /></template>
      <template v-if="affiliatedArtists.length != 0">
        <div class="affiliated-artists">
          <AffiliatedArtist class="affiliated-artist" v-for="artist in affiliatedArtists" :key="artist.artist_id" :affiliated_artist="artist" />
        </div>
      </template>
      <div class="wrapper-center" v-else>
        {{ t('artist.no_affiliated_artist_registered') }}
      </div>
    </ContentContainer>
    <ContentContainer :container-title="t('entity.entity', 2)" v-if="affiliatedEntities && affiliatedEntities?.length != 0">
      <!-- <template #top-right><i class="pi pi-pen-to-square cursor-pointer" @click="emit('editAffiliatedArtistsClicked')" /></template> -->
      <div class="affiliated-entities">
        <AffiliatedEntity class="affiliated-entity" v-for="entity in affiliatedEntities" :key="entity.entity_id" :affiliatedEntity="entity" />
      </div>
    </ContentContainer>
    <ContentContainer
      :container-title="`${t('master_group.in_same_master_group')} (${title_group.master_group_id})`"
      v-if="inSameMasterGroup && inSameMasterGroup?.length != 0"
    >
      <div class="flex justify-content-center links">
        <MasterGroupLink v-for="tg in inSameMasterGroup" :key="tg.id" :title_group="tg" />
      </div>
    </ContentContainer>
    <ContentContainer :container-title="t('series.series')" v-if="series">
      <div class="series">
        <RouterLink :to="`/series/${series.id}`">
          {{ series.name }}
        </RouterLink>
        <i
          v-if="userStore.permissions.includes('remove_title_group_from_series')"
          class="pi pi-times"
          @click="removeSeries"
          v-tooltip.top="t('title_group.remove_series')"
        />
      </div>
    </ContentContainer>
    <ContentContainer :container-title="t('general.alias', 2)" v-if="title_group.name_aliases.length > 0 && title_group.name_aliases[0] !== ''">
      <div class="aliases">
        <span v-for="(alias, index) in title_group.name_aliases" :key="alias"
          >{{ alias }}<span v-if="index !== title_group.name_aliases.length - 1">, </span></span
        >
      </div>
    </ContentContainer>
    <ContentContainer :container-title="t('general.tags')">
      <div class="tags" v-for="tag in title_group.tags" :key="tag">
        <RouterLink :to="{ path: '/torrents', query: { title_group_tags: tag } }">{{ tag }}</RouterLink>
        <i class="pi pi-times" @click="removeTag(tag)" v-tooltip.top="t('title_group.remove_tag')" />
      </div>
      <div>
        <div style="margin-top: 10px">
          <TitleGroupTagSearchBar :hideTags="title_group.tags" :placeholder="t('title_group.add_tag')" @tag-selected="applyTag" />
        </div>
      </div>
    </ContentContainer>
  </div>
</template>
<script setup lang="ts">
import { Galleria } from 'primevue'
import AffiliatedArtist from '@/components/artist/AffiliatedArtist.vue'
import ExternalLink from '@/components/ExternalLink.vue'
import MasterGroupLink from '@/components/MasterGroupLink.vue'
import ContentContainer from '../ContentContainer.vue'
import { useI18n } from 'vue-i18n'
import AffiliatedEntity from '../artist/AffiliatedEntity.vue'
import ImagePreview from '../ImagePreview.vue'
import TitleGroupTagSearchBar from './TitleGroupTagSearchBar.vue'
import {
  applyTagToTitleGroup,
  removeTitleGroupFromSeries,
  removeTagFromTitleGroup,
  type AffiliatedArtistHierarchy,
  type AffiliatedEntityHierarchy,
  type SeriesLite,
  type TitleGroup,
  type TitleGroupTagLite,
  type MasterGroupEntry,
} from '@/services/api-schema'
import { useUserStore } from '@/stores/user'

const { t } = useI18n()
const userStore = useUserStore()

const emit = defineEmits<{
  editAffiliatedArtistsClicked: []
  tagApplied: [string]
  tagRemoved: [string]
  seriesRemoved: []
}>()

const props = defineProps<{
  title_group: TitleGroup
  inSameMasterGroup?: MasterGroupEntry[]
  series?: SeriesLite | null
  affiliatedArtists: AffiliatedArtistHierarchy[]
  affiliatedEntities?: AffiliatedEntityHierarchy[]
  editAffiliationBtns?: boolean
}>()

const applyTag = async (tag: TitleGroupTagLite) => {
  applyTagToTitleGroup({ tag_id: tag.id, title_group_id: props.title_group.id }).then(() => {
    emit('tagApplied', tag.name)
  })
}

const removeSeries = () => {
  if (!props.series) return
  removeTitleGroupFromSeries({ series_id: props.series.id, title_group_id: props.title_group.id }).then(() => {
    emit('seriesRemoved')
  })
}

const removeTag = async (tag_name: string) => {
  removeTagFromTitleGroup({ tag_name, title_group_id: props.title_group.id }).then(() => {
    emit('tagRemoved', tag_name)
  })
}
</script>
<style scoped>
#title-group-sidebar {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  width: 100%;
  .content-container {
    width: 100%;
    margin-top: 20px;
  }
}
.p-galleria {
  border: none;
  width: 100%;
  .p-image {
    display: block;
    width: 100%;
  }
}
.affiliated-artists {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  max-height: 50em;
  overflow-y: scroll;
  .affiliated-artist {
    margin: 0px 4px;
    margin-bottom: 15px;
  }
}
.links {
  a {
    margin: 0px 5px;
  }
}
.external-links {
  display: flex;
  justify-content: center;
  align-items: center;
}
.series,
.tags {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
  i {
    width: 0.6em;
    cursor: pointer;
  }
}
</style>
<style>
#title-group-sidebar .p-galleria-content img {
  /* height: 20em !important; */
  width: 100% !important;
  border-radius: 7px;
}
</style>
