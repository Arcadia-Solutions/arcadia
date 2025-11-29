<template>
  <DataTable :value="staffPMs" :rowClass="(pm) => (isPMRead(pm) ? '' : 'bg-unread')">
    <Column :header="t('conversation.subject')">
      <template #body="slotProps">
        <RouterLink :to="`/staff-pm/${slotProps.data.id}`" @click="isPMRead(slotProps.data) ? null : (notificationsStore.unread_staff_pms_amount -= 1)">
          {{ slotProps.data.subject }}
        </RouterLink>
      </template>
    </Column>
    <Column :header="t('conversation.last_message')">
      <template #body="slotProps">
        {{ timeAgo(slotProps.data.last_message.created_at) }}
        {{ t('general.by') }}
        <RouterLink :to="`/user/${slotProps.data.last_message.created_by.id}`">
          {{ slotProps.data.last_message.created_by.username }}
        </RouterLink>
      </template>
    </Column>
    <Column :header="t('general.started')">
      <template #body="slotProps">
        {{ timeAgo(slotProps.data.created_at) }}
      </template>
    </Column>
    <Column :header="t('general.created_by')">
      <template #body="slotProps">
        <RouterLink :to="`/user/${slotProps.data.created_by.id}`">
          {{ slotProps.data.created_by.username }}
        </RouterLink>
      </template>
    </Column>
    <Column :header="t('staff_pm.resolved')">
      <template #body="slotProps">
        <i v-if="slotProps.data.resolved" class="pi pi-check" />
        <i v-else class="pi pi-times" />
      </template>
    </Column>
  </DataTable>
</template>

<script setup lang="ts">
import { listStaffPms, type StaffPmOverview } from '@/services/api/staffPmService'
import { timeAgo } from '@/services/helpers'
import { useNotificationsStore } from '@/stores/notifications'
import { useUserStore } from '@/stores/user'
import { Column, DataTable } from 'primevue'
import { ref } from 'vue'
import { onMounted } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const notificationsStore = useNotificationsStore()

const loading = ref(true)

const staffPMs = ref<StaffPmOverview[]>([])

const isPMRead = (p: StaffPmOverview) => {
  const userId = useUserStore().id
  return p.last_message.created_by.id === userId || p.created_by.id !== userId
}

onMounted(() => {
  listStaffPms()
    .then((data) => (staffPMs.value = data))
    .finally(() => (loading.value = false))
})
</script>

<style scoped></style>
