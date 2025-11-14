<template>
  <div class="media-info-preview">
    <div class="wrapper-center" style="margin-bottom: 10px; margin-top: -10px">
      <Button :label="t('torrent.toggle_mediainfo')" @click="toggleMediaInfo" size="small" />
    </div>
    <ContentContainer>
      <MediaInfoSummary v-if="showMediaInfoPreview" :source="parsed" />
      <MediaInfoFullTable v-else :mediainfo="mediainfo" />
    </ContentContainer>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { parseMediaInfo, type MediaInfoParsed } from '@/services/fileinfo/mediainfo/parseMediaInfo'
import MediaInfoSummary from './MediaInfoSummary.vue'
import MediaInfoFullTable from './MediaInfoFullTable.vue'
import { Button } from 'primevue'
import { useI18n } from 'vue-i18n'
import ContentContainer from '../ContentContainer.vue'

const { t } = useI18n()

const props = defineProps<{
  mediainfo: string
}>()

const parsed = computed<MediaInfoParsed>(() => parseMediaInfo(props.mediainfo))
const showMediaInfoPreview = ref(true)

const toggleMediaInfo = () => {
  showMediaInfoPreview.value = !showMediaInfoPreview.value
}
</script>

<style scoped>
.media-info-preview {
  max-width: 98%;
}
.media-info-preview .header-text {
  font-weight: bold;
  font-size: 15px;
}
</style>
