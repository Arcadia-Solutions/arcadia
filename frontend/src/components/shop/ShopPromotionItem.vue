<template>
  <ContentContainer class="promotion-item">
    <div class="item-header">
      <span class="item-title">{{ t('shop.buy_promotion') }}</span>
    </div>
    <div class="promotion-details">
      <div class="promotion-info">
        <span>{{ t('shop.promotion_to') }}: {{ promotion.next_class_name }}</span>
        <span>{{ t('shop.promotion_cost') }}: {{ promotion.cost }} BP</span>
      </div>
      <div class="promotion-status">
        <Tag v-if="promotion.requirements_met" :value="t('shop.requirements_met')" severity="success" size="small" />
        <Tag v-else :value="t('shop.requirements_not_met')" severity="danger" size="small" />
      </div>
      <Button :label="t('shop.buy')" size="small" :disabled="!promotion.requirements_met || userBalance < promotion.cost" @click="emit('buy')" />
    </div>
  </ContentContainer>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { Button, Tag } from 'primevue'
import ContentContainer from '@/components/ContentContainer.vue'
import type { PromotionPricing } from '@/services/api-schema'

const { t } = useI18n()

defineProps<{
  promotion: PromotionPricing
  userBalance: number
}>()

const emit = defineEmits<{
  buy: []
}>()
</script>

<style scoped>
.item-header {
  margin-bottom: 10px;
}

.item-title {
  font-weight: bold;
  font-size: 1.1em;
}

.promotion-details {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 15px;
  flex-wrap: wrap;
}

.promotion-info {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.promotion-status {
  display: flex;
  align-items: center;
}
</style>
