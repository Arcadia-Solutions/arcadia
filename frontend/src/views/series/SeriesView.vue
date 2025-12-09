<template>
  <div id="series-view" v-if="series" class="with-sidebar">
    <div class="main">
      <SeriesSlimHeader class="slim-header" :series />
      <ContentContainer v-if="title_group_preview_mode == 'cover-only'">
        <div class="title-groups">
          <TitleGroupPreviewCoverOnly v-for="title_group in title_groups" :key="title_group.id" :titleGroup="title_group" />
        </div>
      </ContentContainer>
      <div v-if="title_group_preview_mode == 'table'">
        <TitleGroupPreviewTable v-for="title_group in title_groups" :key="title_group.id" :title_group="title_group" class="preview-table" />
      </div>
    </div>
    <SeriesSidebar :series />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useRoute } from 'vue-router'
import SeriesSlimHeader from '@/components/series/SeriesSlimHeader.vue'
import ContentContainer from '@/components/ContentContainer.vue'
import TitleGroupPreviewCoverOnly from '@/components/title_group/TitleGroupPreviewCoverOnly.vue'
import TitleGroupPreviewTable from '@/components/title_group/TitleGroupPreviewTable.vue'
import SeriesSidebar from '@/components/series/SeriesSidebar.vue'
import { getSeries, type Series, type TitleGroupHierarchyLite } from '@/services/api-schema'

const route = useRoute()

const series = ref<Series | null>(null)
const title_groups = ref<TitleGroupHierarchyLite[]>([])
const title_group_preview_mode = ref<'table' | 'cover-only'>('table') // TODO: make a select button to switch from cover-only to table
const siteName = import.meta.env.VITE_SITE_NAME

const fetchSeries = async () => {
  const id = Number(route.params.id)
  // TODO: either toast an error message + redirect or show an error component
  if (!Number.isNaN(id)) {
    const data = await getSeries(id)
    series.value = data.series
    title_groups.value = data.title_groups
  }

  document.title = `${series.value?.name} - ${siteName}`
}

onMounted(async () => {
  fetchSeries()
})

watch(() => route.params.id, fetchSeries, { immediate: true })
</script>

<style scoped>
.title-groups {
  display: flex;
  align-items: center;
  justify-content: space-around;
  flex-wrap: wrap;
}

.preview-table {
  margin-bottom: 15px;
}
</style>
<style>
#series-view .series-covers img {
  border-radius: 7px;
}
</style>
