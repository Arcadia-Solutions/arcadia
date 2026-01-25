<template>
  <div class="discount-tiers-editor">
    <DataTable :value="tiers" size="small">
      <Column :header="thresholdLabel">
        <template #body="slotProps">
          <InputNumber v-model="slotProps.data[thresholdKey]" :min="1" size="small" @update:modelValue="emitUpdate" />
        </template>
      </Column>
      <Column :header="t('arcadia_settings.discount_percent')">
        <template #body="slotProps">
          <InputNumber v-model="slotProps.data.discount_percent" :min="0" :max="100" size="small" suffix="%" @update:modelValue="emitUpdate" />
        </template>
      </Column>
      <Column>
        <template #body="slotProps">
          <Button icon="pi pi-trash" severity="danger" size="small" text @click="removeTier(slotProps.index)" />
        </template>
      </Column>
    </DataTable>
    <Button
      :label="t('arcadia_settings.add_tier')"
      icon="pi pi-plus"
      size="small"
      severity="secondary"
      @click="addTier"
      style="margin-top: 10px; margin-bottom: 20px"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import { InputNumber, Button } from 'primevue'
import type { FreeleechTokenDiscountTier, UploadDiscountTier } from '@/services/api-schema'

const { t } = useI18n()

type DiscountTier = FreeleechTokenDiscountTier | UploadDiscountTier

const props = defineProps<{
  modelValue: DiscountTier[]
  tierType: 'freeleech' | 'upload'
}>()

const emit = defineEmits<{
  'update:modelValue': [value: DiscountTier[]]
}>()

const tiers = ref<DiscountTier[]>([...props.modelValue])

const thresholdKey = computed(() => (props.tierType === 'freeleech' ? 'threshold' : 'threshold_gb'))
const thresholdLabel = computed(() => (props.tierType === 'freeleech' ? t('arcadia_settings.threshold_tokens') : t('arcadia_settings.threshold_gb')))

watch(
  () => props.modelValue,
  (newVal) => {
    tiers.value = [...newVal]
  },
  { deep: true },
)

const emitUpdate = () => {
  emit('update:modelValue', [...tiers.value])
}

const addTier = () => {
  if (props.tierType === 'freeleech') {
    tiers.value.push({ threshold: 10, discount_percent: 5 } as FreeleechTokenDiscountTier)
  } else {
    tiers.value.push({ threshold_gb: 10, discount_percent: 5 } as UploadDiscountTier)
  }
  emitUpdate()
}

const removeTier = (index: number) => {
  tiers.value.splice(index, 1)
  emitUpdate()
}
</script>

<style scoped>
.discount-tiers-editor {
  margin-top: 10px;
}
</style>
