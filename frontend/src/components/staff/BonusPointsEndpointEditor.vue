<template>
  <div class="bonus-points-endpoint-editor">
    <DataTable :value="endpoints" size="small">
      <Column :header="t('arcadia_settings.bp_endpoint_method')">
        <template #body="slotProps">
          <Select v-model="slotProps.data.method" :options="Object.values(HttpMethod)" size="small" @update:modelValue="emitUpdate" />
        </template>
      </Column>
      <Column :header="t('arcadia_settings.bp_endpoint_path_prefix')">
        <template #body="slotProps">
          <InputText v-model="slotProps.data.path_prefix" size="small" @update:modelValue="emitUpdate" />
        </template>
      </Column>
      <Column :header="t('arcadia_settings.bp_endpoint_amount')">
        <template #body="slotProps">
          <InputNumber
            :modelValue="rawToDisplayBp(slotProps.data.amount, props.bpDecimalPlaces)"
            :min="0"
            :maxFractionDigits="props.bpDecimalPlaces"
            size="small"
            @update:modelValue="
              (v: number) => {
                slotProps.data.amount = displayToRawBp(v, props.bpDecimalPlaces)
                emitUpdate()
              }
            "
          />
        </template>
      </Column>
      <Column :header="t('arcadia_settings.bp_endpoint_probability')">
        <template #body="slotProps">
          <InputNumber
            v-model="slotProps.data.probability"
            :min="0"
            :max="100"
            :maxFractionDigits="2"
            :step="1"
            suffix="%"
            size="small"
            @update:modelValue="emitUpdate"
          />
        </template>
      </Column>
      <Column>
        <template #body="slotProps">
          <Button icon="pi pi-trash" severity="danger" size="small" text @click="removeEndpoint(slotProps.index)" />
        </template>
      </Column>
    </DataTable>
    <Button
      :label="t('arcadia_settings.add_bp_endpoint')"
      icon="pi pi-plus"
      size="small"
      severity="secondary"
      @click="addEndpoint"
      style="margin-top: 10px; margin-bottom: 20px"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import { InputNumber, InputText, Button, Select } from 'primevue'
import { HttpMethod, type BonusPointsEndpoint } from '@/services/api-schema'
import { rawToDisplayBp, displayToRawBp } from '@/services/helpers'

const { t } = useI18n()

const props = defineProps<{
  modelValue: BonusPointsEndpoint[]
  bpDecimalPlaces: number
}>()

const emit = defineEmits<{
  'update:modelValue': [value: BonusPointsEndpoint[]]
}>()

const endpoints = ref<BonusPointsEndpoint[]>([...props.modelValue])

watch(
  () => props.modelValue,
  (newVal) => {
    endpoints.value = [...newVal]
  },
  { deep: true },
)

const emitUpdate = () => {
  emit('update:modelValue', [...endpoints.value])
}

const addEndpoint = () => {
  endpoints.value.push({ path_prefix: '/api/', method: HttpMethod.Get, amount: 0, probability: 100 })
  emitUpdate()
}

const removeEndpoint = (index: number) => {
  endpoints.value.splice(index, 1)
  emitUpdate()
}
</script>

<style scoped>
.bonus-points-endpoint-editor {
  margin-top: 10px;
}
</style>
