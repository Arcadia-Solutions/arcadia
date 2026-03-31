<template>
  <div class="reorder-dialog">
    <DataTable :value="items" @row-reorder="onRowReorder">
      <Column rowReorder style="width: 2rem" />
      <Column field="name" :header="t('general.name')" />
    </DataTable>
    <Button :label="t('general.save')" size="small" :loading="saving" @click="save" />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { showToast } from '@/main'
import { reorderForumCategories, reorderForumSubCategories } from '@/services/api-schema'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import Button from 'primevue/button'

const { t } = useI18n()

const props = defineProps<{
  mode: 'categories' | 'sub-categories'
  categoryId?: number
  initialItems: Array<{ id: number; name: string }>
}>()

const emit = defineEmits<{
  reordered: []
}>()

const items = ref([...props.initialItems])
const saving = ref(false)

const onRowReorder = (event: { value: Array<{ id: number; name: string }> }) => {
  items.value = event.value
}

const save = () => {
  saving.value = true
  const promise =
    props.mode === 'categories'
      ? reorderForumCategories({
          categories: items.value.map((item, index) => ({ id: item.id, sort_order: index })),
        })
      : reorderForumSubCategories({
          forum_category_id: props.categoryId!,
          sub_categories: items.value.map((item, index) => ({ id: item.id, sort_order: index })),
        })
  promise
    .then(() => {
      showToast('', t('forum.reorder_success'), 'success', 2000)
      emit('reordered')
    })
    .finally(() => {
      saving.value = false
    })
}
</script>

<style scoped>
.reorder-dialog {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20px;
}
</style>
