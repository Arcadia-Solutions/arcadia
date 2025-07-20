<template>
  <Form
    ref="formRef"
    v-slot="$form"
    :initialValues="torrentForm"
    :resolver
    @submit="onFormSubmit"
    validateOnSubmit
    :validateOnValueUpdate="false"
    validateOnBlur
  >
    <div id="create-torrent">
      <div class="mediainfo">
        <FloatLabel>
          <Textarea v-model="torrentForm.mediainfo" name="mediainfo" class="textarea pre-style" rows="5" @update:model-value="mediainfoUpdated" />
          <label for="mediainfo">{{ t('torrent.mediainfo_autofill') }}</label>
        </FloatLabel>
        <Message v-if="$form.mediainfo?.invalid" severity="error" size="small" variant="simple">
          {{ $form.mediainfo.error?.message }}
        </Message>
      </div>
      <div>
        <div class="line">
          <div class="release-name">
            <FloatLabel>
              <InputText v-model="torrentForm.release_name" size="small" name="release_name" />
              <label for="release_name">{{ t('torrent.release_name') }}</label>
            </FloatLabel>
            <Message v-if="$form.release_name?.invalid" severity="error" size="small" variant="simple">
              {{ $form.release_name.error?.message }}
            </Message>
          </div>
          <div>
            <FloatLabel>
              <InputText v-model="torrentForm.release_group" size="small" name="release_group" />
              <label for="release_group">{{ t('torrent.release_group') }}</label>
            </FloatLabel>
            <Message v-if="$form.release_group?.invalid" severity="error" size="small" variant="simple">
              {{ $form.release_group.error?.message }}
            </Message>
          </div>
        </div>
        <div>
          <FloatLabel>
            <Textarea v-model="torrentForm.description" name="description" class="textarea" autoResize rows="5" />
            <label for="description">{{ t('general.description') }}</label>
          </FloatLabel>
          <Message v-if="$form.description?.invalid" severity="error" size="small" variant="simple">
            {{ $form.release_name.error?.message }}
          </Message>
        </div>
        <div class="line">
          <div>
            <FloatLabel>
              <InputText v-model="torrentForm.container" size="small" name="container" />
              <label for="container">{{ t('torrent.container') }}</label>
            </FloatLabel>
            <Message v-if="$form.container?.invalid" severity="error" size="small" variant="simple">
              {{ $form.container.error?.message }}
            </Message>
          </div>
          <div>
            <FloatLabel v-if="['movie', 'podcast', 'video', 'tv_show', 'collection'].indexOf(titleGroupStore.content_type) >= 0">
              <Select v-model="torrentForm.video_codec" inputId="video_codec" :options="selectableVideoCodecs" class="select" size="small" name="video_codec" />
              <label for="video_codec">{{ t('torrent.video_codec') }}</label>
            </FloatLabel>
            <Message v-if="$form.video_codec?.invalid" severity="error" size="small" variant="simple">
              {{ $form.video_codec.error?.message }}
            </Message>
          </div>
          <div>
            <FloatLabel v-if="['movie', 'tv_show', 'video', 'collection'].indexOf(titleGroupStore.content_type) >= 0">
              <Select
                v-model="torrentForm.video_resolution"
                inputId="video_resolution"
                :options="selectableVideoResolutions"
                class="select"
                size="small"
                name="video_resolution"
              />
              <label for="video_resolution">{{ t('torrent.video_resolution') }}</label>
            </FloatLabel>
            <Message v-if="$form.video_resolution?.invalid" severity="error" size="small" variant="simple">
              {{ $form.video_resolution.error?.message }}
            </Message>
          </div>
          <div>
            <FloatLabel
              v-if="
                ['movie', 'tv_show', 'video', 'podcast', 'music', 'collection'].indexOf(titleGroupStore.content_type) >= 0 ||
                editionGroupStore.additional_information.format === 'audiobook'
              "
            >
              <Select v-model="torrentForm.audio_codec" inputId="audio_codec" :options="selectableAudioCodecs" class="select" size="small" name="audio_codec" />
              <label for="audio_codec">{{ t('torrent.audio_codec') }}</label>
            </FloatLabel>
            <Message v-if="$form.audio_codec?.invalid" severity="error" size="small" variant="simple">
              {{ $form.audio_codec.error?.message }}
            </Message>
          </div>
          <div>
            <FloatLabel
              v-if="
                ['movie', 'tv_show', 'music', 'video', 'podcast', 'collection'].indexOf(titleGroupStore.content_type) >= 0 ||
                editionGroupStore.additional_information.format === 'audiobook'
              "
            >
              <Select
                v-model="torrentForm.audio_bitrate_sampling"
                inputId="audio_coded"
                :options="selectableAudioBitrateSamplings"
                class="select"
                size="small"
                name="audio_bitrate_sampling"
              />
              <label for="audio_bitrate_sampling">{{ t('torrent.audio_bitrate_sampling') }}</label>
            </FloatLabel>
            <Message v-if="$form.audio_bitrate_sampling?.invalid" severity="error" size="small" variant="simple">
              {{ $form.audio_bitrate_sampling.error?.message }}
            </Message>
          </div>
          <div>
            <FloatLabel v-if="['movie', 'tv-show', 'video', 'collection'].indexOf(titleGroupStore.content_type) >= 0">
              <Select
                v-model="torrentForm.audio_channels"
                inputId="audio_channels"
                :options="selectableAudioChannels"
                class="select"
                size="small"
                name="audio_channels"
              />
              <label for="audio_channels">{{ t('torrent.audio_channels') }}</label>
            </FloatLabel>
            <Message v-if="$form.audio_channels?.invalid" severity="error" size="small" variant="simple">
              {{ $form.audio_channels.error?.message }}
            </Message>
          </div>
        </div>
        <div>
          <FloatLabel v-if="titleGroupStore.content_type !== 'music'">
            <MultiSelect
              v-model="torrentForm.languages"
              inputId="languages"
              :options="getLanguages()"
              class="select"
              size="small"
              display="chip"
              filter
              name="languages"
            />
            <label for="languages">{{ t('general.language', 2) }}</label>
          </FloatLabel>
          <Message v-if="$form.languages?.invalid" severity="error" size="small" variant="simple">
            {{ $form.languages.error?.message }}
          </Message>
        </div>
        <div>
          <FloatLabel v-if="torrentForm.video_resolution == 'Other'">
            <InputText
              v-model="torrentForm.video_resolution_other_x"
              inputId="video_resolution_other_x"
              size="small"
              name="video_resolution_other_x"
              type="number"
            />
            <label for="video_resolution_other_x">Resolution X</label>
          </FloatLabel>
          <Message v-if="$form.video_resolution_other_x?.invalid" severity="error" size="small" variant="simple">
            {{ $form.video_resolution_other_x.error?.message }}
          </Message>
        </div>
        <div>
          <FloatLabel v-if="torrentForm.video_resolution == 'Other'">
            <InputText
              v-model="torrentForm.video_resolution_other_y"
              inputId="video_resolution_other_y"
              size="small"
              name="video_resolution_other_y"
              type="number"
            />
            <label for="video_resolution_other_y">Resolution Y</label>
          </FloatLabel>
          <Message v-if="$form.video_resolution_other_y?.invalid" severity="error" size="small" variant="simple">
            {{ $form.video_resolution_other_y.error?.message }}
          </Message>
        </div>
        <FloatLabel>
          <MultiSelect
            v-model="torrentForm.features"
            display="chip"
            :options="getFeatures(titleGroupStore.content_type, editionGroupStore.additional_information.format, editionGroupStore.source)"
            filter
            size="small"
            name="features"
          />
          <label for="features">{{ t('torrent.features') }}</label>
        </FloatLabel>
        <!-- <FloatLabel >
          <InputText v-model="torrentForm.duration" size="small" name="duration" />
          <label for="duration">Duration (total, in seconds)</label>
        </FloatLabel> -->
        <!-- <FloatLabel>
        <InputText v-model="torrentForm.audio_bitrate" size="small" name="audio_bitrate" />
        <label for="audio_codec">Audio bitrate (in kb/s)</label>
      </FloatLabel> -->
        <FormField v-if="!initialTorrent" v-slot="$field" name="torrent_file" :initialValue="torrentForm.torrent_file">
          <FileUpload
            ref="torrentFile"
            accept=".torrent"
            :chooseLabel="t('torrent.torrent_file')"
            :cancel-label="t('general.remove')"
            :showUploadButton="false"
            :fileLimit="1"
            @select="onFileSelect"
            v-bind="$field.props"
          >
            <template #content="{ files }">
              {{ files.length != 0 ? files[0].name : 'Select a file' }}
            </template>
          </FileUpload>
          <Message v-if="$form.torrent_file?.invalid" severity="error" size="small" variant="simple">
            {{ $form.torrent_file.error?.message }}
          </Message>
        </FormField>
        <div class="flex align-items-center upload-as-anonymous">
          <Checkbox v-model="torrentForm.uploaded_as_anonymous" name="anonymous" binary />
          <label for="anonymous" style="margin-left: 5px"> {{ t('torrent.upload_as_anonymous') }}</label>
        </div>
      </div>
      <div class="flex justify-content-center">
        <Button
          label="Validate torrent"
          type="submit"
          icon="pi pi-check"
          size="small"
          class="validate-button"
          :loading="uploadingTorrent"
          :disabled="validateButtonDisabled"
          v-tooltip.top="{ disabled: !validateButtonDisabled, value: t('torrent.complete_edition_group_first') }"
        />
      </div>
    </div>
  </Form>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import FloatLabel from 'primevue/floatlabel'
