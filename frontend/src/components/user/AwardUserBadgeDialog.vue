<template>
  <div class="award-badge-dialog">
    <FloatLabel>
      <Select
        v-model="selectedBadgeId"
        :options="awardableBadges"
        optionLabel="name"
        optionValue="id"
        size="small"
        filter
        style="width: 100%"
        :loading="loadingBadges"
      >
        <template #option="{ option }">
          <div class="badge-option">
            <img v-if="option.image_url" :src="option.image_url" class="badge-icon" :alt="option.name" />
            <div>
              <div>{{ option.name }}</div>
              <div class="hint">{{ option.description }}</div>
            </div>
          </div>
        </template>
      </Select>
      <label>{{ t('user_badge.select_badge') }}</label>
    </FloatLabel>

    <FloatLabel>
      <Textarea v-model="note" rows="3" style="width: 100%" />
      <label>{{ t('user_badge.note') }}</label>
    </FloatLabel>

    <div class="wrapper-center">
      <Button :label="t('general.confirm')" size="small" :loading="submitting" :disabled="!selectedBadgeId" @click="submit" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { FloatLabel, Select, Textarea, Button } from 'primevue'
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { awardUserBadge, listUserBadges, type UserBadge, type UserBadgeListItem } from '@/services/api-schema'
import { showToast } from '@/main'

const { t } = useI18n()

const props = defineProps<{
  userId: number
}>()

const emit = defineEmits<{
  awarded: []
}>()

const awardableBadges = ref<UserBadge[]>([])
const selectedBadgeId = ref<number | null>(null)
const note = ref('')
const submitting = ref(false)
const loadingBadges = ref(true)

const isFullBadge = (b: UserBadgeListItem): b is UserBadge => 'name' in b

const submit = () => {
  if (!selectedBadgeId.value) return
  submitting.value = true
  awardUserBadge({
    user_id: props.userId,
    badge_id: selectedBadgeId.value,
    note: note.value || null,
  })
    .then(() => {
      showToast('', t('user_badge.badge_awarded_success'), 'success', 2000)
      emit('awarded')
    })
    .finally(() => {
      submitting.value = false
    })
}

onMounted(() => {
  listUserBadges()
    .then((data) => {
      awardableBadges.value = data.filter(isFullBadge)
    })
    .finally(() => {
      loadingBadges.value = false
    })
})
</script>

<style scoped>
.award-badge-dialog {
  width: 40vw;
  max-width: 500px;
  display: flex;
  flex-direction: column;
  gap: 20px;
}
.badge-option {
  display: flex;
  align-items: center;
  gap: 10px;
}
.badge-icon {
  width: 32px;
  height: 32px;
  object-fit: contain;
  border-radius: 4px;
}
.hint {
  font-size: 0.8rem;
  opacity: 0.7;
}
</style>
