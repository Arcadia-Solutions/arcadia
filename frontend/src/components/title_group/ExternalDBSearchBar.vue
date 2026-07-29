<template>
  <FloatLabel class="external-db-input">
    <IconField>
      <InputText size="small" :name="`input-${sourceId}`" v-model="externalDBId" />
      <label :for="`input-${sourceId}`">{{ inputPlaceholder }}</label>
      <InputIcon
        :class="{
          pi: true,
          'pi-search': !loading,
          'pi-hourglass': loading,
          'cursor-pointer': true,
        }"
        @click="getExternalDBData(externalDBId)"
      />
    </IconField>
  </FloatLabel>
</template>
<script lang="ts" setup>
import { getExternalSourceData, type ExternalDBData } from '@/services/api-schema'
import { FloatLabel, IconField, InputIcon, InputText } from 'primevue'
import { ref } from 'vue'

const emit = defineEmits<{
  dataFound: [ExternalDBData]
}>()
const props = defineProps<{
  inputPlaceholder: string
  sourceId: string
}>()

const externalDBId = ref('')
const loading = ref(false)

const getExternalDBData = (item_id: string | number) => {
  loading.value = true

  return getExternalSourceData({ source_id: props.sourceId, url: item_id.toString() })
    .then((data) => {
      emit('dataFound', data)
      return data
    })
    .finally(() => {
      loading.value = false
    })
}
</script>
