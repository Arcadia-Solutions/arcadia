<template>
  <ContentContainer>
    <Form @submit="search">
      <FloatLabel>
        <InputText v-model="searchForm.name" name="name" size="small" />
        <label for="name">{{ t('general.name') }}</label>
      </FloatLabel>
      <div class="wrapper-center" style="margin-top: 15px">
        <Button :label="t('general.search')" type="submit" :loading />
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

const searchForm = ref<SearchArtistsRequest>({
  name: '',
  page: 1,
  page_size: 50,
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
