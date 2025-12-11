<template>
  <ContentContainer>
    <Form @submit="fetchArtists">
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
import { searchArtists, type SearchArtistsRequest, type ArtistsSearchResponse } from '@/services/api-schema'

const { t } = useI18n()

const form = ref<SearchArtistsRequest>({
  name: '',
  page: 1,
  page_size: 50,
})
const loading = ref(false)

onMounted(async () => {
  await fetchArtists()
})

const fetchArtists = async () => {
  loading.value = true
  const response = await searchArtists(form.value).finally(() => (loading.value = false))
  emit('gotResults', response)
}

const emit = defineEmits<{
  gotResults: [ArtistsSearchResponse]
}>()
</script>
