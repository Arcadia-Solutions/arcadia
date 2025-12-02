<template>
  <ContentContainer>
    <div id="title-group-header">
      <div class="left">
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
        <div class="textual-information">
          <div class="title">
            {{ title_group.name }}
            <span class="title-metadata"> ({{ title_group.original_release_date.substring(0, 4) }}) </span>
          </div>
          <div class="information-line">
            <span class="item-title">{{ t('general.tags') }}:</span>
            <div class="item" v-for="(tag, index) in title_group.tags" :key="tag">{{ tag }}<span v-if="index !== title_group.tags.length - 1">,</span></div>
          </div>
          <div class="information-line" v-if="title_group.name_aliases.length != 0 && title_group.name_aliases[0] != ''">
            <span class="item-title">{{ t('general.alias', 2) }}:</span>
            <div v-for="(alias, index) in title_group.name_aliases" :key="alias">
              {{ alias }}<span v-if="index !== title_group.name_aliases.length - 1">,</span>
            </div>
          </div>
          <div class="information-line">
            <span class="item-title">{{ t('general.original_language') }}:</span>
            {{ title_group.original_language }}
          </div>
          <div class="information-line">
            <span class="item-title">{{ t('general.country') }}:</span>
            {{ title_group.country_from }}
          </div>
          <div v-if="series.id" class="information-line series">
            <span class="item-title">{{ t('series.series') }}:</span>
            <RouterLink :to="`/series/${series.id}`">
              {{ series.name }}
            </RouterLink>
          </div>
          <div class="information-line link-logos">
            <div v-for="link in title_group.external_links" :key="link">
              <ExternalLink :link="link" class="link-logo" />
            </div>
          </div>
        </div>
      </div>
      <div class="right">
        <div class="affiliated_artists">
          <AffiliatedArtist v-for="affiliated_artist in affiliatedArtists" :key="affiliated_artist.artist.id" :affiliated_artist="affiliated_artist" />
        </div>
      </div>
    </div>
  </ContentContainer>
</template>
<script setup lang="ts">
import ContentContainer from '@/components/ContentContainer.vue'

import { Galleria } from 'primevue'
import AffiliatedArtist from '@/components/artist/AffiliatedArtist.vue'
import ExternalLink from '@/components/ExternalLink.vue'
import type { TitleGroup } from '@/services/api/titleGroupService'
import { useI18n } from 'vue-i18n'
import type { SeriesLite } from '@/services/api/seriesService'
import type { AffiliatedArtistHierarchy } from '@/services/api/artistService'
import ImagePreview from '../ImagePreview.vue'

const { t } = useI18n()

defineProps<{
  title_group: TitleGroup
  series: SeriesLite
  affiliatedArtists: AffiliatedArtistHierarchy[]
}>()
</script>
<style scoped>
#title-group-header {
  margin-bottom: 20px;
  display: flex;
  justify-content: space-between;
}
.left {
  display: flex;
}
.p-galleria {
  height: 22em;
  border: none;
  margin-right: 20px;
}
.item-title {
  font-weight: bold;
  margin-right: 7px;
}
.information-line {
  display: flex;
  margin-top: 8px;
}
.information-line .item {
  margin-right: 5px;
}
.series a {
  padding: 0;
}
.link-logos {
  margin-top: 25px;
  display: flex;
  align-items: center;
}
.link-logo {
  margin-right: 7px;
}
.affiliated-artists {
  margin-right: 0;
  margin-left: auto;
}
</style>
<style>
#title-group-header .left .p-galleria-content img {
  height: 20em !important;
  border-radius: 7px;
}
#title-group-header .p-galleria-indicator-list {
  padding: 7px !important;
}
</style>
