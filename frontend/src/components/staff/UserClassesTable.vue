<template>
  <div class="user-classes-table">
    <div class="actions wrapper-center" style="color: white; margin-bottom: 20px">
      <i class="pi pi-plus cursor-pointer" v-tooltip.top="t('user_class.create_user_class')" @click="openDialog()" />
    </div>
    <div style="max-width: 100%; overflow: hidden">
      <DataTable :value="tableRows" :loading="loading" scrollable scrollHeight="80vh" size="small" class="flipped-table" tableStyle="table-layout: fixed">
        <Column field="attribute" :header="t('user_class.user_class', 1)" frozen style="width: 15em" bodyStyle="font-weight: bold" />
        <Column v-for="userClass in userClasses" :key="userClass.name" style="width: 17em" bodyStyle="text-align:right">
          <template #header>
            <div class="class-header" style="text-align: right; width: 100%">
              {{ userClass.name }}
              <i
                v-tooltip.top="t('general.edit')"
                class="action pi pi-pen-to-square"
                @click="openDialog(userClass)"
                v-if="useUserStore().permissions.includes('edit_user_class')"
              />
            </div>
          </template>
          <template #body="slotProps">
            <span v-if="slotProps.data.type === 'boolean'">
              <i v-if="slotProps.data[userClass.name]" class="pi pi-check" style="color: green" />
              <i v-else class="pi pi-times" style="color: red" />
            </span>
            <span v-else-if="slotProps.data.type === 'bytes'">
              {{ bytesToReadable(slotProps.data[userClass.name]) }}
            </span>
            <span v-else-if="slotProps.data.type === 'ratio'">
              {{ slotProps.data[userClass.name].toFixed(2) }}
            </span>
            <div v-else-if="slotProps.data.type === 'array'" class="permissions-list">
              <div v-for="permission in slotProps.data[userClass.name]" :key="permission">
                {{ t(`user_permissions.${permission}`) }}
              </div>
            </div>
            <span v-else>
              {{ slotProps.data[userClass.name] }}
            </span>
          </template>
        </Column>
      </DataTable>
    </div>
    <Dialog closeOnEscape modal :header="isEditMode() ? t('user_class.edit_user_class') : t('user_class.create_user_class')" v-model:visible="dialogVisible">
      <CreateOrEditUserClassDialog :initialUserClass="userClassBeingEdited" @done="onUserClassSaved" />
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { Column, DataTable, Dialog } from 'primevue'
import { getAllUserClasses, type UserClass } from '@/services/api-schema'
import { bytesToReadable } from '@/services/helpers'
import CreateOrEditUserClassDialog from '@/components/user/CreateOrEditUserClassDialog.vue'
import { useUserStore } from '@/stores/user'

const { t } = useI18n()

const userClasses = ref<UserClass[]>([])
const loading = ref(true)
const dialogVisible = ref(false)
const userClassBeingEdited = ref<UserClass>()

interface TableRow {
  attribute: string
  type: 'boolean' | 'number' | 'bytes' | 'ratio' | 'text' | 'array'
  [key: string]: string | number | boolean | string[]
}

const tableRows = ref<TableRow[]>([])
const isEditMode = () => !!userClassBeingEdited.value

const openDialog = (userClass?: UserClass) => {
  userClassBeingEdited.value = userClass
  dialogVisible.value = true
}

const onUserClassSaved = (savedClass: UserClass) => {
  if (userClassBeingEdited.value) {
    const index = userClasses.value.findIndex((uc) => uc.name === userClassBeingEdited.value!.name)
    if (index !== -1) userClasses.value[index] = savedClass
  } else {
    userClasses.value.push(savedClass)
  }
  buildTableRows()
  dialogVisible.value = false
}

const buildTableRows = () => {
  const attributes: Array<{ key: keyof UserClass; label: string; type: TableRow['type'] }> = [
    { key: 'automatic_promotion', label: t('user_class.automatic_promotion'), type: 'boolean' },
    { key: 'automatic_demotion', label: t('user_class.automatic_demotion'), type: 'boolean' },
    { key: 'promotion_allowed_while_warned', label: t('user_class.promotion_allowed_while_warned'), type: 'boolean' },
    { key: 'required_uploaded', label: t('user_class.required_uploaded'), type: 'bytes' },
    { key: 'required_downloaded', label: t('user_class.required_downloaded'), type: 'bytes' },
    { key: 'required_ratio', label: t('user_class.required_ratio'), type: 'ratio' },
    { key: 'required_account_age_in_days', label: t('user_class.required_account_age_in_days'), type: 'number' },
    { key: 'required_forum_posts', label: t('user_class.required_forum_posts'), type: 'number' },
    { key: 'required_forum_posts_in_unique_threads', label: t('user_class.required_forum_posts_in_unique_threads'), type: 'number' },
    { key: 'required_torrent_uploads', label: t('user_class.required_torrent_uploads'), type: 'number' },
    { key: 'required_torrent_uploads_in_unique_title_groups', label: t('user_class.required_torrent_uploads_in_unique_title_groups'), type: 'number' },
    { key: 'required_torrent_snatched', label: t('user_class.required_torrent_snatched'), type: 'number' },
    { key: 'required_seeding_size', label: t('user_class.required_seeding_size'), type: 'bytes' },
    { key: 'required_title_group_comments', label: t('user_class.required_title_group_comments'), type: 'number' },
    { key: 'new_permissions', label: t('user_class.new_permissions'), type: 'array' },
  ]

  tableRows.value = attributes.map((attr) => ({
    attribute: attr.label,
    type: attr.type,
    ...Object.fromEntries(userClasses.value.map((uc) => [uc.name, uc[attr.key]])),
  })) as TableRow[]
}

onMounted(() => {
  getAllUserClasses().then((data) => {
    userClasses.value = data
    buildTableRows()
    loading.value = false
  })
})
</script>

<style scoped>
.user-classes-table {
  margin-top: 20px;
}

.action {
  cursor: pointer;
  margin-left: 8px;
}

.flipped-table :deep(.p-datatable-table) {
  min-width: 100%;
}

.flipped-table :deep(.p-datatable-thead > tr > th) {
  white-space: nowrap;
}

.permissions-list {
  display: flex;
  flex-direction: column;
  text-align: right;
}

.permissions-list div {
  font-size: 0.875rem;
  line-height: 1.3;
}

.flipped-table :deep(.p-datatable-tbody > tr > td) {
  text-align: center;
}

.flipped-table :deep(.p-datatable-tbody > tr > td:first-child) {
  text-align: left;
  font-weight: 500;
}
</style>
