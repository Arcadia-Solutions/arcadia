<template>
  <ContentContainer>
    <div id="donation-search-inputs">
      <div class="line">
        <FloatLabel>
          <InputNumber size="small" v-model="searchForm.donated_by_id" name="donated_by_id" />
          <label for="donated_by_id">{{ t('donation.donated_by') }} ID</label>
        </FloatLabel>
        <FloatLabel>
          <InputNumber size="small" v-model="searchForm.created_by_id" name="created_by_id" />
          <label for="created_by_id">{{ t('general.created_by') }} ID</label>
        </FloatLabel>
      </div>
      <div class="line">
        <FloatLabel>
          <InputNumber size="small" v-model="searchForm.min_amount" name="min_amount" :min="0" :minFractionDigits="2" :maxFractionDigits="2" />
          <label for="min_amount">{{ t('donation.min_amount') }}</label>
        </FloatLabel>
        <FloatLabel>
          <InputNumber size="small" v-model="searchForm.max_amount" name="max_amount" :min="0" :minFractionDigits="2" :maxFractionDigits="2" />
          <label for="max_amount">{{ t('donation.max_amount') }}</label>
        </FloatLabel>
      </div>
      <div class="line">
        <div class="dropdown">
          <label for="sortByDropdown">{{ t('general.sort_by') }}</label>
          <Dropdown
            v-model="searchForm.order_by_column"
            :options="sortByOptions"
            optionLabel="label"
            optionValue="value"
            size="small"
            input-id="sortByDropdown"
          />
        </div>
        <div class="dropdown">
          <label for="orderDropdown">{{ t('general.order_by') }}</label>
          <Dropdown
            v-model="searchForm.order_by_direction"
            :options="orderOptions"
            optionLabel="label"
            optionValue="value"
            size="small"
            input-id="orderDropdown"
          />
        </div>
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
import { InputNumber, FloatLabel, Button, Dropdown } from 'primevue'
import type { SearchDonationsRequest } from '@/services/api-schema'

const { t } = useI18n()
const router = useRouter()

const props = defineProps<{
  loading: boolean
  initialForm: SearchDonationsRequest
}>()

const sortByOptions = ref([
  { label: t('donation.donated_at'), value: 'donated_at' },
  { label: t('general.created_at'), value: 'created_at' },
  { label: t('donation.amount'), value: 'amount' },
])

const orderOptions = [
  { label: t('general.ascending'), value: 'asc' },
  { label: t('general.descending'), value: 'desc' },
]

const searchForm = ref<SearchDonationsRequest>({
  page: 1,
  page_size: 20,
  donated_by_id: null,
  created_by_id: null,
  min_amount: null,
  max_amount: null,
  donated_at_start: null,
  donated_at_end: null,
  order_by_column: 'donated_at',
  order_by_direction: 'desc',
})

const changePage = (page: number) => {
  searchForm.value.page = page
  search()
}

const search = () => {
  router.push({
    query: {
      tab: 'donations',
      ...Object.fromEntries(Object.entries(searchForm.value).filter(([, v]) => v !== null)),
    },
  })
}

defineExpose({
  searchForm,
  changePage,
})

onMounted(() => {
  searchForm.value = props.initialForm
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
  gap: 15px;
  margin-bottom: 15px;
}

.dropdown {
  display: flex;
  align-items: center;
  gap: 5px;
}
</style>
