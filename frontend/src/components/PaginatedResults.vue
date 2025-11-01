<template>
  <PaginationSelector
    v-if="totalItems > pageSize"
    :pageSize="pageSize"
    :totalItems="totalItems"
    :currentPage="currentPage"
    :totalPages="totalPages"
    :pageRanges="pageRanges"
    @goToPage="goToPage($event)"
  />
  <slot />
  <PaginationSelector
    v-if="totalItems > pageSize"
    :pageSize="pageSize"
    :totalItems="totalItems"
    :currentPage="currentPage"
    :totalPages="totalPages"
    :pageRanges="pageRanges"
    @goToPage="goToPage($event)"
  />
</template>

<script setup lang="ts">
import { computed, defineProps, ref } from 'vue'
import PaginationSelector from './PaginationSelector.vue'

const props = defineProps<{
  totalItems: number
  pageSize: number
}>()

const currentPage = ref(1)

type Pagination = {
  page: number
  pageSize: number
}

const emit = defineEmits<{
  changePage: [Pagination]
}>()

const totalPages = computed(() => Math.ceil(props.totalItems / props.pageSize))

const pageRanges = computed(() => {
  const total = totalPages.value
  const maxVisible = 15
  let startPage = Math.max(1, currentPage.value - Math.floor(maxVisible / 2))
  let endPage = startPage + maxVisible - 1

  if (endPage > total) {
    endPage = total
    startPage = Math.max(1, endPage - maxVisible + 1)
  }

  return Array.from({ length: endPage - startPage + 1 }, (_, i) => {
    const page = startPage + i
    const start = (page - 1) * props.pageSize + 1
    const end = Math.min(page * props.pageSize, props.totalItems)
    return { page, label: `${start}-${end}` }
  })
})

const goToPage = (page: number) => {
  if (page < 1 || page > totalPages.value) return
  currentPage.value = page
  emit('changePage', { page, pageSize: props.pageSize })
}
</script>
<style scoped>
.pagination {
  text-align: center;
  margin: 10px 0;
  .p-button {
    margin: 0 10px;
  }
}
</style>
