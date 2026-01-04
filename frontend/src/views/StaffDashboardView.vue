<template>
  <div id="staff-dashboard">
    <Tabs :value="currentTab" @update:value="tabChanged">
      <TabList>
        <Tab value="userApplications" v-if="userStore.permissions.includes('get_user_application')">{{ t('staff.user_application.user_application', 2) }}</Tab>
        <Tab value="staffPms" v-if="userStore.permissions.includes('read_staff_pm')">{{ t('staff_pm.staff_pm', 2) }}</Tab>
        <Tab value="cssSheets" v-if="userStore.permissions.includes('edit_css_sheet') || userStore.permissions.includes('create_css_sheet')">
          {{ t('css_sheet.css_sheet', 2) }}
        </Tab>
        <Tab value="arcadiaSettings" v-if="userStore.permissions.includes('edit_arcadia_settings')">{{ t('arcadia_settings.arcadia_settings') }}</Tab>
        <Tab value="userClasses" v-if="userStore.permissions.includes('edit_user_class') || userStore.permissions.includes('create_user_class')">
          {{ t('user_class.user_class', 2) }}
        </Tab>
        <Tab
          value="donations"
          v-if="
            userStore.permissions.includes('search_donation') ||
            userStore.permissions.includes('create_donation') ||
            userStore.permissions.includes('edit_donation') ||
            userStore.permissions.includes('delete_donation')
          "
        >
          {{ t('donation.donation', 2) }}
        </Tab>
        <Tab value="unauthorizedAccess" v-if="userStore.permissions.includes('search_unauthorized_access')">
          {{ t('unauthorized_access.unauthorized_access', 2) }}
        </Tab>
        <Tab value="userEditLogs" v-if="userStore.permissions.includes('search_user_edit_change_logs')">
          {{ t('user_edit_log.user_edit_log', 2) }}
        </Tab>
      </TabList>
      <!-- tabs are loaded only if they are focused -->
      <TabPanels>
        <TabPanel value="userApplications" v-if="userStore.permissions.includes('get_user_application') && currentTab === 'userApplications'">
          <UserApplications />
        </TabPanel>
        <TabPanel value="staffPms" v-if="userStore.permissions.includes('read_staff_pm') && currentTab === 'staffPms'">
          <StaffPmsTable />
        </TabPanel>
        <TabPanel
          value="cssSheets"
          v-if="(userStore.permissions.includes('edit_css_sheet') || userStore.permissions.includes('create_css_sheet')) && currentTab === 'cssSheets'"
        >
          <CssSheetList showStaffActions />
        </TabPanel>
        <TabPanel value="arcadiaSettings" v-if="userStore.permissions.includes('edit_arcadia_settings') && currentTab === 'arcadiaSettings'">
          <ArcadiaSettings />
        </TabPanel>
        <TabPanel
          value="userClasses"
          v-if="(userStore.permissions.includes('edit_user_class') || userStore.permissions.includes('create_user_class')) && currentTab === 'userClasses'"
        >
          <UserClassesTable />
        </TabPanel>
        <TabPanel
          value="donations"
          v-if="
            (userStore.permissions.includes('search_donation') ||
              userStore.permissions.includes('create_donation') ||
              userStore.permissions.includes('edit_donation') ||
              userStore.permissions.includes('delete_donation')) &&
            currentTab === 'donations'
          "
        >
          <DonationsTable />
        </TabPanel>
        <TabPanel value="unauthorizedAccess" v-if="userStore.permissions.includes('search_unauthorized_access') && currentTab === 'unauthorizedAccess'">
          <UnauthorizedAccessTable />
        </TabPanel>
        <TabPanel value="userEditLogs" v-if="userStore.permissions.includes('search_user_edit_change_logs') && currentTab === 'userEditLogs'">
          <UserEditLogsTable />
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
import UnauthorizedAccessTable from '@/components/staff/UnauthorizedAccessTable.vue'
import UserEditLogsTable from '@/components/staff/UserEditLogsTable.vue'
import { useUserStore } from '@/stores/user'
import { useRouter } from 'vue-router'
import { onMounted, ref, watch } from 'vue'

const { t } = useI18n()
const userStore = useUserStore()
const router = useRouter()

const currentTab = ref('')

const tabChanged = (tab: string | number) => {
  router.push({ query: { tab } })
}

onMounted(() => {
  if (router.currentRoute.value.query.tab) {
    currentTab.value = router.currentRoute.value.query.tab as string
  } else {
    currentTab.value = 'userApplications'
  }
})

watch(
  () => router.currentRoute.value.query.tab,
  (newTab) => {
    if (newTab) {
      currentTab.value = newTab as string
    }
  },
)
</script>

<style scoped>
#staff-dashboard {
  .p-tablist-tab-list {
    display: flex;
    justify-content: center;
  }
}
</style>
