<template>
  <div class="artist-sidebar">
    <ImagePreview class="artist-pictures" :imageLink="artist.pictures[0] ?? '/default_artist_picture.svg'" />
    <ContentContainer v-if="artist.description" :container-title="t('general.description')">
      <div class="description">
        <BBCodeRenderer :content="artist.description" />
      </div>
    </ContentContainer>
  </div>
</template>
<script setup lang="ts">
import ContentContainer from '@/components/ContentContainer.vue'
import BBCodeRenderer from '@/components/community/BBCodeRenderer.vue'
import { useI18n } from 'vue-i18n'
import ImagePreview from '../ImagePreview.vue'
import type { Artist } from '@/services/api-schema'

const { t } = useI18n()

defineProps<{
  artist: Artist
}>()
</script>
<style scoped>
.artist-sidebar {
  display: flex;
  flex-direction: column;
  align-items: center;
}
.content-container {
  margin-top: 10px;
  width: 100%;
}
.description {
  max-height: 50vh;
  overflow-y: scroll;
}
</style>
<style>
.artist-sidebar .artist-pictures {
  img {
    width: 100%;
    border-radius: 7px;
  }
}
</style>
