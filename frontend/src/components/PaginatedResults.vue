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
  <div style="margin: 20px 0">
    <slot />
  </div>
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
import { onMounted } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()

const props = defineProps<{
  totalItems: number
  pageSize: number
  initialPage?: number | null
  totalPages: number
}>()

const currentPage = ref(1)

type Pagination = {
  page: number
  pageSize: number
}

const emit = defineEmits<{
  changePage: [Pagination]
}>()

const pageRanges = computed(() => {
  const total = props.totalPages
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
  if (page < 1 || page > props.totalPages) return
  currentPage.value = page
  router.push({
    // path: route.path,
    query: { page: page },
  })
  emit('changePage', { page, pageSize: props.pageSize })
}

defineExpose({
  goToPage,
})

onMounted(() => {
  if (props.initialPage) {
    currentPage.value = props.initialPage
  }
})
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
