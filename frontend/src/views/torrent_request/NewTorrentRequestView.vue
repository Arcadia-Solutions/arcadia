<template>
  <div>
    <div class="title">{{ t('torrent_request.new_request') }}</div>
    <Accordion :value="titleGroupAccordionValue" class="upload-step-accordion">
      <AccordionPanel value="0" :disabled="titleGroupDisabled">
        <AccordionHeader>
          <div style="display: flex">
            {{ t('title_group.title') }}
            <div v-if="titleGroupStore.id !== 0" style="display: flex">:<TitleGroupSlimHeader style="margin-left: 5px" :titleGroup="titleGroupStore" /></div>
          </div>
        </AccordionHeader>
        <AccordionContent>
          <CreateOrSelectTitleGroup @done="titleGroupDone" />
        </AccordionContent>
      </AccordionPanel>
    </Accordion>
    <Accordion :value="torrentAccordionValue" class="upload-step-accordion">
      <AccordionPanel value="0">
        <AccordionHeader>{{ t('torrent.torrent') }}</AccordionHeader>
        <AccordionContent>
          <CreateOrEditTorrentRequest @done="torrentRequestDone" />
        </AccordionContent>
      </AccordionPanel>
    </Accordion>
  </div>
</template>
<script setup lang="ts">
import { ref } from 'vue'
import Accordion from 'primevue/accordion'
import AccordionPanel from 'primevue/accordionpanel'
import AccordionHeader from 'primevue/accordionheader'
import AccordionContent from 'primevue/accordioncontent'
import { useTitleGroupStore } from '@/stores/titleGroup'
import { useI18n } from 'vue-i18n'
import TitleGroupSlimHeader from '@/components/title_group/TitleGroupSlimHeader.vue'
import { onMounted } from 'vue'
import CreateOrSelectTitleGroup from '@/components/title_group/CreateOrSelectTitleGroup.vue'
import CreateOrEditTorrentRequest from '@/components/torrent_request/CreateOrEditTorrentRequest.vue'
import { useRouter } from 'vue-router'
import { onUnmounted } from 'vue'
import type { TitleGroup, TitleGroupLite, TorrentRequest } from '@/services/api-schema'

const { t } = useI18n()
const router = useRouter()

const titleGroupStore = ref(useTitleGroupStore())

const titleGroupAccordionValue = ref('0')
const titleGroupDisabled = ref(false)
const torrentAccordionValue = ref('0')

const titleGroupDone = (titleGroup?: TitleGroup | TitleGroupLite) => {
  titleGroupAccordionValue.value = ''
  titleGroupDisabled.value = true
  if (titleGroup) {
    titleGroupStore.value.id = titleGroup.id
    titleGroupStore.value.content_type = titleGroup.content_type
    titleGroupStore.value.name = titleGroup.name
    titleGroupStore.value.original_release_date = titleGroup.original_release_date
  }
}

const torrentRequestDone = (torrentRequest: TorrentRequest) => {
  router.push(`/torrent-request/${torrentRequest.id}`)
}

// the store is used to autofill some steps if the user comes from an existing title group
onMounted(() => {
  if (titleGroupStore.value.id !== 0) {
    titleGroupDone()
  }
})
onUnmounted(() => {
  titleGroupStore.value.$reset()
})
</script>

<style scoped>
.announce-url {
  display: flex;
  justify-content: center;
  margin-bottom: 15px;
  .explanation {
    margin-right: 10px;
    font-weight: bold;
  }
}
.form {
  padding-top: 5px;
}
.title {
  margin-bottom: 20px;
  color: var(--color-primary);
}
.upload-step-accordion {
  margin-bottom: 20px;
}
</style>
