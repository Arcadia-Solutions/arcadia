<template>
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
import type { ContentType } from '@/services/api/torrentService'
import { useI18n } from 'vue-i18n'
import { computed } from 'vue'

const { t } = useI18n()

const props = defineProps<{
  torrentRequest: TorrentRequest
  contentType: ContentType
}>()

const computedSlug = computed<string[][]>(() => {
  const firstPart: string[] = []
  const features: string[] = []
  const releaseGroup: string[] = []

  function addIfPresent(arr: string[], value: string | string[] | undefined | null, specify_any: boolean, name?: string) {
    if (value && (typeof value === 'string' || value.length > 0)) {
      arr.push(Array.isArray(value) ? value.join(', ') : value)
    } else if (specify_any && value && typeof value === 'object' && value.length === 0) {
      arr.push(t(`torrent.any_${name}`))
    }
  }

  if (props.torrentRequest.video_resolution) {
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
  addIfPresent(firstPart, props.torrentRequest.source, true, 'source')
  addIfPresent(firstPart, props.torrentRequest.video_codec, true, 'video_codec')
  // addIfPresent(firstPart, props.torrentRequest.edition_name, false, 'edition')
  if (props.contentType !== 'music') {
    addIfPresent(firstPart, props.torrentRequest.container, false, 'container')
  }
  addIfPresent(firstPart, props.torrentRequest.audio_codec, true, 'audio_codec')
  addIfPresent(firstPart, props.torrentRequest.audio_channels, false, 'audio_channels')
  addIfPresent(firstPart, props.torrentRequest.audio_bitrate_sampling, true, 'audio_bitrate_sampling')

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
</script>
<style scoped>
.slash {
  font-weight: 300;
}
</style>
