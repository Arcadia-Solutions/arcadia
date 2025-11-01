<template>
  <div class="pagination">
    <!-- &lt; is the char "<" that avoids having lint errors -->
    <span @click="emit('goToPage', 1)" class="bold" :class="{ 'text-gray-400': currentPage === 1 }" style="cursor: pointer"
      >&lt;&lt; {{ t('general.first') }}</span
    >
    <span @click="emit('goToPage', currentPage - 1)" class="bold" :class="{ 'text-gray-400': currentPage === 1 }" style="cursor: pointer">
      &lt; {{ t('general.prev') }}
    </span>

    <span v-for="range in pageRanges" :key="range.page">
      |
      <span @click="emit('goToPage', range.page)" class="bold" :class="{ 'text-gray-400': currentPage === range.page }" style="cursor: pointer; margin: 0 3px">
        {{ range.label }}
      </span>
    </span>

    |
    <span @click="emit('goToPage', currentPage + 1)" class="bold" :class="{ 'text-gray-400': currentPage === totalPages }" style="cursor: pointer">
      {{ t('general.next') }} &gt;
    </span>
    <span @click="emit('goToPage', totalPages)" class="bold" :class="{ 'text-gray-400': currentPage === totalPages }" style="cursor: pointer">
      {{ t('general.last') }} &gt;&gt;
    </span>
  </div>
</template>

<script setup lang="ts">
import { defineProps } from 'vue'
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
<style scoped></style>
