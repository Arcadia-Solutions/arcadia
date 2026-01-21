<template>
  <div class="set-custom-title">
    <FloatLabel>
      <InputText class="custom-title-input" name="custom_title" v-model="customTitle" />
      <label for="custom_title">{{ t('user.custom_title') }}</label>
    </FloatLabel>
    <Button :label="t('general.save')" size="small" style="margin-top: 10px" :loading @click="saveCustomTitle()" />
  </div>
</template>

<script setup lang="ts">
import { showToast } from '@/main'
import { setUserCustomTitle } from '@/services/api-schema'
import { FloatLabel, InputText } from 'primevue'
import Button from 'primevue/button'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const props = defineProps<{
  userId: number
  currentCustomTitle?: string | null
}>()

const { t } = useI18n()

const customTitle = ref(props.currentCustomTitle ?? '')
const loading = ref(false)

const emit = defineEmits<{
  saved: [customTitle: string | null]
}>()

const saveCustomTitle = () => {
  loading.value = true
  const value = customTitle.value || null
  setUserCustomTitle({
    id: props.userId,
    UpdateUserCustomTitle: {
      custom_title: value,
    },
  })
    .then(() => {
      showToast('Success', t('user.custom_title_saved_success'), 'success', 4000)
      emit('saved', value)
    })
    .finally(() => {
      loading.value = false
    })
}
</script>

<style scoped>
.set-custom-title {
  width: 55em;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}
.custom-title-input {
  width: 45em;
}
</style>
