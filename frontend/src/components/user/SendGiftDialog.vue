<template>
  <div class="send-gift">
    <FloatLabel>
      <InputNumber name="bonus_points" v-model="displayBonusPoints" :min="0" fluid />
      <label for="bonus_points">{{ publicArcadiaSettings.bonus_points_alias }}</label>
    </FloatLabel>
    <FloatLabel>
      <InputNumber name="freeleech_tokens" v-model="gift.freeleech_tokens" :min="0" fluid />
      <label for="freeleech_tokens">{{ t('user.gift.freeleech_tokens') }}</label>
    </FloatLabel>
    <FloatLabel>
      <Textarea class="message" name="message" v-model="gift.message" rows="5" />
      <label for="message">{{ t('user.gift.message') }}</label>
    </FloatLabel>
    <Button :label="t('user.gift.send')" size="small" :loading @click="sendGift()" />
  </div>
</template>

<script setup lang="ts">
import { showToast } from '@/main'
import { rawToDisplayBp, displayToRawBp } from '@/services/helpers'
import { createGift, type Gift, type UserCreatedGift } from '@/services/api-schema'
import { Textarea, FloatLabel, InputNumber } from 'primevue'
import Button from 'primevue/button'
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute } from 'vue-router'
import { usePublicArcadiaSettingsStore } from '@/stores/publicArcadiaSettings'

const { t } = useI18n()
const route = useRoute()
const publicArcadiaSettings = usePublicArcadiaSettingsStore()

const gift = ref<UserCreatedGift>({
  bonus_points: 0,
  freeleech_tokens: 0,
  message: '',
  receiver_id: parseInt(route.params.id as string),
})
const displayBonusPoints = computed({
  get: () => rawToDisplayBp(gift.value.bonus_points, publicArcadiaSettings.bonus_points_decimal_places),
  set: (value: number) => {
    gift.value.bonus_points = displayToRawBp(value, publicArcadiaSettings.bonus_points_decimal_places)
  },
})

const loading = ref(false)

const emit = defineEmits<{
  sent: [gift: Gift]
}>()

const sendGift = () => {
  loading.value = true
  createGift(gift.value)
    .then((data: Gift) => {
      showToast('Success', t('user.gift.sent_success'), 'success', 4000)
      emit('sent', data)
    })
    .finally(() => {
      loading.value = false
    })
}
</script>

<style scoped>
.send-gift {
  padding-top: 20px;
  width: 30em !important;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  gap: 20px;
}
.message {
  width: 25em;
}
</style>
