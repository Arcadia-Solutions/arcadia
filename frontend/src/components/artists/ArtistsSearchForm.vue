<template>
  <ContentContainer>
    <Form @submit="search">
      <FloatLabel>
        <InputText v-model="searchForm.name" name="name" size="small" />
        <label for="name">{{ t('general.name') }}</label>
      </FloatLabel>
      <div style="display: flex; gap: 10px; margin-top: 15px">
        <FloatLabel>
          <Select v-model="searchForm.order_by_column" :options="sortByOptions" optionLabel="label" optionValue="value" size="small" />
          <label>{{ t('general.sort_by') }}</label>
        </FloatLabel>
        <FloatLabel>
          <Select v-model="searchForm.order_by_direction" :options="orderOptions" optionLabel="label" optionValue="value" size="small" />
          <label>{{ t('general.order_by') }}</label>
        </FloatLabel>
      </div>
      <div class="wrapper-center" style="margin-top: 15px">
        <Button :label="t('general.search')" type="submit" :loading />
      </div>
    </Form>
  </ContentContainer>
</template>

<script setup lang="ts">
import ContentContainer from '../ContentContainer.vue'
import { InputText, Button, FloatLabel, Select } from 'primevue'
import { Form } from '@primevue/forms'
import { onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import { ArtistSearchOrderByColumn, OrderByDirection, type SearchArtistsRequest } from '@/services/api-schema'

const { t } = useI18n()
const router = useRouter()

const props = defineProps<{
  loading: boolean
  initialForm: SearchArtistsRequest
}>()

const sortByOptions = [
  { label: t('general.name'), value: ArtistSearchOrderByColumn.Name },
  { label: t('general.created_at'), value: ArtistSearchOrderByColumn.CreatedAt },
  { label: t('artist.title_groups'), value: ArtistSearchOrderByColumn.TitleGroupsAmount },
]

const orderOptions = [
  { label: t('general.ascending'), value: OrderByDirection.Asc },
  { label: t('general.descending'), value: OrderByDirection.Desc },
]

const searchForm = ref<SearchArtistsRequest>({
  name: '',
  page: 1,
  page_size: 50,
  order_by_column: ArtistSearchOrderByColumn.Name,
  order_by_direction: OrderByDirection.Asc,
})

const changePage = (page: number) => {
  searchForm.value.page = page
  search()
}

const search = () => {
  router.push({
    query: Object.fromEntries(
      Object.entries({
        name: searchForm.value.name,
        page: searchForm.value.page,
        page_size: searchForm.value.page_size,
        order_by_column: searchForm.value.order_by_column,
        order_by_direction: searchForm.value.order_by_direction,
      }).filter(([, v]) => v !== undefined && v !== null && v !== ''),
    ),
  })
}

defineExpose({
  searchForm,
  changePage,
})

onMounted(async () => {
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
