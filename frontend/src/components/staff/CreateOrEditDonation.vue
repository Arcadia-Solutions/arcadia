<template>
  <Form
    v-if="componentReady"
    class="donation-dialog"
    v-slot="$form"
    :initialValues="donation"
    :resolver="resolver"
    @submit="save"
    validateOnSubmit
    :validateOnValueUpdate="false"
    validateOnBlur
  >
    <FloatLabel>
      <InputNumber v-model="donation.amount" name="amount" :min="0" :minFractionDigits="2" :maxFractionDigits="2" />
      <label>{{ t('donation.amount') }}</label>
    </FloatLabel>

    <div>
      <FloatLabel>
        <UserSearchBar
          v-model="donorUsername"
          name="donated_by_id"
          placeholder=""
          :initialValue="donorUsername"
          :clearInputOnSelect="false"
          :clickableUserLink="false"
          @userSelected="onDonorSelected"
        />
        <label>{{ t('donation.donated_by') }}</label>
      </FloatLabel>
      <Message v-if="$form.donated_by_id?.invalid" severity="error" size="small" variant="simple">
        {{ $form.donated_by_id.error?.message }}
      </Message>
    </div>

    <FloatLabel>
      <DatePicker v-model="donatedAtDate" name="donated_at" showTime hourFormat="24" />
      <label>{{ t('donation.donated_at') }}</label>
    </FloatLabel>

    <FloatLabel>
      <Textarea v-model="donation.note" name="note" rows="5" />
      <label>{{ t('donation.note') }}</label>
    </FloatLabel>

    <div class="wrapper-center" style="margin-top: 30px">
      <Button :label="t('general.confirm')" size="small" :loading="loading" type="submit" />
    </div>
  </Form>
</template>

<script setup lang="ts">
import { FloatLabel, InputNumber, Textarea, Button, DatePicker, Message } from 'primevue'
import { Form, type FormResolverOptions, type FormSubmitEvent } from '@primevue/forms'
import { ref, onMounted, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { createDonation, editDonation, type DonationSearchResult, type UserLite, type UserCreatedDonation, type Donation } from '@/services/api-schema'
import { showToast } from '@/main'
import UserSearchBar from '@/components/user/UserSearchBar.vue'

const { t } = useI18n()

const props = defineProps<{
  initialDonation?: DonationSearchResult
}>()

const emit = defineEmits<{
  created: [Donation]
  edited: [Donation]
}>()

const donation = ref<UserCreatedDonation>({
  amount: 0,
  donated_by_id: 0,
  donated_at: null,
  note: null,
})

const componentReady = ref(false)
const donorUsername = ref('')
const donatedAtDate = ref<Date | null>(null)
const loading = ref(false)
const isEditMode = computed(() => !!props.initialDonation)

const onDonorSelected = (user: UserLite) => {
  donation.value.donated_by_id = user.id
}

const resolver = ({ values }: FormResolverOptions) => {
  const errors: Partial<Record<keyof UserCreatedDonation, { message: string }[]>> = {}

  if (!values.donated_by_id || values.donated_by_id === 0) {
    errors.donated_by_id = [{ message: t('donation.donor_required') }]
  }

  return { errors }
}

const save = async ({ valid }: FormSubmitEvent) => {
  if (!valid) return

  loading.value = true

  if (isEditMode.value && props.initialDonation) {
    const donatedAt = donatedAtDate.value ? donatedAtDate.value.toISOString() : new Date().toISOString()
    editDonation({
      id: props.initialDonation.id,
      amount: donation.value.amount,
      donated_by_id: donation.value.donated_by_id,
      donated_at: donatedAt,
      note: donation.value.note,
    })
      .then((data) => {
        showToast('Success', t('donation.donation_edited_success'), 'success', 2000)
        emit('edited', data)
      })
      .finally(() => {
        loading.value = false
      })
  } else {
    createDonation(donation.value)
      .then((data) => {
        showToast('Success', t('donation.donation_created_success'), 'success', 2000)
        emit('created', data)
      })
      .finally(() => {
        loading.value = false
      })
  }
}

onMounted(() => {
  if (props.initialDonation) {
    donation.value = {
      amount: props.initialDonation.amount,
      donated_by_id: props.initialDonation.donated_by_id,
      donated_at: props.initialDonation.donated_at,
      note: props.initialDonation.note,
    }
    donorUsername.value = props.initialDonation.donated_by.username
    donatedAtDate.value = new Date(props.initialDonation.donated_at)
  }
  componentReady.value = true
})
</script>

<style scoped>
.donation-dialog {
  display: flex;
  flex-direction: column;
  gap: 20px;
}
</style>
