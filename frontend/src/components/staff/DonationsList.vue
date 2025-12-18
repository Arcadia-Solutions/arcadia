<template>
  <div class="donations-container">
    <div class="donations-header">
      <h3>{{ t('donation.donation', 2) }}</h3>
      <Button :label="t('donation.add_donation')" icon="pi pi-plus" @click="showAddDialog = true" />
    </div>

    <div v-if="donationStats" class="donation-stats">
      <div class="stats-card">
        <span class="stats-label">{{ t('donation.current_progress') }}</span>
        <ProgressBar :value="progressPercentage" :showValue="false" />
        <span class="stats-value">{{ formatCurrency(donationStats.current_total) }} / {{ formatCurrency(donationStats.goal) }}</span>
        <span class="stats-period">{{ t(`donation.period.${donationStats.period}`) }}</span>
      </div>
    </div>

    <DataTable :value="donations?.results" :loading="loading" responsiveLayout="scroll">
      <Column :header="t('general.created_at')">
        <template #body="slotProps">
          {{ formatDate(slotProps.data.created_at) }}
        </template>
      </Column>
      <Column :header="t('donation.amount')">
        <template #body="slotProps"> {{ formatCurrency(slotProps.data.amount) }} {{ slotProps.data.currency }} </template>
      </Column>
      <Column :header="t('donation.donor_name')">
        <template #body="slotProps">
          {{ slotProps.data.donor_name || t('general.anonymous') }}
        </template>
      </Column>
      <Column :header="t('donation.note')">
        <template #body="slotProps">
          {{ slotProps.data.note || '-' }}
        </template>
      </Column>
      <Column :header="t('general.action', 2)">
        <template #body="slotProps">
          <Button icon="pi pi-pencil" text @click="editDonation(slotProps.data)" />
          <Button icon="pi pi-trash" text severity="danger" @click="confirmDelete(slotProps.data)" />
        </template>
      </Column>
    </DataTable>

    <Paginator
      v-if="donations && donations.total_items > pageSize"
      :rows="pageSize"
      :totalRecords="donations.total_items"
      :first="(page - 1) * pageSize"
      @page="onPageChange"
    />

    <Dialog v-model:visible="showAddDialog" :header="editingDonation ? t('donation.edit_donation') : t('donation.add_donation')" modal>
      <div class="donation-form">
        <div class="field">
          <label>{{ t('donation.amount') }}</label>
          <InputNumber v-model="donationForm.amount" mode="decimal" :minFractionDigits="2" :maxFractionDigits="2" />
        </div>
        <div class="field">
          <label>{{ t('donation.currency') }}</label>
          <InputText v-model="donationForm.currency" />
        </div>
        <div class="field">
          <label>{{ t('donation.donor_name') }}</label>
          <InputText v-model="donationForm.donor_name" :placeholder="t('general.anonymous')" />
        </div>
        <div class="field">
          <label>{{ t('donation.note') }}</label>
          <Textarea v-model="donationForm.note" rows="3" />
        </div>
      </div>
      <template #footer>
        <Button :label="t('general.cancel')" text @click="closeDialog" />
        <Button :label="t('general.validate')" @click="saveDonation" />
      </template>
    </Dialog>

    <Dialog v-model:visible="showSettingsDialog" :header="t('donation.settings')" modal>
      <div class="donation-form">
        <div class="field">
          <label>{{ t('donation.goal') }}</label>
          <InputNumber v-model="settingsForm.donation_goal" mode="decimal" :minFractionDigits="2" :maxFractionDigits="2" />
        </div>
        <div class="field">
          <label>{{ t('donation.period.period') }}</label>
          <Select v-model="settingsForm.donation_goal_period" :options="periodOptions" optionLabel="label" optionValue="value" />
        </div>
      </div>
      <template #footer>
        <Button :label="t('general.cancel')" text @click="showSettingsDialog = false" />
        <Button :label="t('general.validate')" @click="saveSettings" />
      </template>
    </Dialog>

    <Button :label="t('donation.settings')" icon="pi pi-cog" text class="settings-btn" @click="openSettings" />

    <Dialog v-model:visible="showDeleteDialog" :header="t('general.delete')" modal>
      <p>{{ t('donation.confirm_delete') }}</p>
      <template #footer>
        <Button :label="t('general.cancel')" text @click="showDeleteDialog = false" />
        <Button :label="t('general.delete')" severity="danger" @click="performDelete" />
      </template>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import {
  getDonations,
  createDonation,
  editDonation as editDonationApi,
  deleteDonation,
  getDonationStats,
  getDonationSettings,
  updateDonationSettings,
  type Donation,
  type PaginatedResultsDonation,
  type DonationStats,
  type UserCreatedDonation,
  type EditedDonation,
  type DonationSettings,
} from '@/services/api-schema'
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import Button from 'primevue/button'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import Dialog from 'primevue/dialog'
import InputNumber from 'primevue/inputnumber'
import InputText from 'primevue/inputtext'
import Textarea from 'primevue/textarea'
import ProgressBar from 'primevue/progressbar'
import Paginator from 'primevue/paginator'
import Select from 'primevue/select'

