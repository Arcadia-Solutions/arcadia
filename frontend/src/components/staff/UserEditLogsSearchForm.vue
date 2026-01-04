<template>
  <ContentContainer>
    <div id="user-edit-logs-search-inputs">
      <div class="line">
        <FloatLabel>
          <InputNumber size="small" v-model="searchForm.user_id" name="user_id" />
          <label for="user_id">{{ t('user.user') }} ID</label>
        </FloatLabel>
        <FloatLabel>
          <InputText size="small" v-model="searchForm.item_type" name="item_type" />
          <label for="item_type">{{ t('user_edit_log.item_type') }}</label>
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
            :options="getOrderByDirectionOptions(t)"
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
import { InputNumber, InputText, FloatLabel, Button, Dropdown } from 'primevue'
import { UserEditChangeLogSortByColumn, type SearchUserEditChangeLogsRequest } from '@/services/api-schema'
import { getOrderByDirectionOptions } from '@/services/helpers'

const { t } = useI18n()
const router = useRouter()

const props = defineProps<{
  loading: boolean
  initialForm: SearchUserEditChangeLogsRequest
}>()

const sortByOptions = ref([{ label: t('user_edit_log.edited_at'), value: UserEditChangeLogSortByColumn.EditedAt }])

const searchForm = ref<SearchUserEditChangeLogsRequest>({
  page: 1,
  page_size: 20,
  user_id: undefined,
  item_type: undefined,
  sort_by_column: UserEditChangeLogSortByColumn.EditedAt,
  sort_by_direction: 'desc',
})

const changePage = (page: number) => {
  searchForm.value.page = page
  search()
}

const search = () => {
  router.push({
    query: {
      tab: 'userEditLogs',
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