import InputText from 'primevue/inputtext'
import Textarea from 'primevue/textarea'
import Select from 'primevue/select'
import Button from 'primevue/button'
import Checkbox from 'primevue/checkbox'
import FileUpload, { type FileUploadSelectEvent } from 'primevue/fileupload'
import MultiSelect from 'primevue/multiselect'
import Message from 'primevue/message'
import { FormField, type FormResolverOptions, type FormSubmitEvent } from '@primevue/forms'
import { Form } from '@primevue/forms'
import { getFileInfo } from '@/services/fileinfo/fileinfo.js'
import { useEditionGroupStore } from '@/stores/editionGroup'
import { uploadTorrent, editTorrent, type Torrent, type UploadedTorrent, type EditedTorrent } from '@/services/api/torrentService'
import { useTitleGroupStore } from '@/stores/titleGroup'
import { useI18n } from 'vue-i18n'
import { getFeatures, getLanguages } from '@/services/helpers'
import { nextTick } from 'vue'
import type { VNodeRef } from 'vue'
import _ from 'lodash'
import { showToast } from '@/main'

const formRef = ref<VNodeRef | null>(null)
const torrentFile = ref({ files: [] as unknown[] })
const torrentForm = ref({
  id: 0,
  edition_group_id: 0,
  release_name: '',
  release_group: '',
  mediainfo: '',
  description: '',
  languages: [],
  container: '',
  video_codec: null,
  video_resolution: null,
  video_resolution_other_x: null,
  video_resolution_other_y: null,
  duration: null,
  audio_codec: null,
  audio_bitrate: null,
  subtitle_languages: [],
  features: [],
  audio_channels: null,
  audio_bitrate_sampling: null,
  torrent_file: '',
  uploaded_as_anonymous: false,
})
// TODO : move all the selectable* arrays to an helper function
const selectableVideoCodecs = ['mpeg1', 'mpeg2', 'divX', 'DivX', 'h264', 'h265', 'vc-1', 'vp9', 'BD50', 'UHD100']
const selectableVideoResolutions = ['Other', '480p', '480i', '576p', '576i', '720p', '1080p', '1080i', '1440p', '2160p', '4320p']
const selectableAudioCodecs = ['aac', 'opus', 'mp3', 'mp2', 'aac', 'ac3', 'dts', 'flac', 'pcm', 'true-hd', 'dsd']
const selectableAudioBitrateSamplings = [
  '64',
  '128',
  '192',
  '256',
  '320',
  'APS (VBR)',
  'V2 (VBR)',
  'V1 (VBR)',
  'V0 (VBR)',
  'APX (VBR)',
  'Lossless',
  '24bit Lossless',
  'DSD64',
  'DSD128',
  'DSD256',
  'DSD512',
  'Other',
]
const selectableAudioChannels = ['1.0', '2.0', '2.1', '5.0', '5.1', '7.1']
const uploadingTorrent = ref(false)
const titleGroupStore = ref(useTitleGroupStore())
const editionGroupStore = ref(useEditionGroupStore())

