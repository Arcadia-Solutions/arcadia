<template>
  <span v-if="sortedBy !== 'edition'">
    <span v-for="(item, itemIndex) in [editionGroup.source, editionGroup.name].filter((property) => property !== null)" :key="itemIndex">
      <span class="slash" v-if="itemIndex > 0"> / </span>
      <span>{{ item }}</span>
    </span>
    <span style="margin: 0.5em">|</span>
  </span>

  <span class="bold" v-if="torrent.extras.length > 0">
    {{ t('torrent.extras.extras') }}
    (<span v-for="(item, itemIndex) in torrent.extras" :key="itemIndex">
      <span v-if="itemIndex > 0">, </span>
      <span class="bold">{{ t(`torrent.extras.${item}`) }}</span> </span
    >)
    <span class="slash"> / </span>
  </span>

  <template v-for="(part, key, partIndex) in computedSlug" :key="key">
    <template v-if="part.length > 0">
      <span class="slash" v-if="partIndex > 0"> / </span>
      <span v-for="(item, itemIndex) in part" :key="`${key}-${itemIndex}`">
        <span class="slash" v-if="itemIndex > 0"> / </span>
        <span :class="{ bold: key === 'features', warning: key === 'warnings' }">{{ item }}</span>
      </span>
    </template>
  </template>

  <span v-if="torrent.peer_status">
    <span class="slash"> / </span>
    <span
      :style="{
        color:
          torrent.peer_status === 'seeding'
            ? 'green'
            : torrent.peer_status === 'leeching'
              ? 'orange'
              : torrent.peer_status === 'grabbed'
                ? 'blue'
                : torrent.peer_status === 'snatched'
                  ? 'red'
                  : '',
      }"
    >
      {{ t(`torrent.${torrent.peer_status}`) }}
    </span>
  </span>
</template>

<script lang="ts" setup>
import { useI18n } from 'vue-i18n'
import { computed } from 'vue'
import type { ContentType, EditionGroupInfoLite, TorrentHierarchyLite } from '@/services/api-schema'

const { t } = useI18n()

const props = defineProps<{
  torrent: TorrentHierarchyLite
  editionGroup: EditionGroupInfoLite
  contentType: ContentType
  sortedBy: string
}>()

const computedSlug = computed(() => {
  const firstPart: string[] = []
  const features: string[] = []
  const releaseGroup: string[] = []
  const warnings: string[] = []

  if (props.torrent.video_resolution && props.sortedBy !== 'video_resolution') {
    if (props.torrent.video_resolution === 'Other' && 'video_resolution_other_x' in props.torrent && 'video_resolution_other_y' in props.torrent) {
      firstPart.push(`${props.torrent.video_resolution_other_x as number}x${props.torrent.video_resolution_other_y as number}`)
    } else {
      firstPart.push(props.torrent.video_resolution)
    }
  }
  // if (props.editionGroup?.source && props.sortedBy !== 'edition') {
  //   firstPart.push(props.editionGroup.source)
  // }
  if (props.torrent.video_codec) {
    firstPart.push(props.torrent.video_codec)
  }
  // if (props.editionGroup?.name && props.sortedBy !== 'edition') {
  //   firstPart.push(props.editionGroup.name)
  // }
  if (
    props.torrent.container &&
    (props.contentType !== 'music' || props.torrent.extras.length > 0) &&
    props.editionGroup.additional_information['format'] !== 'audiobook'
  ) {
    firstPart.push(props.torrent.container)
  }
  if (props.torrent.audio_codec && props.sortedBy !== 'audio_codec') {
    firstPart.push(props.torrent.audio_codec)
  }
  if (props.torrent.audio_channels) {
    firstPart.push(props.torrent.audio_channels)
  }
  if (props.torrent.audio_bitrate_sampling) {
    firstPart.push(props.torrent.audio_bitrate_sampling)
  }
  if (props.torrent.languages.length === 1 && props.torrent.languages[0] !== 'English') {
    firstPart.push(props.torrent.languages[0])
  }
  if (props.torrent.languages.length > 1) {
    firstPart.push(t('torrent.multi_language'))
  }

  if (props.torrent.features) {
    props.torrent.features.forEach((feature) => features.push(feature))
  }

  if (props.torrent.release_group) {
    releaseGroup.push(props.torrent.release_group)
  }

  if (props.torrent.reports.length > 0) {
    warnings.push(t('general.reported'))
  }
  if ('trumpable' in props.torrent && props.torrent.trumpable) {
    warnings.push(t('torrent.trumpable'))
  }

  // The order of these properties in the returned object will dictate their order in the rendered slug.
  return { firstPart, features, releaseGroup, warnings }
})
</script>

<style scoped>
.slash {
  font-weight: 300;
}
.seeding {
  color: green;
}
.leeching {
  color: yellow;
}
.snatched {
  color: white;
}
.warning {
  color: orange;
}
</style>
