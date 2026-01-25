<template>
  <div id="shop-view">
    <Tabs :value="currentTab" @update:value="tabChanged">
      <TabList>
        <Tab value="shop">{{ t('shop.shop') }}</Tab>
        <Tab value="history">{{ t('shop.purchase_history') }}</Tab>
      </TabList>
      <TabPanels>
        <TabPanel value="shop" v-if="currentTab === 'shop'">
          <div class="shop-items" v-if="pricing">
            <ShopItemRow
              :title="t('shop.buy_freeleech_tokens')"
              :basePrice="pricing.freeleech_token_base_price"
              :discountTiers="pricing.freeleech_token_discount_tiers"
              tierKey="threshold"
              :min="1"
              :max="1000"
              :step="1"
              :sliderLabel="t('shop.quantity')"
              :userBalance="user.bonus_points"
              @buy="buyFreeleechTokensHandler"
            />
            <ShopItemRow
              :title="t('shop.buy_upload')"
              :basePrice="pricing.upload_base_price_per_gb"
              :discountTiers="pricing.upload_discount_tiers"
              tierKey="threshold_gb"
              :min="1"
              :max="1000"
              :step="1"
              :sliderLabel="t('shop.amount_gb')"
              :userBalance="user.bonus_points"
              @buy="buyUploadHandler"
            />
            <ShopPromotionItem v-if="pricing.promotion" :promotion="pricing.promotion" :userBalance="user.bonus_points" @buy="confirmPromotionPurchase" />
          </div>
          <div v-else class="loading">
            <ProgressSpinner />
          </div>
        </TabPanel>
        <TabPanel value="history" v-if="currentTab === 'history'">
          <ShopPurchaseHistory />
        </TabPanel>
      </TabPanels>
    </Tabs>
  </div>
  <Dialog v-model:visible="confirmDialogVisible" modal :header="t('shop.confirm_purchase')" closeOnEscape>
    <div class="confirm-content">
      <p>{{ confirmMessage }}</p>
    </div>
    <template #footer>
      <Button :label="t('general.cancel')" severity="secondary" size="small" @click="confirmDialogVisible = false" />
      <Button :label="t('general.confirm')" size="small" @click="confirmPurchase" :loading="purchasing" />
    </template>
  </Dialog>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useUserStore } from '@/stores/user'
import { useRouter } from 'vue-router'
import Tabs from 'primevue/tabs'
import TabList from 'primevue/tablist'
import Tab from 'primevue/tab'
import TabPanels from 'primevue/tabpanels'
import TabPanel from 'primevue/tabpanel'
import { Button, Dialog, ProgressSpinner } from 'primevue'
import ShopItemRow from '@/components/shop/ShopItemRow.vue'
import ShopPromotionItem from '@/components/shop/ShopPromotionItem.vue'
import ShopPurchaseHistory from '@/components/shop/ShopPurchaseHistory.vue'
import { getShopPricing, buyFreeleechTokens, buyUpload, buyPromotion, getMe, type ShopPricing } from '@/services/api-schema'
import { showToast } from '@/main'

const { t } = useI18n()
const user = useUserStore()
const router = useRouter()

const pricing = ref<ShopPricing | null>(null)
const currentTab = ref('shop')
const confirmDialogVisible = ref(false)
const confirmMessage = ref('')
const purchasing = ref(false)
const pendingPurchase = ref<{ type: 'freeleech' | 'upload' | 'promotion'; quantity: number } | null>(null)

const tabChanged = (tab: string | number) => {
  router.push({ query: { tab } })
  currentTab.value = tab as string
}

const buyFreeleechTokensHandler = (quantity: number) => {
  pendingPurchase.value = { type: 'freeleech', quantity }
  confirmMessage.value = t('shop.confirm_buy_freeleech', { quantity })
  confirmDialogVisible.value = true
}

const buyUploadHandler = (gb: number) => {
  pendingPurchase.value = { type: 'upload', quantity: gb }
  confirmMessage.value = t('shop.confirm_buy_upload', { amount: gb })
  confirmDialogVisible.value = true
}

const confirmPromotionPurchase = () => {
  pendingPurchase.value = { type: 'promotion', quantity: 0 }
  confirmMessage.value = t('shop.confirm_buy_promotion', { className: pricing.value?.promotion?.next_class_name })
  confirmDialogVisible.value = true
}

const confirmPurchase = () => {
  if (!pendingPurchase.value) return
  purchasing.value = true

  const refreshUser = () => {
    getMe().then((data) => {
      user.setUser(data.user)
      getShopPricing().then((data) => {
        pricing.value = data
      })
    })
  }

  if (pendingPurchase.value.type === 'freeleech') {
    buyFreeleechTokens({ quantity: pendingPurchase.value.quantity })
      .then(() => {
        showToast('', t('shop.purchase_successful'), 'success', 3000)
        confirmDialogVisible.value = false
        refreshUser()
      })
      .catch(() => {
        showToast('', t('shop.insufficient_balance'), 'error', 3000)
      })
      .finally(() => {
        purchasing.value = false
      })
  } else if (pendingPurchase.value.type === 'upload') {
    buyUpload({ bytes: pendingPurchase.value.quantity * 1024 * 1024 * 1024 })
      .then(() => {
        showToast('', t('shop.purchase_successful'), 'success', 3000)
        confirmDialogVisible.value = false
        refreshUser()
      })
      .catch(() => {
        showToast('', t('shop.insufficient_balance'), 'error', 3000)
      })
      .finally(() => {
        purchasing.value = false
      })
  } else if (pendingPurchase.value.type === 'promotion') {
    buyPromotion()
      .then(() => {
        showToast('', t('shop.purchase_successful'), 'success', 3000)
        confirmDialogVisible.value = false
        refreshUser()
      })
      .catch(() => {
        showToast('', t('shop.insufficient_balance'), 'error', 3000)
      })
      .finally(() => {
        purchasing.value = false
      })
  }
}

onMounted(() => {
  if (router.currentRoute.value.query.tab) {
    currentTab.value = router.currentRoute.value.query.tab as string
  }
  getShopPricing().then((data) => {
    pricing.value = data
  })
})
</script>

<style scoped>
.shop-items {
  display: flex;
  flex-direction: column;
  gap: 15px;
  margin-top: 15px;
}

.loading {
  display: flex;
  justify-content: center;
  padding: 40px;
}

.confirm-content {
  padding: 10px 0;
}
</style>
