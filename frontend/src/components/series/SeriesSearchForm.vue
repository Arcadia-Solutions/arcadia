<template>
  <ContentContainer>
    <Form @submit="fetchSeries">
      <FloatLabel>
        <InputText v-model="form.name" name="name" size="small" />
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
import { onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { searchSeries, type SearchSeriesQuery, type SeriesSearchResponse } from '@/services/api-schema'

const { t } = useI18n()

const form = ref<SearchSeriesQuery>({
  name: '',
  page: 1,
  page_size: 50,
  tags: [],
})
const loading = ref(false)
const seriesSearchResponse = ref<SeriesSearchResponse>()

onMounted(async () => {
  await fetchSeries()
})

const fetchSeries = async () => {
  loading.value = true
  seriesSearchResponse.value = await searchSeries(form.value).finally(() => (loading.value = false))
  emit('gotResults', seriesSearchResponse.value)
}

const emit = defineEmits<{
  gotResults: [SeriesSearchResponse]
}>()
</script>
<style scoped></style>
