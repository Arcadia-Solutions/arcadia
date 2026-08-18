<template>
  <FloatLabel class="external-db-input">
    <IconField v-tooltip.right="{ value: sourcesTooltip, escape: false, class: 'sources-tooltip' }">
      <InputText size="small" :name="`input-${sourceId}`" v-model="externalDBId" />
      <label :for="`input-${sourceId}`">{{ inputPlaceholder }}</label>
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
import { getExternalSourceData, type ContentType, type ExternalDBData } from '@/services/api-schema'
import { FloatLabel, IconField, InputIcon, InputText } from 'primevue'
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const emit = defineEmits<{
  dataFound: [ExternalDBData]
}>()
const props = defineProps<{
  inputPlaceholder: string
  sourceId: string
  // the websites the source accepts links from, with the content types each of them supports
  sources: { [website: string]: ContentType[] }
  // what the uploader picked, sent along so a plugin only overrides it where the page says otherwise
  contentType: ContentType | null
}>()

// only the websites supporting what the uploader picked are worth listing, and only once there
// are several of them. the tooltip is written as html to show them as a bullet point list
const sourcesTooltip = computed(() => {
  const contentType = props.contentType
  if (contentType === null) {
    return null
  }

  const websites = Object.entries(props.sources)
    .filter(([, contentTypes]) => contentTypes.includes(contentType))
    .map(([website]) => website)

  return websites.length < 2 ? null : `${t('title_group.supported_sources')}<ul>${websites.map((website) => `<li>${website}</li>`).join('')}</ul>`
})

const externalDBId = ref('')
const loading = ref(false)

const getExternalDBData = (item_id: string | number) => {
  loading.value = true

  return getExternalSourceData({ source_id: props.sourceId, url: item_id.toString(), content_type: props.contentType ?? undefined })
    .then((data) => {
      emit('dataFound', data)
      return data
    })
    .finally(() => {
      loading.value = false
    })
}
</script>

<!-- the tooltip is appended to the body, out of the component's tree, so its style cannot be scoped -->
<style>
/* the websites are listed one per line, long names widen the tooltip */
.p-tooltip.sources-tooltip {
  max-width: none;
}
.sources-tooltip .p-tooltip-text {
  white-space: nowrap;
  word-break: normal;
}
.sources-tooltip ul {
  margin: 0;
  padding-left: 15px;
}
</style>
