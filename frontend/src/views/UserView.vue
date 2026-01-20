<template>
  <div id="user-view" v-if="user">
    <div class="main">
      <div class="top-bar">
        <div class="username">
          {{ user.username }}
          <i v-if="user.banned" v-tooltip.top="t('user.banned')" class="banned pi pi-ban" />
          <i v-if="!user.banned && user.warned" v-tooltip.top="t('user.warned')" class="warned pi pi-exclamation-triangle" />
        </div>
        <div class="actions">
          <RouterLink :to="`/conversation/new?receiverId=${user.id}&username=${user.username}`" class="no-color" v-if="userStore.id !== user.id">
            <i v-tooltip.top="t('user.message_user', [user.username])" class="pi pi-envelope" />
          </RouterLink>
          <template v-if="userStore.permissions.includes('edit_user_permissions')">
            <i v-tooltip.top="t('user.manage_permissions')" class="cursor-pointer pi pi-key" @click="editPermissionsDialogVisible = true" />
          </template>
          <template v-if="userStore.permissions.includes('change_user_class')">
            <i v-tooltip.top="t('user.change_user_class')" class="cursor-pointer pi pi-crown" @click="changeUserClassDialogVisible = true" />
          </template>
          <template v-if="userStore.permissions.includes('lock_user_class')">
            <i
              v-if="user.class_locked"
              v-tooltip.top="t('user.unlock_user_class')"
              class="cursor-pointer pi pi-unlock"
              @click="lockUnlockClassDialogVisible = true"
            />
            <i v-else v-tooltip.top="t('user.lock_user_class')" class="cursor-pointer pi pi-lock" @click="lockUnlockClassDialogVisible = true" />
          </template>
          <template v-if="userStore.permissions.includes('warn_user') && userStore.id !== user.id">
            <i v-tooltip.top="t('user.warn')" class="cursor-pointer pi pi-exclamation-triangle" @click="warnUserDialogVisible = true" />
          </template>
          <template v-if="userStore.id === user.id">
            <i v-tooltip.top="t('general.edit')" class="cursor-pointer pi pi-pen-to-square" @click="editUserDialogVisible = true" />
          </template>
        </div>
      </div>
      <ContentContainer :containerTitle="t('general.description')" class="section">
        <BBCodeRenderer :content="user.description" />
      </ContentContainer>
      <ContentContainer v-if="peers" :containerTitle="t('torrent.clients_and_ips')" class="section">
        <PeerTable :peers />
      </ContentContainer>
      <LatestTorrents
        :titleGroups="uploadedTorrents"
        class="section"
        :containerTitle="t('user.uploads')"
        :containerTitleLink="`/torrents?torrent_created_by_id=${user.id}`"
        type="uploads"
      />
      <LatestTorrents
        :titleGroups="snatchedTorrents"
        class="section"
        :containerTitle="t('user.snatches')"
        :containerTitleLink="`/torrents?torrent_snatched_by_id=${user.id}&order_by_column=torrent_snatched_at`"
        type="snatches"
      />
    </div>
    <UserSidebar :user class="sidebar" />
  </div>
  <Dialog closeOnEscape modal :header="t('user.warn_user')" v-model:visible="warnUserDialogVisible">
    <WarnUserDialog @warned="warnUserDialogVisible = false" />
  </Dialog>
  <Dialog closeOnEscape modal :header="t('user.edit_profile')" v-model:visible="editUserDialogVisible">
    <EditUserDialog @done="userEdited" :initialUser="user as EditedUser" v-if="user" />
  </Dialog>
  <Dialog closeOnEscape modal :header="t('user.manage_permissions')" v-model:visible="editPermissionsDialogVisible">
    <EditPermissionsDialog @saved="permissionsSaved" :userId="user!.id" v-if="editPermissionsDialogVisible && user" />
  </Dialog>
  <Dialog closeOnEscape modal :header="t('user.change_user_class')" v-model:visible="changeUserClassDialogVisible">
    <ChangeUserClassDialog @saved="userClassChanged" :userId="user!.id" :currentClassName="user!.class_name" v-if="changeUserClassDialogVisible && user" />
  </Dialog>
  <Dialog
    closeOnEscape
    modal
    :header="user?.class_locked ? t('user.unlock_user_class') : t('user.lock_user_class')"
    v-model:visible="lockUnlockClassDialogVisible"
  >
    <LockUnlockUserClassDialog @saved="classLockChanged" :userId="user!.id" :classLocked="user!.class_locked" v-if="lockUnlockClassDialogVisible && user" />
  </Dialog>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import PeerTable from '@/components/user/PeerTable.vue'
