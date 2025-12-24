<template>
  <div id="add-title-group-to-series-dialog">
    <div class="input-wrapper">
      <InputText placeholder="title group link" v-model="titleGroupLink" />
    </div>
    <div class="wrapper-center" style="margin-top: 10px">
      <Button :label="t('series.add_title_group_to_series')" size="small" :loading @click="addTitleGroup" />
    </div>
  </div>
</template>
<script setup lang="ts">
import { InputText, Button } from 'primevue'
import { useI18n } from 'vue-i18n'
import { ref } from 'vue'
import { addTitleGroupToSeries, type TitleGroup } from '@/services/api-schema'

const { t } = useI18n()

const emit = defineEmits<{
  titleGroupAdded: [TitleGroup]
}>()
const props = defineProps<{
  seriesId: number
}>()

const loading = ref(false)
const titleGroupLink = ref('')

const addTitleGroup = async () => {
  loading.value = true
  const titleGroupId = parseInt(titleGroupLink.value.split('/').pop() as string)

  addTitleGroupToSeries({ series_id: props.seriesId, title_group_id: titleGroupId })
    .then((data) => {
      emit('titleGroupAdded', data)
    })
    .finally(() => (loading.value = false))
}
</script>
<style scoped>
#add-title-group-to-series-dialog {
  width: 70vw;
  display: flex;
  flex-direction: column;
  align-items: center;
}
.input-wrapper {
  width: 100%;
}
</style>
