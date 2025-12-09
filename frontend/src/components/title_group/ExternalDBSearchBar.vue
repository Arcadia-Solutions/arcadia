<template>
  <FloatLabel class="external-db-input">
    <IconField>
      <InputText size="small" :name="`input-${database}`" v-model="externalDBId" />
      <label :for="`input-${database}`">{{ inputPlaceholder }}</label>
      <InputIcon
        :class="{
          pi: true,
          'pi-search': !loading,
          'pi-hourglass': loading,
          'cursor-pointer': true,
        }"
        @click="getExternalDBData(externalDBId)"
      />
    </IconField>
  </FloatLabel>
</template>
<script lang="ts" setup>
import { getComicVineData, getIsbnData, getMusicbranzData, getTMDBData, type ExternalDBData } from '@/services/api-schema'
import { FloatLabel, IconField, InputIcon, InputText } from 'primevue'
import { ref } from 'vue'

const emit = defineEmits<{
  dataFound: [ExternalDBData]
}>()
const props = defineProps<{
  inputPlaceholder: string
  database: string
}>()

const externalDBId = ref('')
const loading = ref(false)

const getExternalDBData = async (item_id: string | number) => {
  loading.value = true

  let request: Promise<ExternalDBData>

  switch (props.database) {
    case 'isbn': {
      request = getIsbnData(item_id.toString())
      break
    }
    case 'comic_vine': {
      request = getComicVineData(item_id.toString())
      break
    }
    case 'musicbrainz': {
      request = getMusicbranzData(item_id.toString())
      break
    }
    case 'tmdb': {
      request = getTMDBData(item_id.toString())
      break
    }
    default:
      loading.value = false
      throw 'database not supported'
  }

  return request
    .then((data) => {
      emit('dataFound', data)
      return data
    })
    .finally(() => {
      loading.value = false
    })
}
</script>
