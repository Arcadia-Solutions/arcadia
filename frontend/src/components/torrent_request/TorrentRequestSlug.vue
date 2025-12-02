<template>
  <!-- Edition information -->
  <span v-for="(item, itemIndex) in computedSecondSlug" :key="itemIndex">
    <span class="slash" v-if="itemIndex > 0"> / </span>
    <span>{{ item }}</span>
  </span>
  <span class="separator" v-if="computedSlug.some((part) => part.length > 0) && computedSecondSlug.length > 0"> | </span>
  <!-- Torrent information -->
  <template v-for="(part, partIndex) in computedSlug" :key="partIndex">
    <template v-if="part.length > 0">
      <span v-for="(item, itemIndex) in part" :key="itemIndex">
        <span class="slash" v-if="itemIndex > 0 || (partIndex > 0 && computedSlug[partIndex].length > 0)"> / </span>
        <span :class="{ bold: partIndex === 1 }">{{ item }}</span>
      </span>
    </template>
  </template>
</template>

<script lang="ts" setup>
import type { TorrentRequest } from '@/services/api/torrentRequestService'
import type { ContentType } from '@/services/api/titleGroupService'
import { useI18n } from 'vue-i18n'
import { computed } from 'vue'
import { isAttributeUsed } from '@/services/helpers'

const { t } = useI18n()

const props = defineProps<{
  torrentRequest: TorrentRequest
  contentType: ContentType
}>()

function addIfPresent(arr: string[], value: string | string[] | undefined | null, specify_any: boolean, name?: string, prefix?: string) {
  if (value && (typeof value === 'string' || value.length > 0)) {
    arr.push(Array.isArray(value) ? value.join(', ') : value)
  } else if (specify_any && value && typeof value === 'object' && value.length === 0) {
    arr.push(t(`${prefix}.any_${name}`))
  }
}

const computedSlug = computed<string[][]>(() => {
  const firstPart: string[] = []
  const features: string[] = []
  const releaseGroup: string[] = []

  if (isAttributeUsed('video_resolution', props?.contentType)) {
    if (
      props.torrentRequest.video_resolution.includes('Other') &&
      'video_resolution_other_x' in props.torrentRequest &&
      'video_resolution_other_y' in props.torrentRequest
    ) {
      firstPart.push(`${props.torrentRequest.video_resolution_other_x as number}x${props.torrentRequest.video_resolution_other_y as number}`)
    } else if (props.torrentRequest.video_resolution.length > 0) {
      firstPart.push(props.torrentRequest.video_resolution.join(', '))
    } else {
      firstPart.push(t('torrent.any_video_resolution'))
    }
  }

  if (isAttributeUsed('video_codec', props?.contentType)) {
    addIfPresent(firstPart, props.torrentRequest.video_codec, true, 'video_codec', 'torrent')
  }

  if (props.contentType !== 'music') {
    addIfPresent(firstPart, props.torrentRequest.container, false, 'container', 'torrent')
  }

  if (isAttributeUsed('audio_codec', props?.contentType)) {
    addIfPresent(firstPart, props.torrentRequest.audio_codec, true, 'audio_codec', 'torrent')
  }
  addIfPresent(firstPart, props.torrentRequest.audio_channels, false, 'audio_channels', 'torrent')
  if (isAttributeUsed('audio_bitrate_sampling', props?.contentType)) {
    addIfPresent(firstPart, props.torrentRequest.audio_bitrate_sampling, true, 'audio_bitrate_sampling', 'torrent')
  }

  if (props.torrentRequest.languages.length === 1 && props.torrentRequest.languages[0] !== 'English') {
    firstPart.push(props.torrentRequest.languages[0])
  }
  if (props.torrentRequest.languages.length > 1) {
    firstPart.push(t('torrent.multi_language'))
  }

  if (props.torrentRequest.features) {
    props.torrentRequest.features.forEach((feature) => features.push(feature))
  }

  if (props.torrentRequest.release_group) {
    releaseGroup.push(props.torrentRequest.release_group)
  }

  return [firstPart, features, releaseGroup]
})

const computedSecondSlug = computed<string[]>(() => {
  const slug: string[] = []

  addIfPresent(slug, props.torrentRequest.edition_name, false, undefined, 'edition_group')
  addIfPresent(slug, props.torrentRequest.source, true, 'source', 'edition_group')

  return slug
})
</script>

<style scoped>
.slash {
  font-weight: 300;
}
.separator {
  margin: 0 4px;
}
</style>
