<template>
  <FloatLabel class="external-db-input">
    <IconField>
      <InputText size="small" :name="`input-${database}`" v-model="externalDBId" />
      <label :for="`input-${database}`">{{ inputPlaceholder }}</label>
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
import type { ExternalDBData } from '@/services/api-schema'
import { getExternalDatabaseData } from '@/services/api/externalDatabasesService'
import { FloatLabel, IconField, InputIcon, InputText } from 'primevue'
import { ref } from 'vue'

const emit = defineEmits<{
  dataFound: [ExternalDBData]
}>()
const props = defineProps<{
  inputPlaceholder: string
  database: string
}>()

const externalDBId = ref('')
const loading = ref(false)

const getExternalDBData = (item_id: string | number) => {
  loading.value = true
  getExternalDatabaseData(item_id, props.database)
    .then((data) => {
      // data.title_group.original_release_date = new Date(data.title_group.original_release_date)
      emit('dataFound', data)
    })
    .finally(() => {
      loading.value = false
    })
}
</script>
