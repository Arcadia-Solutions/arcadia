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
  </div>
  <div class="wrapper-center">
    <Button :label="t('user_settings.save')" @click="saveSettings" />
  </div>
  <Dialog closeOnEscape modal :header="t('user_settings.change_css_sheet')" v-model:visible="changeCssSheetDialogVisible">
    <CssSheetList @sheetClicked="cssSheetChanged" />
  </Dialog>
</template>
<script setup lang="ts">
import { getUserSettings, updateUserSettings, type UserSettings } from '@/services/api/userService'
import { isEqual } from 'lodash-es'
import { onBeforeUnmount, onMounted, ref, toRaw } from 'vue'
import { useI18n } from 'vue-i18n'
import ContentContainer from '@/components/ContentContainer.vue'
import { Button, Dialog } from 'primevue'
import CssSheetList from '@/components/CssSheetList.vue'
import type { CssSheet } from '@/services/api/cssSheetService'
import { showToast } from '@/main'

const { t } = useI18n()

const initialSettings = ref<UserSettings>()
const updatedSettings = ref<UserSettings>()
const changeCssSheetDialogVisible = ref(false)

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
      showToast('', t('user_settings.saved'), 'success', 2000)
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
