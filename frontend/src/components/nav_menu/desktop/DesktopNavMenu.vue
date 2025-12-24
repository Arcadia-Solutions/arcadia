<template>
  <div id="desktopNavMenu" class="actions">
    <!-- the title group store is filled on the title group page, if the user clicks on the upload button from there
    it should be emptied as the upload form autofills the title group -->
    <RouterLink to="/upload" v-if="userStore.permissions.includes('upload_torrent')" @click="useTitleGroupStore().$reset()">
      <Button icon="pi pi-upload" severity="secondary" size="small" />
    </RouterLink>
    <Button icon="pi pi-moon" @click="toggleDarkMode()" severity="secondary" size="small" />
    <RouterLink :to="`/user/${user.id}`">
      <Button :onmouseenter="show" :onmouseleave="onLeaveUserIcon" icon="pi pi-user" severity="secondary" size="small" />
      <Popover :onmouseleave="onLeavePopover" :onmouseenter="() => (isHoveringDropdown = true)" :dismissable="false" ref="op">
        <RouterLink to="/conversations">
          <div class="user-action flex gap-2 px-2">
            <i class="pi pi-envelope" />
            <small class="font-medium">{{ t('conversation.conversation', 2) }}</small>
          </div>
        </RouterLink>
        <RouterLink to="/notifications">
          <div class="user-action flex gap-2 px-2">
            <i class="pi pi-bell" />
            <small class="font-medium">{{ t('notification.notification', 2) }}</small>
          </div>
        </RouterLink>
        <RouterLink to="/user-settings">
          <div class="user-action flex gap-2 px-2">
            <i class="pi pi-cog" />
            <small class="font-medium">{{ t('user_settings.settings') }}</small>
          </div>
        </RouterLink>
        <RouterLink to="/staff-pms">
          <div class="user-action flex gap-2 px-2">
            <i class="pi pi-info-circle" />
            <small class="font-medium">{{ t('staff_pm.staff_pm', 2) }}</small>
          </div>
        </RouterLink>
        <div class="danger user-action sign-out flex gap-2 px-2 cursor-pointer" @click="handleLogout">
          <i class="pi pi-sign-out" />
          <small class="font-medium">{{ t('user.logout') }}</small>
        </div>
      </Popover>
    </RouterLink>
  </div>
</template>

<script setup lang="ts">
import { useUserStore } from '@/stores/user'
import { logout } from '@/services/api-schema'
import { Button } from 'primevue'
import Popover from 'primevue/popover'
import { useI18n } from 'vue-i18n'
const { t } = useI18n()
import { ref } from 'vue'
import router from '@/router'
import { useTitleGroupStore } from '@/stores/titleGroup'

const userStore = useUserStore()

const op = ref<InstanceType<typeof Popover> & HTMLAnchorElement>()
const user = useUserStore()
const isHoveringDropdown = ref(false)

const onLeavePopover = () => {
  isHoveringDropdown.value = false
  op.value?.hide()
}

const show = (event: Event) => {
  op.value?.show(event)
}

// delay a bit so we can know if the user unhovered the user icon,
// to hover the popover or not
const onLeaveUserIcon = () => {
  setTimeout(() => {
    if (!isHoveringDropdown.value) {
      op.value?.hide()
    }
  }, 100)
}

const handleLogout = async () => {
  try {
    await logout()
  } catch {
    // Ignore errors (token may already be expired)
  }
  localStorage.removeItem('token')
  localStorage.removeItem('refreshToken')
  localStorage.removeItem('user')
  user.removeUser()
  router.push('/login')
}

const toggleDarkMode = () => {
  document.documentElement.classList.toggle('dark-theme')
}
</script>

<style scoped>
.actions .p-button {
  margin-left: 7px;
}
.user-action {
  padding: 5px 0;
  align-items: center;
  transition: 0.3s ease;
}
</style>
