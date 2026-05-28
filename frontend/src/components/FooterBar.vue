<template>
  <div id="footer">
    <div v-if="publicArcadiaSettings.custom_footer" id="custom-footer" v-html="publicArcadiaSettings.custom_footer" />
    {{ t('footer.site_and_design') }}:
    <a href="https://github.com/Arcadia-Solutions/arcadia" target="_blank">Arcadia</a>
  </div>
</template>
<script lang="ts" setup>
import { useI18n } from 'vue-i18n'
import { usePublicArcadiaSettingsStore } from '@/stores/publicArcadiaSettings'

const { t } = useI18n()
const publicArcadiaSettings = usePublicArcadiaSettingsStore()
</script>
<style scoped>
#footer {
  text-align: center;
  margin-bottom: 10px;
}
</style>
