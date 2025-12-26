<template>
  <div class="lock-unlock-class">
    <p>{{ classLocked ? t('user.confirm_unlock_user_class') : t('user.confirm_lock_user_class') }}</p>
    <Button :label="t('general.confirm')" size="small" :loading @click="toggleLock()" />
  </div>
</template>

<script setup lang="ts">
import { showToast } from '@/main'
import { lockUnlockUserClass, type UserClassLockStatus } from '@/services/api-schema'
import Button from 'primevue/button'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps<{
  userId: number
  classLocked: boolean
}>()

const loading = ref(false)

const emit = defineEmits<{
  saved: [classLocked: boolean]
}>()

const toggleLock = () => {
  loading.value = true
  const lockStatus: UserClassLockStatus = {
    class_locked: !props.classLocked,
  }
  lockUnlockUserClass({
    id: props.userId,
    UserClassLockStatus: lockStatus,
  }).then(() => {
    loading.value = false
    showToast('Success', t('user.class_lock_changed_success'), 'success', 4000)
    emit('saved', !props.classLocked)
  })
}
</script>

<style scoped>
.lock-unlock-class {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20px;
}
</style>
