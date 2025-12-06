<template>
  <div id="staff-dashboard">
    <Tabs value="1">
      <TabList>
        <Tab value="0">{{ t('staff.user_application.user_application', 2) }}</Tab>
        <Tab value="1">{{ t('staff_pm.staff_pm', 2) }}</Tab>
        <Tab value="2">{{ t('css_sheet.css_sheet', 2) }}</Tab>
      </TabList>
      <TabPanels>
        <TabPanel value="0">
          <UserApplications />
        </TabPanel>
        <TabPanel value="1">
          <StaffPmsTable />
        </TabPanel>
        <TabPanel value="2">
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

const { t } = useI18n()
</script>

<style scoped>
#staff-dashboard {
  .p-tablist-tab-list {
    display: flex;
    justify-content: center;
  }
}
</style>
