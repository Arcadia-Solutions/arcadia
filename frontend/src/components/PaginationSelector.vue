<template>
  <div class="pagination">
    <!-- &lt; is the char "<" that avoids having lint errors -->
    <span v-if="currentPage > 1" @click="emit('goToPage', 1)" class="bold" :class="{ 'text-gray-400': currentPage === 1 }" style="cursor: pointer">
      &lt;&lt; {{ t('general.first') }}
    </span>
    <span
      v-if="currentPage > 1"
      @click="emit('goToPage', currentPage - 1)"
      class="bold"
      :class="{ 'text-gray-400': currentPage === 1 }"
      style="cursor: pointer"
    >
      &lt; {{ t('general.prev') }}
    </span>

    <span v-for="(range, i) in pageRanges" :key="range.page">
      <span class="separator" v-if="currentPage > 1 || i > 0">|</span>
      <span @click="emit('goToPage', range.page)" class="bold" :class="{ 'text-gray-400': currentPage === range.page }" style="cursor: pointer">
        {{ range.label }}
      </span>
    </span>

    <span
      v-if="currentPage < totalPages"
      @click="emit('goToPage', currentPage + 1)"
      class="bold"
      :class="{ 'text-gray-400': currentPage === totalPages }"
      style="cursor: pointer"
    >
      <span class="separator">|</span>
      {{ t('general.next') }} &gt;
    </span>
    <span
      v-if="currentPage < totalPages"
      @click="emit('goToPage', totalPages)"
      class="bold"
      :class="{ 'text-gray-400': currentPage === totalPages }"
      style="cursor: pointer"
    >
      {{ t('general.last') }} &gt;&gt;
    </span>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

defineProps<{
  totalItems: number
  pageSize: number
  currentPage: number
  totalPages: number
  pageRanges: { page: number; label: string }[]
}>()

const emit = defineEmits<{
  goToPage: [number]
}>()
</script>
<style scoped>
.separator {
  margin: 0 5px;
}
</style>