import { useUserStore } from '@/stores/user'
import { useRoute } from 'vue-router'
import UserSidebar from '@/components/user/UserSidebar.vue'
import BBCodeRenderer from '@/components/community/BBCodeRenderer.vue'
import ContentContainer from '@/components/ContentContainer.vue'
import { useI18n } from 'vue-i18n'
import WarnUserDialog from '@/components/user/WarnUserDialog.vue'
import { Dialog } from 'primevue'
import { watch } from 'vue'
import LatestTorrents from '@/components/torrent/LatestTorrents.vue'
import EditUserDialog from '@/components/user/EditUserDialog.vue'
import EditPermissionsDialog from '@/components/user/EditPermissionsDialog.vue'
import ChangeUserClassDialog from '@/components/user/ChangeUserClassDialog.vue'
import LockUnlockUserClassDialog from '@/components/user/LockUnlockUserClassDialog.vue'
import { getMe, getUser, type EditedUser, type Peer, type PublicUser, type TitleGroupHierarchyLite, type User } from '@/services/api-schema'

const peers = ref<Peer[] | null>(null)
const user = ref<User | PublicUser | null>(null)
const uploadedTorrents = ref<TitleGroupHierarchyLite[]>([])
const snatchedTorrents = ref<TitleGroupHierarchyLite[]>([])
const siteName = import.meta.env.VITE_SITE_NAME

const userStore = useUserStore()
const route = useRoute()
const { t } = useI18n()

const warnUserDialogVisible = ref(false)
const editUserDialogVisible = ref(false)
const editPermissionsDialogVisible = ref(false)
const changeUserClassDialogVisible = ref(false)
const lockUnlockClassDialogVisible = ref(false)

const userEdited = (userEdited: EditedUser) => {
  user.value = { ...user.value, ...userEdited } as User
  editUserDialogVisible.value = false
}

const permissionsSaved = () => {
  editPermissionsDialogVisible.value = false
}

const userClassChanged = (className: string) => {
  if (user.value) {
    user.value.class_name = className
  }
  changeUserClassDialogVisible.value = false
}

const classLockChanged = (classLocked: boolean) => {
  if (user.value) {
    user.value.class_locked = classLocked
  }
  lockUnlockClassDialogVisible.value = false
}

const fetchUser = async () => {
  if (parseInt(route.params.id.toString()) == userStore.id) {
    // logged in user
    ;({
      peers: peers.value,
      user: user.value,
      last_five_uploaded_torrents: uploadedTorrents.value,
      last_five_snatched_torrents: snatchedTorrents.value,
    } = await getMe())
    userStore.setUser(user.value as User)
  } else {
    // viewing another user
    ;({
      user: user.value,
      last_five_uploaded_torrents: uploadedTorrents.value,
      last_five_snatched_torrents: snatchedTorrents.value,
    } = await getUser(parseInt(route.params.id.toString())))
  }

  document.title = `User '${user.value.username}' - ${siteName}`
}

onMounted(async () => {
  fetchUser()
})

watch(
  () => route.params.id,
  (newId, oldId) => {
    if (oldId !== undefined) {
      fetchUser()
    }
  },
  { immediate: true },
)
</script>

<style scoped>
#user-view {
  display: flex;
}
.main {
  width: 75%;
  margin-right: 10px;
}
.top-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.username {
  font-weight: bold;
  font-size: 1.3em;
  margin-bottom: 10px;
  .banned {
    color: red;
  }
  .warned {
    color: yellow;
  }
}
.actions {
  i {
    margin-left: 7px;
  }
}
.section {
  margin-bottom: 15px;
}
.sidebar {
  width: 25%;
}
</style>
