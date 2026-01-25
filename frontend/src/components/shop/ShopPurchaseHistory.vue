<template>
  <div class="purchase-history">
    <DataTable :value="purchases" :loading="loading">
      <template #empty>
        <div class="empty-message">{{ t('shop.no_purchases') }}</div>
      </template>
      <Column field="purchased_at" :header="t('general.time')">
        <template #body="slotProps">
          {{ timeAgo(slotProps.data.purchased_at) }}
        </template>
      </Column>
      <Column field="item_type" :header="t('shop.item_type')">
        <template #body="slotProps">
          {{ getItemTypeLabel(slotProps.data.item_type) }}
        </template>
      </Column>
      <Column field="quantity" :header="t('shop.quantity')">
        <template #body="slotProps">
          {{ formatQuantity(slotProps.data) }}
        </template>
      </Column>
      <Column field="extra_info" :header="t('shop.details')">
        <template #body="slotProps">
          {{ slotProps.data.extra_info || '-' }}
        </template>
      </Column>
      <Column field="bonus_points_spent" :header="t('shop.bp_spent')">
        <template #body="slotProps"> {{ slotProps.data.bonus_points_spent }} BP </template>
      </Column>
    </DataTable>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import { getShopPurchaseHistory, type ShopPurchase, ShopItem } from '@/services/api-schema'
import { timeAgo, bytesToReadable } from '@/services/helpers'

const { t } = useI18n()

const purchases = ref<ShopPurchase[]>([])
const loading = ref(true)

const getItemTypeLabel = (itemType: string) => {
  switch (itemType) {
    case ShopItem.FreeleechTokens:
      return t('shop.freeleech_tokens')
    case ShopItem.Upload:
      return t('shop.upload')
    case ShopItem.Promotion:
      return t('shop.promotion')
    default:
      return itemType
  }
}

const formatQuantity = (purchase: ShopPurchase) => {
  switch (purchase.item_type) {
    case ShopItem.Upload:
      return bytesToReadable(purchase.quantity)
    case ShopItem.Promotion:
      return '-'
    default:
      return purchase.quantity
  }
}

onMounted(() => {
  getShopPurchaseHistory()
    .then((data) => {
      purchases.value = data
    })
    .finally(() => {
      loading.value = false
    })
})
</script>

<style scoped>
.purchase-history {
  margin-top: 15px;
}

.empty-message {
  text-align: center;
  padding: 20px;
  color: var(--text-color-secondary);
}
</style>
