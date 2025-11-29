<template>
  <div class="actions wrapper-center">
    <RouterLink to="/staff-pms/new">
      <i class="pi pi-plus" v-tooltip.top="t('staff_pm.new')" />
    </RouterLink>
  </div>
  <StaffPmsTable />
</template>

<script lang="ts" setup>
import StaffPmsTable from '@/components/staff_pm/StaffPmsTable.vue'
import { useI18n } from 'vue-i18n'
import { RouterLink } from 'vue-router'

const { t } = useI18n()
</script>
<style scoped>
.actions {
  margin-bottom: 15px;
  a {
    color: white;
  }
}
</style>
