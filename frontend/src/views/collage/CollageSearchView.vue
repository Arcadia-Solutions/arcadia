<template>
  <div class="wrapper-center actions">
    <RouterLink to="/new-collage">
      <i class="pi pi-plus" v-tooltip.top="t('collage.new_collage')" />
    </RouterLink>
    <i class="pi pi-user" v-tooltip.top="t('collage.collages_i_started')" />
    <i class="pi pi-bookmark" v-tooltip.top="t('collage.bookmarked_collages')" />
  </div>
  <CollageSearchForm @gotResults="gotResults" style="margin-bottom: 15px" />
  <CollagesTable :collages showCollageType showTags />
</template>
<script setup lang="ts">
import type { PaginatedResults_CollageSearchResult, CollageSearchResult } from '@/services/api/collageService'
import { ref } from 'vue'
import CollageSearchForm from '@/components/collage/CollageSearchForm.vue'
import CollagesTable from '@/components/collage/CollagesTable.vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const collages = ref<CollageSearchResult[]>([])

const gotResults = (collageSearchResponse: PaginatedResults_CollageSearchResult) => {
  collages.value = collageSearchResponse.results
}
</script>
<style scoped>
#collage-view {
  display: flex;
}
.actions {
  i {
    margin: 10px;
    color: white;
  }
}
.main-content {
  width: 80%;
  margin-right: 10px;
}
</style>
