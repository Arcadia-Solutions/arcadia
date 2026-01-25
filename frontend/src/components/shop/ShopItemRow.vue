<template>
  <ContentContainer class="shop-item-row">
    <div class="item-header">
      <span class="item-title">{{ title }}</span>
    </div>
    <div class="item-controls">
      <div class="slider-section">
        <div class="quantity-input">
          <label>{{ sliderLabel }}:</label>
          <InputNumber v-model="quantity" :min="min" :max="max" :step="step" size="small" style="margin-left: 4px" />
        </div>
        <Slider v-model="quantity" :min="min" :max="max" :step="step" />
      </div>
      <div class="price-section">
        <div class="price-details">
          <span class="base-price">{{ t('shop.base_price') }}: {{ basePrice }} {{ publicArcadiaSettings.bonus_points_alias }}</span>
          <span v-if="discountPercent > 0" class="discount">{{ t('shop.discount') }}: {{ discountPercent }}%</span>
          <span class="total-price">{{ t('shop.total_price') }}: {{ totalPrice }} {{ publicArcadiaSettings.bonus_points_alias }}</span>
        </div>
        <Button :label="t('shop.buy')" size="small" :disabled="userBalance < totalPrice" @click="emit('buy', quantity)" />
      </div>
    </div>
    <div class="discount-tiers" v-if="discountTiers.length > 0">
      <span class="tiers-label">{{ t('shop.discount_tiers') }}:</span>
      <div class="tiers-list">
        <Tag
          v-for="tier in discountTiers"
          :key="getTierThreshold(tier)"
          :value="`${getTierThreshold(tier)}+ = ${tier.discount_percent}%`"
          :severity="quantity >= getTierThreshold(tier) ? 'success' : 'secondary'"
          size="small"
        />
      </div>
    </div>
  </ContentContainer>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { Button, InputNumber, Slider, Tag } from 'primevue'
import type { FreeleechTokenDiscountTier, UploadDiscountTier } from '@/services/api-schema'
import ContentContainer from '../ContentContainer.vue'
import { usePublicArcadiaSettingsStore } from '@/stores/publicArcadiaSettings'

const { t } = useI18n()
const publicArcadiaSettings = usePublicArcadiaSettingsStore()

type DiscountTier = FreeleechTokenDiscountTier | UploadDiscountTier

const props = defineProps<{
  title: string
  basePrice: number
  discountTiers: DiscountTier[]
  tierKey: 'threshold' | 'threshold_gb'
  min: number
  max: number
  step: number
  sliderLabel: string
  userBalance: number
}>()

const emit = defineEmits<{
  buy: [quantity: number]
}>()

const quantity = ref(props.min)

const getTierThreshold = (tier: DiscountTier): number => {
  if ('threshold' in tier) return tier.threshold
  return tier.threshold_gb
}

const discountPercent = computed(() => {
  const sortedTiers = [...props.discountTiers].sort((a, b) => getTierThreshold(b) - getTierThreshold(a))
  for (const tier of sortedTiers) {
    if (quantity.value >= getTierThreshold(tier)) {
      return tier.discount_percent
    }
  }
  return 0
})

const totalPrice = computed(() => {
  const price = props.basePrice * quantity.value * (1 - discountPercent.value / 100)
  return Math.round(price)
})
</script>

<style scoped>
.item-header {
  margin-bottom: 10px;
}

.item-title {
  font-weight: bold;
  font-size: 1.1em;
}

.item-controls {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 20px;
  flex-wrap: wrap;
}

.slider-section {
  flex: 1;
  min-width: 200px;
}

.quantity-input {
  margin-bottom: 8px;
  margin-left: 4px;
  :deep(input) {
    width: 7em !important;
  }
}

.price-section {
  display: flex;
  align-items: center;
  gap: 15px;
}

.price-details {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 2px;
  font-size: 0.9em;
}

.discount {
  color: var(--color-primary);
}

.total-price {
  font-weight: bold;
}

.discount-tiers {
  margin-top: 15px;
  padding-top: 10px;
  border-top: 1px solid var(--color-border);
}

.tiers-label {
  font-size: 0.85em;
  margin-right: 10px;
}

.tiers-list {
  display: inline-flex;
  gap: 8px;
  flex-wrap: wrap;
  margin-top: 5px;
}
</style>
