<template>
  <div>
    <div class="title">{{ t('torrent.upload_torrent') }}</div>
    <div class="announce-url">
      <div class="explanation">{{ t('torrent.announce_url') }}:</div>
      <div class="url">{{ announceUrl }}</div>
    </div>
    <Accordion :value="titleGroupAccordionValue" class="upload-step-accordion">
      <AccordionPanel value="0" :disabled="titleGroupDisabled">
        <AccordionHeader>
          <div style="display: flex">
            {{ t('title_group.title') }}
            <div v-if="titleGroupStore.id !== 0" style="display: flex">:<TitleGroupSlimHeader style="margin-left: 5px" :titleGroup="titleGroupStore" /></div>
          </div>
        </AccordionHeader>
        <AccordionContent>
          <!-- <CreateOrSelectTitleGroup @done="titleGroupDone" /> -->
          <CreateOrSelectTitleGroup @editionGroupDataFound="editionGroupDataFound" @done="titleGroupDone" @siwtchedToCreate="titleGroupSwitchedToCreate" />
        </AccordionContent>
      </AccordionPanel>
    </Accordion>
    <Accordion
      :value="editionGroupAccordionValue"
      class="upload-step-accordion"
      v-tooltip.top="{
        value: t('edition_group.complete_title_group_first'),
        disabled: titleGroupDisabled,
      }"
    >
      <AccordionPanel value="0" :disabled="editionGroupDisabled || !titleGroupDisabled">
        <AccordionHeader>
          <span>
            {{ t('torrent.edition') }}<span v-if="editionGroup" class="bold">: {{ getEditionGroupSlug(editionGroup) }}</span>
          </span>
        </AccordionHeader>
        <AccordionContent>
          <CreateOrSelectEditionGroup ref="editionRef" @done="editionGroupDone" />
        </AccordionContent>
      </AccordionPanel>
    </Accordion>
    <Accordion :value="torrentAccordionValue" class="upload-step-accordion">
      <AccordionPanel value="0">
        <AccordionHeader>{{ t('torrent.torrent') }}</AccordionHeader>
        <AccordionContent>
          <CreateOrEditTorrent @done="torrentDone" />
        </AccordionContent>
      </AccordionPanel>
    </Accordion>
  </div>
</template>
<script setup lang="ts">
import { onUnmounted, ref } from 'vue'
import Accordion from 'primevue/accordion'
import AccordionPanel from 'primevue/accordionpanel'
import AccordionHeader from 'primevue/accordionheader'
import AccordionContent from 'primevue/accordioncontent'
import CreateOrSelectEditionGroup from '@/components/edition_group/CreateOrSelectEditionGroup.vue'
import CreateOrEditTorrent from '@/components/torrent/CreateOrEditTorrent.vue'
import { useEditionGroupStore } from '@/stores/editionGroup'
import { useTitleGroupStore } from '@/stores/titleGroup'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { getUploadInformation, type Torrent } from '@/services/api/torrentService'
import TitleGroupSlimHeader from '@/components/title_group/TitleGroupSlimHeader.vue'
import { onMounted } from 'vue'
import { getEditionGroupSlug } from '@/services/helpers'
import CreateOrSelectTitleGroup from '@/components/title_group/CreateOrSelectTitleGroup.vue'
import type { TitleGroup, TitleGroupLite } from '@/services/api/titleGroupService'
import type { EditionGroupInfoLite, UserCreatedEditionGroup } from '@/services/api/editionGroupService'

const router = useRouter()
const { t } = useI18n()

const announceUrl = ref('')
const editionGroupStore = ref(useEditionGroupStore())
const titleGroupStore = ref(useTitleGroupStore())

const titleGroupAccordionValue = ref('0')
const titleGroupDisabled = ref(false)
const editionGroupAccordionValue = ref('0')
const editionGroupDisabled = ref(false)
const editionRef = ref<InstanceType<typeof CreateOrSelectEditionGroup>>()
const torrentAccordionValue = ref('0')
const editionGroup = ref<EditionGroupInfoLite | null>(null)

const titleGroupDone = (titleGroup?: TitleGroup | TitleGroupLite) => {
  titleGroupAccordionValue.value = ''
  titleGroupDisabled.value = true
  if (titleGroup) {
    titleGroupStore.value.id = titleGroup.id
    titleGroupStore.value.content_type = titleGroup.content_type
    titleGroupStore.value.name = titleGroup.name
    titleGroupStore.value.original_release_date = titleGroup.original_release_date
    if ('edition_groups' in titleGroup) {
      titleGroupStore.value.edition_groups = titleGroup.edition_groups
    }
  }
}

const titleGroupSwitchedToCreate = () => {
  if (editionRef.value) editionRef.value.action = 'create'
}

const editionGroupDataFound = (eg: UserCreatedEditionGroup) => {
  if (editionRef.value) editionRef.value.updateEditionGroupForm(eg)
}

const editionGroupDone = (eg: EditionGroupInfoLite) => {
  editionGroup.value = eg
  // editionGroupStore.value = { ...eg, ...editionGroupStore.value }
  editionGroupStore.value.id = eg.id
  editionGroupStore.value.source = eg.source
  editionGroupStore.value.additional_information = eg.additional_information
  editionGroupAccordionValue.value = ''
  editionGroupDisabled.value = true
  // torrentAccordionValue.value = '0'
}

const torrentDone = (torrent: Torrent) => {
  router.push('/title-group/' + titleGroupStore.value.id + '?torrentId=' + torrent.id)
}

// the store is used to autofill some steps if the user comes from an existing title group
onMounted(() => {
  if (titleGroupStore.value.id !== 0) {
    titleGroupDone()
  }
  getUploadInformation().then((data) => {
    announceUrl.value = data.announce_url
  })
})
onUnmounted(() => {
  titleGroupStore.value.$reset()
  editionGroupStore.value.$reset()
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
