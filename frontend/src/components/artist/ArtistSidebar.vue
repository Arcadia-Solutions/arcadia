<template>
  <div class="artist-sidebar">
    <ImagePreview class="artist-pictures" :imageLink="artist.pictures[0] ?? '/default_artist_picture.svg'" />
    <ContentContainer v-if="artist.description" :container-title="t('general.description')">
      <div class="description">
        <BBCodeRenderer :content="artist.description" />
      </div>
    </ContentContainer>
    <ContentContainer v-if="artist.aliases.length > 0" :container-title="t('general.alias', 2)">
      <div class="aliases">{{ artist.aliases.join(', ') }}</div>
    </ContentContainer>
    <ContentContainer :container-title="t('community.statistics')">
      <div>{{ t('artist.title_groups') }}: {{ artist.title_groups_amount }}</div>
      <div>{{ t('edition_group.edition_group', 2) }}: {{ artist.edition_groups_amount }}</div>
      <div>{{ t('statistics.torrents') }}: {{ artist.torrents_amount }}</div>
      <div>{{ t('torrent.seeders') }}: {{ artist.seeders_amount }}</div>
      <div>{{ t('torrent.leecher', 2) }}: {{ artist.leechers_amount }}</div>
      <div>{{ t('user.snatches') }}: {{ artist.snatches_amount }}</div>
    </ContentContainer>
    <ContentContainer v-if="sortedTags.length > 0" :container-title="t('general.tags')">
      <div class="tags">
        <span v-for="[name, count] in sortedTags" :key="name" class="tag">
          <RouterLink :to="{ path: '/torrents', query: { title_group_tags: name } }">{{ name }}</RouterLink>
          ({{ count }})
        </span>
      </div>
    </ContentContainer>
  </div>
</template>
<script setup lang="ts">
import { computed } from 'vue'
import ContentContainer from '@/components/ContentContainer.vue'
import BBCodeRenderer from '@/components/community/BBCodeRenderer.vue'
import { useI18n } from 'vue-i18n'
import ImagePreview from '../ImagePreview.vue'
import type { Artist } from '@/services/api-schema'

const { t } = useI18n()

const props = defineProps<{
  artist: Artist
  tags: { [key: string]: number }
}>()

const sortedTags = computed(() => Object.entries(props.tags).sort((a, b) => b[1] - a[1]))
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
.tags {
  display: flex;
  flex-wrap: wrap;
  gap: 5px;
}
.tag {
  background: var(--p-content-hover-background);
  padding: 2px 6px;
  border-radius: 4px;
}
</style>
<style>
.artist-sidebar .artist-pictures {
  display: block;
  width: 100%;
  img {
    width: 100%;
    border-radius: 7px;
  }
}
</style>
