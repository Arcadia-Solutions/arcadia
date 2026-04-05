<template>
  <div class="edit-artist">
    <FloatLabel style="margin-bottom: 30px">
      <InputText name="name" v-model="editedArtist.name" />
      <label for="name">{{ t('artist.name') }}</label>
    </FloatLabel>
    <BBCodeEditor
      :initialValue="initialArtist.description"
      :label="t('general.description')"
      @valueChange="(val: string) => (editedArtist.description = val)"
    />
    <div class="pictures input-list">
      <label>{{ t('general.pictures') }}</label>
      <ImageUploader v-if="publicArcadiaSettings.display_image_host_drag_and_drop" @uploaded="onImageUploaded" />
      <div v-for="(_picture, index) in editedArtist.pictures" :key="index">
        <InputText size="small" v-model="editedArtist.pictures[index]" />
        <Button v-if="index == 0" @click="addPicture" icon="pi pi-plus" size="small" />
        <Button v-if="index != 0 || editedArtist.pictures.length > 1" @click="removePicture(index)" icon="pi pi-minus" size="small" />
      </div>
    </div>
    <div class="aliases input-list">
      <label>{{ t('general.alias', 2) }}</label>
      <div v-for="(_alias, index) in editedArtist.aliases" :key="index">
        <InputText size="small" v-model="editedArtist.aliases[index]" />
        <Button v-if="index == 0" @click="addAlias" icon="pi pi-plus" size="small" />
        <Button v-if="index != 0 || editedArtist.aliases.length > 1" @click="removeAlias(index)" icon="pi pi-minus" size="small" />
      </div>
    </div>
    <div class="wrapper-center">
      <Button :label="t('general.confirm')" size="small" :loading="loading" @click="sendEdits()" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { FloatLabel, InputText } from 'primevue'
import Button from 'primevue/button'
import { ref, onMounted, toRaw } from 'vue'
import { useI18n } from 'vue-i18n'
import BBCodeEditor from '../community/BBCodeEditor.vue'
import ImageUploader from '../ImageUploader.vue'
import { editArtist, type Artist, type EditedArtist } from '@/services/api-schema'
import { usePublicArcadiaSettingsStore } from '@/stores/publicArcadiaSettings'

const { t } = useI18n()
const publicArcadiaSettings = usePublicArcadiaSettingsStore()

const props = defineProps<{
  initialArtist: EditedArtist
}>()

const editedArtist = ref<EditedArtist>({
  id: 0,
  name: '',
  description: '',
  pictures: [],
  aliases: [],
})
const loading = ref(false)

const emit = defineEmits<{
  done: [Artist]
}>()

const addPicture = () => {
  editedArtist.value.pictures.push('')
}

const removePicture = (index: number) => {
  editedArtist.value.pictures.splice(index, 1)
}

const addAlias = () => {
  editedArtist.value.aliases.push('')
}

const removeAlias = (index: number) => {
  editedArtist.value.aliases.splice(index, 1)
}

const onImageUploaded = (url: string) => {
  if (editedArtist.value.pictures.length === 1 && editedArtist.value.pictures[0] === '') {
    editedArtist.value.pictures[0] = url
  } else {
    editedArtist.value.pictures.push(url)
  }
}

const sendEdits = () => {
  loading.value = true
  editedArtist.value.pictures = editedArtist.value.pictures.filter((picture) => picture.trim() !== '')
  editedArtist.value.aliases = editedArtist.value.aliases.filter((alias) => alias.trim() !== '')
  editArtist(editedArtist.value).then((newArtist) => {
    loading.value = false
    emit('done', newArtist)
  })
}

onMounted(() => {
  editedArtist.value = structuredClone(toRaw(props.initialArtist))
  if (editedArtist.value.pictures.length === 0) {
    editedArtist.value.pictures = ['']
  }
  if (editedArtist.value.aliases.length === 0) {
    editedArtist.value.aliases = ['']
  }
})
</script>

<style scoped>
.edit-artist {
  width: 50vw;
}
.pictures {
  margin-top: 20px;
  margin-bottom: 20px;
}
.aliases {
  margin-bottom: 20px;
}
.input-list {
  label {
    display: block;
    margin-bottom: 10px;
  }
  div {
    display: flex;
    gap: 5px;
    margin-bottom: 5px;
    input {
      flex: 1;
    }
  }
}
</style>
