<template>
  <div class="bonus-points-log">
    <div class="filters">
      <FloatLabel>
        <DatePicker size="small" v-model="fromDateModel" inputId="from_date" dateFormat="yy-mm-dd" />
        <label for="from_date">{{ t('shop.from_date') }}</label>
      </FloatLabel>
      <FloatLabel>
        <DatePicker size="small" v-model="toDateModel" inputId="to_date" dateFormat="yy-mm-dd" />
        <label for="to_date">{{ t('shop.to_date') }}</label>
      </FloatLabel>
      <FloatLabel>
        <MultiSelect
          v-model="selectedActions"
          :options="actionOptions"
          optionLabel="label"
          optionValue="value"
          size="small"
          inputId="actions_filter"
          display="chip"
          filter
          style="min-width: 16em; max-width: 30em"
        />
        <label for="actions_filter">{{ t('shop.actions_filter') }}</label>
      </FloatLabel>
      <Button :label="t('general.search')" size="small" @click="applyFilters" :loading="loading" />
    </div>
    <PaginatedResults :totalItems :pageSize :initialPage="page" :totalPages @changePage="onPageChange">
      <DataTable :value="logs" :loading="loading" size="small" lazy :sortField :sortOrder @sort="onSort">
        <template #empty>
          <div class="empty-message">{{ t('shop.no_bonus_points_log_entries', { alias: publicArcadiaSettings.bonus_points_alias }) }}</div>
        </template>
        <Column field="created_at" :header="t('general.time')" sortable>
          <template #body="slotProps">
            {{ timeAgo(slotProps.data.created_at) }}
          </template>
        </Column>
        <Column field="action" :header="t('shop.action')" sortable>
          <template #body="slotProps">
            {{ t(`shop.action_${slotProps.data.action}`) }}
          </template>
        </Column>
        <Column field="amount" :header="t('shop.amount')" sortable>
          <template #body="slotProps">
            <span :class="slotProps.data.amount >= 0 ? 'positive' : 'negative'">
              {{ slotProps.data.amount >= 0 ? '+' : '' }}{{ formatBp(slotProps.data.amount) }}
            </span>
          </template>
        </Column>
      </DataTable>
    </PaginatedResults>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import { Button, DatePicker, FloatLabel, MultiSelect } from 'primevue'
import PaginatedResults from '@/components/PaginatedResults.vue'
import { searchBonusPointsLogs, BonusPointsLogAction, BonusPointsLogOrderByColumn, OrderByDirection, type BonusPointsLog } from '@/services/api-schema'
import { timeAgo, formatBp as formatBpShared } from '@/services/helpers'
import { usePublicArcadiaSettingsStore } from '@/stores/publicArcadiaSettings'
import type { DataTableSortEvent } from 'primevue/datatable'

const { t } = useI18n()
const publicArcadiaSettings = usePublicArcadiaSettingsStore()

const logs = ref<BonusPointsLog[]>([])
const loading = ref(true)

const sortField = ref<BonusPointsLogOrderByColumn>(BonusPointsLogOrderByColumn.CreatedAt)
const sortOrder = ref<1 | -1>(-1)
const page = ref(1)
const pageSize = ref(50)
const totalItems = ref(0)
const totalPages = computed(() => Math.ceil(totalItems.value / pageSize.value))

const fromDateModel = ref<Date>(new Date('2025-12-01T00:00:00'))
const toDateModel = ref<Date>(new Date())
const selectedActions = ref<BonusPointsLogAction[]>([])

const actionOptions = Object.values(BonusPointsLogAction).map((value) => ({
  label: t(`shop.action_${value}`),
  value,
}))

const formatBp = (value: number) => formatBpShared(value, publicArcadiaSettings.bonus_points_decimal_places, true)

const fetchLogs = () => {
  loading.value = true
  const fromDate = new Date(fromDateModel.value)
  fromDate.setHours(0, 0, 0, 0)
  const toDate = new Date(toDateModel.value)
  toDate.setHours(23, 59, 59, 999)

  searchBonusPointsLogs({
    page: page.value,
    page_size: pageSize.value,
    order_by_column: sortField.value,
    order_by_direction: sortOrder.value === 1 ? OrderByDirection.Asc : OrderByDirection.Desc,
    from_date: fromDate.toISOString(),
    to_date: toDate.toISOString(),
    actions: selectedActions.value.join(','),
  })
    .then((data) => {
      logs.value = data.results
      totalItems.value = data.total_items
      pageSize.value = data.page_size
    })
    .finally(() => {
      loading.value = false
    })
}

const applyFilters = () => {
  page.value = 1
  fetchLogs()
}

const onSort = (event: DataTableSortEvent) => {
  sortField.value = event.sortField as BonusPointsLogOrderByColumn
  sortOrder.value = (event.sortOrder as 1 | -1) ?? -1
  page.value = 1
  fetchLogs()
}

const onPageChange = (pagination: { page: number }) => {
  page.value = pagination.page
  fetchLogs()
}

onMounted(() => {
  fetchLogs()
})
</script>

<style scoped>
.bonus-points-log {
  margin-top: 15px;
}

.filters {
  display: flex;
  align-items: flex-end;
  gap: 15px;
  margin-bottom: 20px;
  flex-wrap: wrap;
}

.empty-message {
  text-align: center;
  padding: 20px;
  color: var(--text-color-secondary);
}

.positive {
  color: var(--p-green-500);
}

.negative {
  color: var(--p-red-500);
}
</style>