const { t } = useI18n()

const props = defineProps<{
  initialTorrent?: EditedTorrent
}>()
const emit = defineEmits<{
  done: [torrent: Torrent]
}>()

const resolver = ({ values }: FormResolverOptions) => {
  const errors: Partial<Record<keyof UploadedTorrent, { message: string }[]>> = {}

  // if (values.release_name.length < 5) {
  //   errors.release_name = [{ message: t('error.write_more_than_x_chars', [5]) }]
  // }
  // if (values.release_group.length < 2) {
  //   errors.release_group = [{ message: 'Write more than 2 characters' }]
  // }
  // if (values.description == '') {
  //   errors.description = [{ message: 'Write a description' }]
  // }
  if (values.container == '') {
    errors.container = [{ message: t('error.select_container') }]
  }
  if (['movie', 'tv_show'].indexOf(titleGroupStore.value.content_type) >= 0 && !values.video_codec) {
    errors.video_codec = [{ message: t('error.select_codec') }]
  }
  if (['movie', 'tv_show'].indexOf(titleGroupStore.value.content_type) >= 0 && !values.video_resolution) {
    errors.video_resolution = [{ message: t('error.select_resolution') }]
  }
  if (['movie', 'tv_show', 'music'].indexOf(titleGroupStore.value.content_type) >= 0 && !values.audio_codec) {
    errors.audio_codec = [{ message: t('error.select_codec') }]
  }
  if (['movie', 'tv_show', 'music'].indexOf(titleGroupStore.value.content_type) >= 0 && !values.audio_bitrate_sampling) {
    errors.audio_bitrate_sampling = [{ message: t('error.select_bitrate') }]
  }
  if (titleGroupStore.value.content_type !== 'music' && values.languages && values.languages.length === 0) {
    errors.languages = [{ message: t('error.select_at_least_x_language', [1]) }]
  }
  if (!torrentForm.value.torrent_file) {
    errors.torrent_file = [{ message: t('error.select_torrent_file') }]
  }
  if (values.video_resolution === 'Other') {
    if (!values.video_resolution_other_x || isNaN(Number(values.video_resolution_other_x))) {
      errors.video_resolution_other_x = [{ message: "Invalid resolution X" }]
    }
    if (!values.video_resolution_other_y || isNaN(Number(values.video_resolution_other_y))) {
      errors.video_resolution_other_y = [{ message: "Invalid resolution Y" }]
    }
  }

  return {
    errors,
  }
}
const onFormSubmit = ({ valid }: FormSubmitEvent) => {
  if (valid) {
    sendTorrent()
  }
}
const onFileSelect = (event: FileUploadSelectEvent) => {
  if (event.files && event.files.length > 0) {
    // keep a single file
    const file = event.files[event.files.length - 1] as string
    // torrentFile.value.files.clear()
    torrentFile.value.files = [file]
    torrentForm.value.torrent_file = file
  }
}
const mediainfoUpdated = async () => {
  const mediainfoExtractedInfo = getFileInfo(torrentForm.value.mediainfo)
  if (mediainfoExtractedInfo) {
    torrentForm.value.mediainfo = mediainfoExtractedInfo.sanitizedMediainfo
    await nextTick()
    formRef.value?.setFieldValue('mediainfo', mediainfoExtractedInfo.sanitizedMediainfo)
    Object.assign(torrentForm.value, mediainfoExtractedInfo)
    await nextTick()
    try {
      // some fields fail because they are not in the primevueform, but they are in torrentForm
      formRef.value?.setValues(mediainfoExtractedInfo)
    } catch {}
    showToast('', t('torrent.autofilled_mediainfo'), 'success', 4000, true, 'tr')
  }
}
const sendTorrent = () => {
  uploadingTorrent.value = true
  if (props.initialTorrent) {
    torrentForm.value.id = props.initialTorrent.id
    editTorrent(torrentForm.value as EditedTorrent)
      .then((data) => {
        showToast('', t('torrent.torrent_edited_success'), 'success', 3000, true, 'tr')
        emit('done', data)
      })
      .finally(() => {
        uploadingTorrent.value = false
      })
  } else {
    torrentForm.value.edition_group_id = editionGroupStore.value.id
    uploadTorrent(torrentForm.value)
      .then((data) => {
        emit('done', data)
      })
      .finally(() => {
        uploadingTorrent.value = false
      })
  }
}
const validateButtonDisabled = computed(() => {
  return editionGroupStore.value.id === 0 && !props.initialTorrent
})
onMounted(async () => {
  if (props.initialTorrent) {
    Object.assign(torrentForm.value, _.pick(props.initialTorrent, Object.keys(torrentForm.value)))
    await nextTick()
    // some field is apparently undefined, the whole form seems to still get populated though
    // formRef.value?.setValues(torrentForm.value)
    Object.keys(torrentForm.value).forEach((key) => {
      try {
        formRef.value?.setFieldValue(key, torrentForm.value[key as keyof typeof torrentForm.value])
      } catch {
        // some fields fail because they are not in the primevueform, but they are in torrentForm
      }
    })
  }
})
</script>
<style scoped>
#create-torrent {
  padding-top: 20px;
}
.textarea {
  width: 100%;
  height: 10em;
}
.p-floatlabel {
  margin-top: 30px;
}
.mediainfo .p-floatlabel {
  margin-top: 0px;
}
.release-name {
  width: 100%;
  input {
    width: 100%;
  }
}
.select {
  min-width: 200px;
}
.upload-as-anonymous {
  margin-top: 20px;
}
.validate-button {
  margin-top: 20px;
}
</style>
<style>
#create-torrent {
  .p-fileupload {
    max-width: 300px;
    margin-top: 15px;
  }
}
</style>
