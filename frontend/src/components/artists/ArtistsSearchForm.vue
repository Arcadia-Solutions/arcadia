<template>
  <ContentContainer v-if="searchForm">
    <Form @submit="search">
      <FloatLabel>
        <InputText v-model="searchForm.name" name="name" size="small" />
        <label for="name">{{ t('general.name') }}</label>
      </FloatLabel>
      <div class="wrapper-center" style="margin-top: 15px">
        <Button :label="t('general.search')" type="submit" :loading size="small" />
      </div>
    </Form>
  </ContentContainer>
</template>

<script setup lang="ts">
import ContentContainer from '../ContentContainer.vue'
import { InputText, Button, FloatLabel } from 'primevue'
import { Form } from '@primevue/forms'
import { onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import type { SearchArtistsRequest } from '@/services/api-schema'

const { t } = useI18n()
const router = useRouter()

const props = defineProps<{
  loading: boolean
  initialForm: SearchArtistsRequest
}>()

const searchForm = ref<SearchArtistsRequest | null>(null)

const changePage = (page: number) => {
  if (!searchForm.value) return
  searchForm.value.page = page
  search()
}

const search = () => {
  if (!searchForm.value) return
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
  changePage,
})

onMounted(() => {
  searchForm.value = { ...props.initialForm }
})

watch(
  () => props.initialForm,
  (newVal) => {
    searchForm.value = { ...newVal }
  },
)
</script>
