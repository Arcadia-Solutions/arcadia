<template>
  <div class="actions wrapper-center">
    <RouterLink to="/css-sheets/new" v-if="showStaffActions">
      <i class="pi pi-plus" style="color: white" v-tooltip.top="t('css_sheet.create_sheet')" />
    </RouterLink>
  </div>
  <div class="sheets">
    <div v-for="sheet in cssSheets" :key="sheet.name" class="sheet">
      <img :src="sheet.preview_image_url" width="300em" alt="CSS Sheet Image" class="cursor-pointer" @click="emit('sheetClicked', sheet)" />
      <div class="name">
        {{ sheet.name }} <span v-if="defaultSheetName === sheet.name">({{ t('general.default') }})</span>
        <RouterLink :to="{ name: 'EditCssSheet', params: { name: sheet.name } }">
          <i v-if="showStaffActions" class="pi pi-pen-to-square" style="color: white; margin-left: 4px" v-tooltip.top="t('user_settings.edit_css_sheet')" />
        </RouterLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getCssSheets, type CssSheet } from '@/services/api/cssSheetService'
import { onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'

defineProps<{
  showStaffActions?: boolean
}>()
const emit = defineEmits<{
  sheetClicked: [CssSheet]
}>()

const { t } = useI18n()

const cssSheets = ref<CssSheet[]>([])
const defaultSheetName = ref<string>('')

onMounted(() => {
  getCssSheets().then((sheets) => {
    cssSheets.value = sheets.css_sheets
    defaultSheetName.value = sheets.default_sheet_name
  })
})
</script>

<style scoped>
.sheets {
  display: flex;
  flex-wrap: wrap;
  max-width: 70vw;
}
.sheet {
  margin: 10px;
  .name {
    text-align: center;
    font-weight: bold;
  }
}
</style>
