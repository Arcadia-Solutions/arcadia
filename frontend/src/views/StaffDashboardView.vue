<template>
  <div id="staff-dashboard">
    <Tabs value="4">
      <TabList>
        <Tab value="0" v-if="userStore.permissions.includes('get_user_application')">{{ t('staff.user_application.user_application', 2) }}</Tab>
        <Tab value="1" v-if="userStore.permissions.includes('read_staff_pm')">{{ t('staff_pm.staff_pm', 2) }}</Tab>
        <Tab value="2" v-if="userStore.permissions.includes('edit_css_sheet') || userStore.permissions.includes('create_css_sheet')">
          {{ t('css_sheet.css_sheet', 2) }}
        </Tab>
        <Tab value="3" v-if="userStore.permissions.includes('edit_arcadia_settings')">{{ t('arcadia_settings.arcadia_settings') }}</Tab>
        <Tab value="4" v-if="userStore.permissions.includes('edit_user_class') || userStore.permissions.includes('create_user_class')">
          {{ t('user_class.user_class', 2) }}
        </Tab>
        <Tab
          value="5"
          v-if="
            userStore.permissions.includes('search_donation') ||
            userStore.permissions.includes('create_donation') ||
            userStore.permissions.includes('edit_donation') ||
            userStore.permissions.includes('delete_donation')
          "
        >
          {{ t('donation.donation', 2) }}
        </Tab>
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
        <TabPanel value="3" v-if="userStore.permissions.includes('edit_arcadia_settings')">
          <ArcadiaSettings />
        </TabPanel>
        <TabPanel value="4" v-if="userStore.permissions.includes('edit_user_class') || userStore.permissions.includes('create_user_class')">
          <UserClassesTable />
        </TabPanel>
        <TabPanel
          value="5"
          v-if="
            userStore.permissions.includes('search_donation') ||
            userStore.permissions.includes('create_donation') ||
            userStore.permissions.includes('edit_donation') ||
            userStore.permissions.includes('delete_donation')
          "
        >
          <DonationsTable />
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
import ArcadiaSettings from '@/components/staff/ArcadiaSettings.vue'
import UserClassesTable from '@/components/staff/UserClassesTable.vue'
import DonationsTable from '@/components/staff/DonationsTable.vue'
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