const { t } = useI18n()

const loading = ref(true)
const page = ref(1)
const pageSize = 20

const donations = ref<PaginatedResultsDonation | null>(null)
const donationStats = ref<DonationStats | null>(null)

const showAddDialog = ref(false)
const showSettingsDialog = ref(false)
const showDeleteDialog = ref(false)
const editingDonation = ref<Donation | null>(null)
const donationToDelete = ref<Donation | null>(null)

const donationForm = ref<UserCreatedDonation>({
  amount: 0,
  currency: 'EUR',
  donor_name: null,
  user_id: null,
  note: '',
})

const settingsForm = ref<DonationSettings>({
  donation_goal: 0,
  donation_goal_period: 'monthly',
})

const periodOptions = computed(() => [
  { label: t('donation.period.monthly'), value: 'monthly' },
  { label: t('donation.period.yearly'), value: 'yearly' },
])

const progressPercentage = computed(() => {
  if (!donationStats.value || donationStats.value.goal === 0) return 0
  return Math.min(100, (donationStats.value.current_total / donationStats.value.goal) * 100)
})

const formatCurrency = (value: number) => {
  return new Intl.NumberFormat('fr-FR', { style: 'currency', currency: 'EUR' }).format(value)
}

const formatDate = (date: string) => {
  return new Date(date).toLocaleDateString('fr-FR', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  })
}

const fetchDonations = async () => {
  loading.value = true
  try {
    donations.value = await getDonations({ page: page.value, page_size: pageSize })
  } finally {
    loading.value = false
  }
}

const fetchStats = async () => {
  donationStats.value = await getDonationStats()
}

const onPageChange = (event: { page: number }) => {
  page.value = event.page + 1
  fetchDonations()
}

const editDonation = (donation: Donation) => {
  editingDonation.value = donation
  donationForm.value = {
    amount: donation.amount,
    currency: donation.currency,
    donor_name: donation.donor_name,
    user_id: donation.user_id,
    note: donation.note,
  }
  showAddDialog.value = true
}

const closeDialog = () => {
  showAddDialog.value = false
  editingDonation.value = null
  donationForm.value = {
    amount: 0,
    currency: 'EUR',
    donor_name: null,
    user_id: null,
    note: '',
  }
}

const saveDonation = async () => {
  if (editingDonation.value) {
    const edited: EditedDonation = { ...donationForm.value }
    await editDonationApi({ id: editingDonation.value.id, EditedDonation: edited })
  } else {
    await createDonation(donationForm.value)
  }
  closeDialog()
  await fetchDonations()
  await fetchStats()
}

const confirmDelete = (donation: Donation) => {
  donationToDelete.value = donation
  showDeleteDialog.value = true
}

const performDelete = async () => {
  if (donationToDelete.value) {
    await deleteDonation(donationToDelete.value.id)
    showDeleteDialog.value = false
    donationToDelete.value = null
    await fetchDonations()
    await fetchStats()
  }
}

const openSettings = async () => {
  const settings = await getDonationSettings()
  settingsForm.value = { ...settings }
  showSettingsDialog.value = true
}

const saveSettings = async () => {
  await updateDonationSettings(settingsForm.value)
  showSettingsDialog.value = false
  await fetchStats()
}

onMounted(() => {
  fetchDonations()
  fetchStats()
})
</script>

<style scoped>
.donations-container {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.donations-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.donation-stats {
  margin-bottom: 1rem;
}

.stats-card {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  padding: 1rem;
  border-radius: 8px;
  background: var(--p-surface-800);
  border: 1px solid var(--p-surface-600);
}

.stats-label {
  font-weight: bold;
  color: var(--p-text-color);
}

.stats-value {
  font-size: 1.2rem;
  color: var(--p-primary-color);
  font-weight: 600;
}

.stats-period {
  font-size: 0.9rem;
  color: var(--p-text-muted-color);
}

.donation-form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  min-width: 300px;
}

.field {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.settings-btn {
  align-self: flex-end;
}
</style>
