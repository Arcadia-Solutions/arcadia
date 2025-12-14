<template>
  <div class="edit-permissions">
    <FloatLabel>
      <MultiSelect v-model="selectedPermissions" :options="allPermissions" optionLabel="label" optionValue="value" class="permissions-select" display="chip" />
      <label>{{ t('user.manage_permissions') }}</label>
    </FloatLabel>
    <Button :label="t('general.save')" size="small" :loading @click="savePermissions()" style="margin-top: 20px" />
  </div>
</template>

<script setup lang="ts">
import { showToast } from '@/main'
import { editUserPermissions, getUserPermissions, UserPermission, type UpdatedUserPermissions } from '@/services/api-schema'
import { FloatLabel, MultiSelect } from 'primevue'
import Button from 'primevue/button'
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps<{
  userId: number
}>()

const selectedPermissions = ref<Array<UserPermission>>([])
const loading = ref(false)

const allPermissions = computed(() =>
  Object.values(UserPermission).map((permission) => ({
    value: permission,
    label: t(`user_permissions.${permission}`),
  })),
)

const emit = defineEmits<{
  saved: []
}>()

onMounted(async () => {
  const permissions = await getUserPermissions(props.userId)
  selectedPermissions.value = permissions
})

const savePermissions = () => {
  loading.value = true
  const updatedPermissions: UpdatedUserPermissions = {
    permissions: selectedPermissions.value,
  }
  editUserPermissions({
    id: props.userId,
    UpdatedUserPermissions: updatedPermissions,
  }).then(() => {
    loading.value = false
    showToast('Success', t('user.profile_edited_success'), 'success', 4000)
    emit('saved')
  })
}
</script>

<style scoped>
.edit-permissions {
  width: 80vw;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}
.permissions-select {
  min-width: 20em;
}
.permissions-select :deep(.p-multiselect-label) {
  display: flex;
  flex-wrap: wrap;
}
</style>
