<template>
  <div id="select-edition-group" v-if="action == 'select'">
    <FloatLabel class="select-edition">
      <Select
        v-model="selectedEditionGroup"
        @value-change="editionGroupSelected"
        inputId="edition_group"
        :options="[...titleGroupStore.edition_groups, t('general.create_new_one')]"
        size="small"
        class="select-existing-edition"
      >
        <template #value="slotProps">
          <span v-if="slotProps.value && typeof slotProps.value !== 'string'">
            {{ getEditionGroupSlug(slotProps.value) }}
          </span>
        </template>
        <template #option="slotProps">
          <span v-if="typeof slotProps.option === 'string'"> {{ slotProps.option }}</span>
          <span v-else>
            {{ getEditionGroupSlug(slotProps.option) }}
          </span>
        </template>
      </Select>
      <label for="edition_group">{{ t('torrent.edition') }}</label>
    </FloatLabel>
  </div>
  <div v-if="action === 'create'">
    <CreateOrEditEditionGroup
      ref="createOrEditEditionGroupRef"
      :titleGroup="titleGroupStore"
      @validated="sendEditionGroup"
      :sending-edition-group="creatingEditionGroup"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, toRaw } from 'vue'
import FloatLabel from 'primevue/floatlabel'
import Select from 'primevue/select'
import { useTitleGroupStore } from '@/stores/titleGroup'
import CreateOrEditEditionGroup from './CreateOrEditEditionGroup.vue'
import { getEditionGroupSlug } from '@/services/helpers'
import { useI18n } from 'vue-i18n'
import { createEditionGroup, type EditionGroup, type EditionGroupInfoLite, type UserCreatedEditionGroup } from '@/services/api-schema'

const action = ref<'create' | 'select'>('select')

const titleGroupStore = useTitleGroupStore()
const selectedEditionGroup = ref<EditionGroupInfoLite | string | null>(null)
const creatingEditionGroup = ref(false)

const { t } = useI18n()
const createOrEditEditionGroupRef = ref<InstanceType<typeof CreateOrEditEditionGroup>>()

const emit = defineEmits<{
  done: [editionGroup: EditionGroupInfoLite]
}>()
const editionGroupSelected = () => {
  // this should be an invariant - TODO: should we emit a warning if the value is actually null?
  if (typeof selectedEditionGroup.value === 'string') {
    action.value = 'create'
  } else if (selectedEditionGroup.value) {
    emit('done', selectedEditionGroup.value)
  }
}
const sendEditionGroup = (editionGroupForm?: UserCreatedEditionGroup) => {
  creatingEditionGroup.value = true
  const formattededitionGroupForm = structuredClone(toRaw(editionGroupForm))
  if (!formattededitionGroupForm) return
  // otherwise there is a json parse error, last char is "Z"
  // formattededitionGroupForm.release_date = formattededitionGroupForm.release_date.slice(0, -1)

  formattededitionGroupForm.additional_information = formattededitionGroupForm.additional_information
    ? Object.fromEntries(Object.entries(formattededitionGroupForm.additional_information).filter(([, value]) => value !== null && value !== ''))
    : {}
  formattededitionGroupForm.covers = formattededitionGroupForm.covers.filter((cover) => cover.trim() !== '')
  formattededitionGroupForm.external_links = formattededitionGroupForm.external_links.filter((link) => link.trim() !== '')
  formattededitionGroupForm.distributor = formattededitionGroupForm.distributor == '' ? null : formattededitionGroupForm.distributor

  createEditionGroup(formattededitionGroupForm)
    .then((data: EditionGroup) => {
      emit('done', data)
    })
    .finally(() => {
      creatingEditionGroup.value = false
    })
}

const updateEditionGroupForm = (form: UserCreatedEditionGroup) => {
  if (createOrEditEditionGroupRef.value) {
    createOrEditEditionGroupRef.value.updateEditionGroupForm(form)
  }
}

defineExpose({
  updateEditionGroupForm,
  action,
})
</script>
<style scoped>
.select-edition {
  margin-top: 0;
}
.select-existing-edition {
  width: 500px;
}
.validate-button {
  margin-top: 20px;
}
</style>
