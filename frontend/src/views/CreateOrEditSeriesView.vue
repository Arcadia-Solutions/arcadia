<template>
  <ContentContainer>
    <Form
      ref="formRef"
      v-slot="$form"
      :initialValues="seriesForm"
      :resolver
      @submit="onFormSubmit"
      validateOnSubmit
      :validateOnValueUpdate="false"
      validateOnBlur
    >
      <div class="line">
        <div>
          <FloatLabel>
            <InputText size="small" v-model="seriesForm.name" name="name" />
            <label for="name">{{ t('general.name') }}</label>
          </FloatLabel>
          <Message v-if="$form.name?.invalid" severity="error" size="small" variant="simple">
            {{ $form.name.error?.message }}
          </Message>
        </div>
      </div>
      <div>
        <FloatLabel>
          <Textarea v-model="seriesForm.description" name="description" class="description" autoResize rows="5" />
          <label for="description">{{ t('general.description') }}</label>
        </FloatLabel>
        <Message v-if="$form.description?.invalid" severity="error" size="small" variant="simple">
          {{ $form.description.error?.message }}
        </Message>
      </div>
      <div class="covers input-list">
        <label>{{ t('general.cover', 2) }}</label>
        <div v-for="(link, index) in seriesForm.covers" :key="index">
          <InputText size="small" v-model="seriesForm.covers[index]" :name="`covers[${index}]`" />
          <Button v-if="index == 0" @click="addCover" icon="pi pi-plus" size="small" />
          <Button v-if="index != 0 || seriesForm.covers.length > 1" @click="removeCover(index)" icon="pi pi-minus" size="small" />
        </div>
      </div>
      <div class="banners input-list">
        <label>Banners</label>
        <div v-for="(link, index) in seriesForm.banners" :key="index">
          <InputText size="small" v-model="seriesForm.banners[index]" :name="`banners[${index}]`" />
          <Button v-if="index == 0" @click="addBanner" icon="pi pi-plus" size="small" />
          <Button v-if="index != 0 || seriesForm.banners.length > 1" @click="removeBanner(index)" icon="pi pi-minus" size="small" />
        </div>
      </div>
      <div class="flex justify-content-center">
        <Button :label="editMode ? t('general.edit') : t('series.new_series')" icon="pi pi-check" size="small" class="validate-button" type="submit" :loading="sendingSeries" />
      </div>
    </Form>
  </ContentContainer>
</template>
<script setup lang="ts">
import { onMounted, ref, computed } from 'vue'
import FloatLabel from 'primevue/floatlabel'
import InputText from 'primevue/inputtext'
import Textarea from 'primevue/textarea'
import Button from 'primevue/button'
import Message from 'primevue/message'
import { Form, type FormResolverOptions, type FormSubmitEvent } from '@primevue/forms'
import { useI18n } from 'vue-i18n'
import { createSeries, type Series, type UserCreatedSeries } from '@/services/api/seriesService'
import type { VNodeRef } from 'vue'
import { useRouter } from 'vue-router'
import ContentContainer from '@/components/ContentContainer.vue'

interface Props {
  initialSeriesForm?: Series | null
}
const { initialSeriesForm = null } = defineProps<Props>()

const { t } = useI18n()
const router = useRouter()

const formRef = ref<VNodeRef | null>(null)
const editMode = computed(() => initialSeriesForm !== null)
const sendingSeries = ref(false)

const emit = defineEmits<{
  done: [series: Series]
}>()

const seriesForm = ref<UserCreatedSeries>({
  name: '',
  description: '',
  covers: [''],
  banners: [''],
  tags: [],
})

const resolver = ({ values }: FormResolverOptions) => {
  const errors: Partial<Record<keyof UserCreatedSeries, { message: string }[]>> = {}

  if (values.name.length < 1) {
    errors.name = [{ message: t('error.write_more_than_x_chars', [0]) }]
  }
  if (values.description.length < 10) {
    errors.description = [{ message: t('error.write_more_than_x_chars', [10]) }]
  }

  return {
    errors,
  }
}

const onFormSubmit = async ({ valid }: FormSubmitEvent) => {
  if (valid) {
    const cleanedForm: UserCreatedSeries = {
      ...seriesForm.value,
      covers: seriesForm.value.covers.filter((cover) => cover.trim() !== ''),
      banners: seriesForm.value.banners.filter((banner) => banner.trim() !== ''),
    }
    
    if (editMode.value) {
      // TODO: implement update when API is available
      // await updateSeries(cleanedForm)
    } else {
      sendingSeries.value = true
      try {
        const createdSeries = await createSeries(cleanedForm)
        emit('done', createdSeries)
        router.push(`/series/${createdSeries.id}`)
      } finally {
        sendingSeries.value = false
      }
    }
  }
}

const addCover = () => {
  seriesForm.value.covers.push('')
}
const removeCover = (index: number) => {
  seriesForm.value.covers.splice(index, 1)
}
const addBanner = () => {
  seriesForm.value.banners.push('')
}
const removeBanner = (index: number) => {
  seriesForm.value.banners.splice(index, 1)
}

onMounted(() => {
  if (initialSeriesForm !== null) {
    seriesForm.value = {
      name: initialSeriesForm.name,
      description: initialSeriesForm.description || '',
      covers: (initialSeriesForm.covers && initialSeriesForm.covers.length > 0) ? initialSeriesForm.covers : [''],
      banners: (initialSeriesForm.banners && initialSeriesForm.banners.length > 0) ? initialSeriesForm.banners : [''],
      tags: [],
    }
  }
})
</script>
<style scoped>
.description {
  width: 100%;
  height: 10em;
}

.validate-button {
  margin-top: 20px;
}

.input-list {
  margin-top: 15px;
}

.input-list .p-component {
  margin-right: 5px;
  margin-bottom: 5px;
}

.input-list input {
  width: 400px;
}

.line {
  display: flex;
  gap: 15px;
  margin-bottom: 15px;
}
</style>

