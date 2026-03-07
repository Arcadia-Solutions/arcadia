<template>
  <div class="title">{{ t('user_settings.settings') }}</div>
  <div class="settings" v-if="updatedSettings">
    <ContentContainer class="section" :container-title="t('user_settings.appearance')">
      <div class="line">
        {{ t('user_settings.css_sheet') }}:
        <Button
          :label="updatedSettings.css_sheet_name ?? 'Default'"
          size="small"
          style="margin-left: 5px"
          v-tooltip.top="t('user_settings.change_css_sheet')"
          @click="changeCssSheetDialogVisible = true"
        />
      </div>
    </ContentContainer>
    <ContentContainer v-if="publicSettings.irc_enabled" class="section" :container-title="t('user_settings.irc')">
      <div class="line">
        {{ t('user_settings.irc_account') }}:
        <Button
          v-if="userStore.irc_password"
          :label="t('user_settings.reset_irc_password')"
          size="small"
          severity="warn"
          style="margin-left: 5px"
          @click="ircDialogVisible = true"
        />
        <Button v-else :label="t('user_settings.create_irc_account')" size="small" style="margin-left: 5px" @click="ircDialogVisible = true" />
      </div>
      <div v-if="userStore.irc_password" class="line" style="margin-top: 10px">
        <b>{{ t('user_settings.irc_username_label') }}:</b> {{ userStore.username }}
      </div>
      <div v-if="userStore.irc_password" class="line" style="margin-top: 5px">
        <b>{{ t('user_settings.irc_password_label') }}:</b>
        <code v-if="ircPasswordVisible" style="margin-left: 5px">{{ userStore.irc_password }}</code>
        <Button
          :label="ircPasswordVisible ? t('user_settings.hide_irc_password') : t('user_settings.show_irc_password')"
          size="small"
          severity="secondary"
          style="margin-left: 5px"
          @click="ircPasswordVisible = !ircPasswordVisible"
        />
      </div>
    </ContentContainer>
  </div>
  <div class="wrapper-center">
    <Button :label="t('user_settings.save')" @click="saveSettings" />
  </div>
  <Dialog closeOnEscape modal :header="t('user_settings.change_css_sheet')" v-model:visible="changeCssSheetDialogVisible">
    <CssSheetList @sheetClicked="cssSheetChanged" />
  </Dialog>
  <Dialog
    closeOnEscape
    modal
    :header="userStore.irc_password ? t('user_settings.reset_irc_password') : t('user_settings.create_irc_account')"
    v-model:visible="ircDialogVisible"
  >
    <IrcAccountDialog v-if="ircDialogVisible" />
  </Dialog>
</template>
<script setup lang="ts">
import { isEqual } from 'lodash-es'
import { onBeforeUnmount, onMounted, ref, toRaw } from 'vue'
import { useI18n } from 'vue-i18n'
import ContentContainer from '@/components/ContentContainer.vue'
import { Button, Dialog } from 'primevue'
import CssSheetList from '@/components/CssSheetList.vue'
import IrcAccountDialog from '@/components/user/IrcAccountDialog.vue'
import { showToast } from '@/main'
import { useRouter, useRoute } from 'vue-router'
import { getUserSettings, updateUserSettings, type CssSheet, type UserSettings } from '@/services/api-schema'
import { useUserStore } from '@/stores/user'
import { usePublicArcadiaSettingsStore } from '@/stores/publicArcadiaSettings'

const { t } = useI18n()
const router = useRouter()
const route = useRoute()
const userStore = useUserStore()
const publicSettings = usePublicArcadiaSettingsStore()

const initialSettings = ref<UserSettings>()
const updatedSettings = ref<UserSettings>()
const changeCssSheetDialogVisible = ref(false)
const ircDialogVisible = ref(false)
const ircPasswordVisible = ref(false)

const cssSheetChanged = (cssSheet: CssSheet) => {
  if (!updatedSettings.value) return
  updatedSettings.value.css_sheet_name = cssSheet.name
  changeCssSheetDialogVisible.value = false
}

const saveSettings = () => {
  if (!updatedSettings.value || !initialSettings.value) return
  if (!isEqual(initialSettings.value, updatedSettings.value)) {
    updateUserSettings(updatedSettings.value).then(() => {
      initialSettings.value = updatedSettings.value
      router.push({ query: { saved: 'true' } }).then(() => {
        router.go(0)
      })
    })
  } else {
    showToast('', t('user_settings.settings_were_not_changed'), 'info', 2000)
  }
}

onMounted(() => {
  getUserSettings().then((settings) => {
    initialSettings.value = settings
    updatedSettings.value = structuredClone(toRaw(initialSettings.value))
  })
  if (route.query.saved === 'true') {
    showToast('', t('user_settings.saved'), 'success', 3000)
    router.push({ query: {} })
  }
})
onBeforeUnmount(() => {
  if (updatedSettings.value) {
    if (!isEqual(initialSettings.value, updatedSettings.value)) {
      // TODO: alert of unsaved settings
    }
  }
})
</script>

<style scoped>
.line {
  align-items: center;
}
.section {
  margin-bottom: 15px;
}
</style>
