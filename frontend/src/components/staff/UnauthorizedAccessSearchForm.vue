<template>
  <ContentContainer>
    <div id="unauthorized-access-search-inputs">
      <div class="line">
        <FloatLabel>
          <InputNumber size="small" v-model="searchForm.user_id" name="user_id" />
          <label for="user_id">{{ t('user.user') }} ID</label>
        </FloatLabel>
        <FloatLabel>
          <Dropdown
            v-model="searchForm.permission"
            :options="permissionOptions"
            optionLabel="label"
            optionValue="value"
            size="small"
            input-id="permissionDropdown"
            showClear
            style="min-width: 10em"
          />
          <label for="permissionDropdown">{{ t('unauthorized_access.permission') }}</label>
        </FloatLabel>
      </div>
      <div class="line">
        <FloatLabel>
          <DatePicker size="small" v-model="fromDateModel" name="from_date" dateFormat="yy-mm-dd" />
          <label for="from_date">{{ t('unauthorized_access.from_date') }}</label>
        </FloatLabel>
        <FloatLabel>
          <DatePicker size="small" v-model="toDateModel" name="to_date" dateFormat="yy-mm-dd" />
          <label for="to_date">{{ t('unauthorized_access.to_date') }}</label>
        </FloatLabel>
      </div>
      <div class="line">
        <FloatLabel>
          <Dropdown
            v-model="searchForm.sort_by_column"
            :options="sortByOptions"
            optionLabel="label"
            optionValue="value"
            size="small"
            input-id="sortByDropdown"
            style="min-width: 10em"
          />
          <label for="sortByDropdown">{{ t('general.sort_by') }}</label>
        </FloatLabel>
        <FloatLabel>
          <Dropdown
            v-model="searchForm.sort_by_direction"
            :options="orderOptions"
            optionLabel="label"
            optionValue="value"
            size="small"
            input-id="orderDropdown"
            style="min-width: 10em"
          />
          <label for="orderDropdown">{{ t('general.order_by') }}</label>
        </FloatLabel>
      </div>
      <div class="flex justify-content-center" style="margin-top: 15px">
        <Button :loading :label="t('general.search')" @click="search" />
      </div>
    </div>
  </ContentContainer>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import ContentContainer from '@/components/ContentContainer.vue'
import { InputNumber, FloatLabel, Button, Dropdown, DatePicker } from 'primevue'
import { UnauthorizedAccessSortByColumn, UserPermission, type SearchUnauthorizedAccessLogsRequest } from '@/services/api-schema'

const { t } = useI18n()
const router = useRouter()

const props = defineProps<{
  loading: boolean
  initialForm: SearchUnauthorizedAccessLogsRequest
}>()

const permissionOptions = ref(
  Object.entries(UserPermission).map(([, value]) => ({
    label: value,
    value: value,
  })),
)

const sortByOptions = ref([
  { label: t('general.created_at'), value: UnauthorizedAccessSortByColumn.CreatedAt },
  { label: t('unauthorized_access.permission'), value: UnauthorizedAccessSortByColumn.MissingPermission },
])

const orderOptions = [
  { label: t('general.ascending'), value: 'asc' },
  { label: t('general.descending'), value: 'desc' },
]

const fromDateModel = ref<Date>(new Date(Date.now() - 7 * 24 * 60 * 60 * 1000))
const toDateModel = ref<Date>(new Date())

const initFromDate = new Date(fromDateModel.value)
initFromDate.setHours(0, 0, 0, 0)
const initToDate = new Date(toDateModel.value)
initToDate.setHours(23, 59, 59, 999)

const searchForm = ref<SearchUnauthorizedAccessLogsRequest>({
  page: 1,
  page_size: 20,
  user_id: undefined,
  permission: undefined,
  from_date: initFromDate.toISOString(),
  to_date: initToDate.toISOString(),
  sort_by_column: UnauthorizedAccessSortByColumn.CreatedAt,
  sort_by_direction: 'desc',
})

const changePage = (page: number) => {
  searchForm.value.page = page
  search()
}

const search = () => {
  const fromDate = new Date(fromDateModel.value)
  fromDate.setHours(0, 0, 0, 0)
  const toDate = new Date(toDateModel.value)
  toDate.setHours(23, 59, 59, 999)

  searchForm.value.from_date = fromDate.toISOString()
  searchForm.value.to_date = toDate.toISOString()

  router.push({
    query: {
      tab: 'unauthorizedAccess',
      ...Object.fromEntries(Object.entries(searchForm.value).filter(([, v]) => v !== null && v !== undefined)),
    },
  })
}

defineExpose({
  searchForm,
  changePage,
})

onMounted(() => {
  searchForm.value = { ...props.initialForm }
  if (props.initialForm.from_date) {
    fromDateModel.value = new Date(props.initialForm.from_date)
  }
  if (props.initialForm.to_date) {
    toDateModel.value = new Date(props.initialForm.to_date)
  }
})

watch(
  () => searchForm.value,
  (newVal, oldVal) => {
    if (newVal.page === oldVal.page) {
      searchForm.value.page = 1
    }
  },
  { deep: true },
)
</script>

<style scoped>
.line {
  display: flex;
  align-items: end;
  gap: 15px;
  margin-bottom: 15px;
}
</style>
