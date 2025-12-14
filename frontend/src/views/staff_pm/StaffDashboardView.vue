<template>
  <div id="staff-dashboard">
    <Tabs value="1">
      <TabList>
        <Tab value="0">{{ t('staff.user_application.user_application', 2) }}</Tab>
        <Tab value="1">{{ t('staff_pm.staff_pm', 2) }}</Tab>
        <Tab value="2">{{ t('css_sheet.css_sheet', 2) }}</Tab>
      </TabList>
      <TabPanels>
        <TabPanel value="0" v-if="userStore.permissions.includes('get_user_application')">
          <UserApplications />
        </TabPanel>
        <TabPanel value="1" v-if="userStore.permissions.includes('read_staff_pm')">
          <StaffPmsTable />
        </TabPanel>
        <TabPanel value="2" v-if="userStore.permissions.includes('edit_css_sheet') || userStore.permissions.includes('create_css_sheet')">
          <CssSheetList showStaffActions />
        </TabPanel>
      </TabPanels>
    </Tabs>
  </div>
</template>

<script setup lang="ts">
import Tabs from 'primevue/tabs'
import TabList from 'primevue/tablist'
import Tab from 'primevue/tab'
import TabPanels from 'primevue/tabpanels'
import TabPanel from 'primevue/tabpanel'
import UserApplications from '@/components/staff/UserApplications.vue'
import { useI18n } from 'vue-i18n'
import StaffPmsTable from '@/components/staff_pm/StaffPmsTable.vue'
import CssSheetList from '@/components/CssSheetList.vue'
import { useUserStore } from '@/stores/user'

const { t } = useI18n()
const userStore = useUserStore()
</script>

<style scoped>
#staff-dashboard {
  .p-tablist-tab-list {
    display: flex;
    justify-content: center;
  }
}
</style>
