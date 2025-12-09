<template>
  <Form
    class="form"
    v-slot="$form"
    :initialValues="titleGroupForm"
    :resolver
    @submit="sendTitleGroup"
    validateOnSubmit
    :validateOnValueUpdate="false"
    validateOnBlur
    ref="formRef"
  >
    <FloatLabel style="margin-top: 0">
      <Select
        v-model="titleGroupForm.content_type"
        @update:model-value="(content_type) => (titleGroupStore.content_type = content_type)"
        inputId="content_type"
        name="content_type"
        :options="getSelectableContentTypes()"
        class="select"
        size="small"
      >
        <template #option="slotProps">
          <span>{{ t(`title_group.content_type.${slotProps.option}`) }}</span>
        </template>
        <template #value="slotProps">
          <span v-if="slotProps.value">
            {{ t(`title_group.content_type.${slotProps.value}`) }}
          </span>
        </template>
      </Select>
      <label for="content_type">{{ t('title_group.content_type.content_type') }}</label>
    </FloatLabel>
    <Message v-if="$form.content_type?.invalid" severity="error" size="small" variant="simple">
      {{ $form.content_type.error?.message }}
    </Message>
    <div class="name">
      <FloatLabel>
        <InputText size="small" v-model="titleGroupForm.name" name="name" class="name-input" />
        <label for="name">{{ t('general.name') }}</label>
      </FloatLabel>
      <Message v-if="$form.name?.invalid" severity="error" size="small" variant="simple">
        {{ $form.name.error?.message }}
      </Message>
    </div>
    <div v-if="titleGroupForm.content_type !== null">
      <div class="line" v-if="titleGroupForm.content_type == 'software'">
        <FloatLabel>
          <InputNumber size="small" v-model="titleGroupForm.master_group_id" name="master_group_id" :format="false" />
          <label for="master_group_id">{{ t('master_group.master_group_id') }}</label>
        </FloatLabel>
      </div>
      <div class="line">
        <div v-if="selectableCategories[titleGroupForm.content_type].length !== 0">
          <FloatLabel>
            <Select
              v-model="titleGroupForm.category"
              inputId="category"
              :options="selectableCategories[titleGroupForm.content_type]"
              size="small"
              name="category"
              class="select"
            />
            <label for="category">{{ t('general.category') }}</label>
          </FloatLabel>
          <Message v-if="$form.category?.invalid" severity="error" size="small" variant="simple">
            {{ $form.category.error?.message }}
          </Message>
        </div>
      </div>
      <div class="tags" style="width: 100%" v-if="!editMode">
        <TitleGroupTagsInput v-model="titleGroupForm.tags" @keydown.enter.prevent />
        <Message v-if="$form.tags?.invalid" severity="error" size="small" variant="simple">
          {{ $form.tags.error?.message }}
        </Message>
      </div>
      <div>
        <FloatLabel>
          <Textarea v-model="titleGroupForm.description" name="description" class="description" autoResize rows="5" />
          <label for="description">{{ t('general.description') }}</label>
        </FloatLabel>
        <Message v-if="$form.description?.invalid" severity="error" size="small" variant="simple">
          {{ $form.description.error?.message }}
        </Message>
      </div>
      <div class="line">
        <div v-if="titleGroupForm.content_type == 'software'">
          <FloatLabel>
            <Select v-model="titleGroupForm.platform" inputId="platform" :options="getPlatforms()" class="select" size="small" name="platform" filter />
            <label for="platform">{{ t('title_group.platform') }}</label>
          </FloatLabel>
          <Message v-if="$form.platform?.invalid" severity="error" size="small" variant="simple">
            {{ $form.platform.error?.message }}
          </Message>
        </div>
        <div>
          <FloatLabel>
            <Select
              v-model="titleGroupForm.original_language"
              inputId="original_language"
              :options="getLanguages()"
              class="select"
              size="small"
              name="original_language"
              filter
            />
            <label for="original_language">{{ t('general.original_language') }}</label>
          </FloatLabel>
          <Message v-if="$form.original_language?.invalid" severity="error" size="small" variant="simple">
            {{ $form.original_language.error?.message }}
          </Message>
        </div>
        <div>
          <FloatLabel>
            <Select
              v-model="titleGroupForm.country_from"
              inputId="country_from"
              :options="selectableCountries"
              class="select"
              size="small"
              name="country_from"
              filter
            />
            <label for="country_from">{{ t('general.country') }}</label>
          </FloatLabel>
          <Message v-if="$form.country_from?.invalid" severity="error" size="small" variant="simple">
            {{ $form.country_from.error?.message }}
          </Message>
        </div>
      </div>
      <div class="original-release-date">
        <label for="original_release_date" class="block">{{ t('title_group.original_release_date') }}</label>
        <DatePicker
          :manual-input="false"
          v-model="original_release_date"
          showIcon
          iconDisplay="input"
          inputId="original_release_date"
          size="small"
          dateFormat="yy-mm-dd"
          name="original_release_date"
        />
        <Message v-if="$form.original_release_date?.invalid" severity="error" size="small" variant="simple">
          {{ $form.original_release_date.error?.message }}
        </Message>
      </div>
      <div v-if="!props.editMode">
        <label>{{ t('artist.artist', 2) }}</label>
        <EditAffiliatedArtists
          ref="editAffiliatedArtistsRef"
          :contentType="titleGroupForm.content_type"
          :initial-artists-affiliations="initialArtistsAffiliation"
        />
      </div>
      <div class="covers input-list">
        <label>{{ t('general.cover', 2) }}</label>
        <div v-for="(_link, index) in titleGroupForm.covers" :key="index">
          <InputText size="small" v-model="titleGroupForm.covers[index]" />
          <Button v-if="index == 0" @click="addCover" icon="pi pi-plus" size="small" />
          <Button v-if="index != 0 || titleGroupForm.covers.length > 1" @click="removeCover(index)" icon="pi pi-minus" size="small" />
          <Message v-if="($form.covers as unknown as FormFieldState[])?.[index]?.invalid" severity="error" size="small" variant="simple">
            {{ ($form.covers as unknown as FormFieldState[])[index].error?.message }}
          </Message>
        </div>
      </div>
      <div class="screenshots input-list" v-if="titleGroupForm.content_type == 'software'">
        <label>{{ t('general.screenshots') }}</label>
        <div v-for="(_link, index) in titleGroupForm.screenshots" :key="index">
          <InputText size="small" v-model="titleGroupForm.screenshots[index]" />
          <Button v-if="index == 0" @click="addScreenshot" icon="pi pi-plus" size="small" />
          <Button v-if="index != 0 || titleGroupForm.screenshots.length > 1" @click="removeScreenshot(index)" icon="pi pi-minus" size="small" />
          <Message v-if="($form.screenshots as unknown as FormFieldState[])?.[index]?.invalid" severity="error" size="small" variant="simple">
            {{ ($form.screenshots as unknown as FormFieldState[])[index].error?.message }}
          </Message>
        </div>
      </div>
      <div class="external-links input-list">
        <label>{{ t('general.external_link', 2) }}</label>
        <div v-for="(_link, index) in titleGroupForm.external_links" :key="index">
          <InputText size="small" v-model="titleGroupForm.external_links[index]" :name="`external_links[${index}]`" />
          <Button v-if="index == 0" @click="addLink" icon="pi pi-plus" size="small" />
          <Button v-if="index != 0 || titleGroupForm.external_links.length > 1" @click="removeLink(index)" icon="pi pi-minus" size="small" />
          <Message v-if="($form.external_links as unknown as FormFieldState[])?.[index]?.invalid" severity="error" size="small" variant="simple">
            {{ ($form.external_links as unknown as FormFieldState[])[index].error?.message }}
          </Message>
        </div>
      </div>
    </div>
    <div class="flex justify-content-center">
      <Button label="Validate title" icon="pi pi-check" type="submit" size="small" class="validate-button" :loading="sendingTitleGroup" />
    </div>
  </Form>
</template>
<script setup lang="ts">
import { ref, computed } from 'vue'
import { Form, type FormFieldState, type FormResolverOptions, type FormSubmitEvent } from '@primevue/forms'
import FloatLabel from 'primevue/floatlabel'
import InputText from 'primevue/inputtext'
import Textarea from 'primevue/textarea'
import Select from 'primevue/select'
import Button from 'primevue/button'
import DatePicker from 'primevue/datepicker'
import Message from 'primevue/message'
import { InputNumber } from 'primevue'
import { useI18n } from 'vue-i18n'
import { getSelectableContentTypes, getLanguages, getPlatforms, isValidUrl } from '@/services/helpers'
import { useTitleGroupStore } from '@/stores/titleGroup'
import type { VNodeRef } from 'vue'
import EditAffiliatedArtists from '../artist/EditAffiliatedArtists.vue'
import { onMounted } from 'vue'
import { nextTick } from 'vue'
import _ from 'lodash'
import { showToast } from '@/main'
import type { UserCreatedTitleGroupForm } from './CreateOrSelectTitleGroup.vue'
import TitleGroupTagsInput from '../TitleGroupTagsInput.vue'
import {
  createTitleGroup,
  editTitleGroup,
  type AffiliatedArtistHierarchy,
  type ContentType,
  type EditedTitleGroup,
  type TitleGroup,
  type TitleGroupCategory,
  type UserCreatedAffiliatedArtist,
  type UserCreatedTitleGroup,
} from '@/services/api-schema'

const props = defineProps<{
  initialTitleGroup?: EditedTitleGroup | UserCreatedTitleGroupForm
  initialArtistsAffiliation?: AffiliatedArtistHierarchy[] | UserCreatedAffiliatedArtist[]
  editMode?: boolean
}>()
const titleGroupStore = ref(useTitleGroupStore())

const sendingTitleGroup = ref(false)

const titleGroupForm = ref({
  id: 0,
  name: '',
  name_aliases: [],
  tagline: null,
  description: '',
  original_language: '',
  original_release_date: '',
  covers: [''],
  screenshots: [''],
  external_links: [''],
  category: null,
  country_from: '',
  affiliated_artists: [],
  tags: [] as string[],
  master_group_id: null,
  platform: null,
  embedded_links: {},
  content_type: null as ContentType | null,
})
const formRef = ref<VNodeRef | null>(null)
const editAffiliatedArtistsRef = ref<VNodeRef | null>(null)

const original_release_date = computed({
  get() {
    const isValidDateStr = !isNaN(Date.parse(titleGroupForm.value.original_release_date ?? ''))
    return isValidDateStr ? new Date(titleGroupForm.value.original_release_date ?? '') : null
  },
  set(newValue) {
    titleGroupForm.value.original_release_date = newValue?.toISOString() ?? ''
  },
})

const selectableCountries = ['France', 'UK', 'USA']
const selectableCategories: Record<ContentType, TitleGroupCategory[]> = {
  book: ['Illustrated', 'Periodical', 'Book', 'Article', 'Manual'],
  music: ['Single', 'Album', 'Ep'],
  podcast: [],
  movie: ['FeatureFilm', 'ShortFilm'],
  video: [],
  software: ['Program', 'Game'],
  collection: ['Other'],
  tv_show: [],
}

const { t } = useI18n()

const emit = defineEmits<{
  done: [titleGroup: TitleGroup]
}>()

// type FormErrors = {
//   [key in keyof UserCreatedTitleGroup]: UserCreatedTitleGroup[key] extends Array<unknown>
//     ? { message: string }[][]
//     : { message: string }[]
// }
const resolver = ({ values }: FormResolverOptions) => {
  const errors: Partial<Record<keyof UserCreatedTitleGroup, { message: string }[]>> = {}

  if (titleGroupForm.value.content_type === null) {
    errors.content_type = [{ message: t('error.select_content_type') }]
    return { errors }
  }
  if (values.name.length < 1) {
    errors.name = [{ message: t('error.write_more_than_x_chars', [0]) }]
  }
  if (!titleGroupForm.value.category && selectableCategories[titleGroupForm.value.content_type]) {
    errors.category = [{ message: t('error.select_category') }]
  }
  //TODO config: the minimum amount of tags required should be taken from the global config file
  if (titleGroupForm.value.tags.length === 0 && !props.editMode) {
    // somehow isn't displayed in the form and doesn't prevent submitting
    errors.tags = [{ message: t('error.enter_at_least_x_tags', [1]) }]
  }
  if (values.description.length < 10) {
    errors.description = [{ message: t('error.write_more_than_x_chars', [10]) }]
  }
  if (values.platform == '') {
    errors.platform = [{ message: t('error.select_platform') }]
  }
  if (titleGroupForm.value.content_type !== 'music' && values.original_language == '') {
    errors.original_language = [{ message: t('error.select_language') }]
  }
  if (values.country_from == '') {
    errors.country_from = [{ message: t('error.select_country') }]
  }
  if (values.original_release_date === null || values.original_release_date == '') {
    errors.original_release_date = [{ message: t('error.select_date') }]
  }
  // affiliated_artists_names.value.forEach((artist_name: string, index: number) => {
  //   if (artist_name === '') {
  //     if (!('affiliated_artists' in errors)) {
  //       errors.affiliated_artists = []
  //     }
  //     errors.affiliated_artists![index] = { message: t('error.invalid_name') }
  //   }
  // })
  // titleGroupForm.value.affiliated_artists.forEach((artist: UserCreatedAffiliatedArtist, index: number) => {
  //   if (artist.roles.length === 0) {
  //     if (!('affiliated_artists' in errors)) {
  //       errors.affiliated_artists = []
  //     }
  //     errors.affiliated_artists![index] = {
  //       message: t('error.artist_must_have_at_least_one_role'),
  //     }
  //   }
  // })
  values.external_links.forEach((link: string, index: number) => {
    if (!isValidUrl(link)) {
      if (!('external_links' in errors)) {
        errors.external_links = []
      }
      errors.external_links![index] = { message: t('error.invalid_url') }
    }
  })
  //TODO: should be values.covers, but somehow it is undefined
  titleGroupForm.value.covers.forEach((link: string, index: number) => {
    if (!isValidUrl(link)) {
      if (!('covers' in errors)) {
        errors.covers = []
      }
      errors.covers![index] = { message: t('error.invalid_url') }
    }
  })
  if (values.screenshots) {
    values.screenshots.forEach((link: string, index: number) => {
      if (!isValidUrl(link)) {
        if (!('screenshots' in errors)) {
          errors.screenshots = []
        }
        errors.screenshots![index] = { message: t('error.invalid_url') }
      }
    })
  }
  return {
    errors,
  }
}

const sendTitleGroup = async ({ valid }: FormSubmitEvent) => {
  if (!valid) {
    return
  }
  sendingTitleGroup.value = true
  titleGroupForm.value.screenshots = titleGroupForm.value.screenshots.filter((screenshot) => screenshot.trim() !== '')
  titleGroupForm.value.external_links = titleGroupForm.value.external_links.filter((link) => link.trim() !== '')
  if (props.editMode && props.initialTitleGroup) {
    titleGroupForm.value.id = props.initialTitleGroup.id
    editTitleGroup(titleGroupForm.value as EditedTitleGroup)
      .then((data) => {
        showToast('', t('title_group.title_group_edited_success'), 'success', 3000, true, 'tr')
        emit('done', data)
      })
      .finally(() => {
        sendingTitleGroup.value = false
      })
  } else {
    try {
      // create artists that need to be created
      await editAffiliatedArtistsRef.value.createInexistingArtists()
    } catch {
      sendingTitleGroup.value = false
      return
    }
    titleGroupForm.value.affiliated_artists = editAffiliatedArtistsRef.value.affiliated_artists
    const formattedTitleGroupForm = JSON.parse(JSON.stringify(titleGroupForm.value))
    createTitleGroup(formattedTitleGroupForm)
      .then((data) => {
        emit('done', data)
      })
      .finally(() => {
        sendingTitleGroup.value = false
      })
  }
}

const addLink = () => {
  titleGroupForm.value.external_links.push('')
}
const removeLink = (index: number) => {
  titleGroupForm.value.external_links.splice(index, 1)
}
const addCover = () => {
  titleGroupForm.value.covers.push('')
}
const removeCover = (index: number) => {
  titleGroupForm.value.covers.splice(index, 1)
}
const addScreenshot = () => {
  titleGroupForm.value.screenshots.push('')
}
const removeScreenshot = (index: number) => {
  titleGroupForm.value.screenshots.splice(index, 1)
}

// const updateTitleGroupForm = (form: Partial<UserCreatedTitleGroupForm>) => {
//   if (form.affiliated_artists && form.affiliated_artists.length === 0) {
//     form.affiliated_artists = titleGroupForm.value.affiliated_artists
//   }
//   titleGroupForm.value = {
//     ...titleGroupForm.value,
//     ...form,
//   }
//   if (titleGroupForm.value.tags.length > 0) {
//     tagsString.value = titleGroupForm.value.tags.join(',')
//   }
//   try {
//     // some fields fail because they are not in the primevueform, but they are in titleGroupForm
//     formRef.value?.setValues(titleGroupForm.value)
//   } catch {}
// }

onMounted(async () => {
  if (props.initialTitleGroup) {
    Object.assign(titleGroupForm.value, _.pick(props.initialTitleGroup, Object.keys(titleGroupForm.value)))
    await nextTick()
    Object.keys(titleGroupForm.value).forEach((key) => {
      try {
        formRef.value?.setFieldValue(key, titleGroupForm.value[key as keyof typeof titleGroupForm.value])
      } catch {
        // some fields fail because they are not in the primevueform, but they are in titleGroupForm
      }
    })
  }
})
// watch(
//   () => props.initialTitleGroup,
//   (newValue) => {
//     if (newValue !== null) {
//       updateTitleGroupForm(newValue)
//     }
//   },
//   { immediate: true },
// )
</script>
<style scoped>
.description {
  width: 100%;
  height: 10em;
}
.name {
  width: 50%;
  .name-input {
    width: 100%;
  }
}
.select {
  width: 200px;
}
.original-release-date {
  margin-top: 30px;
  margin-bottom: 20px;
}
.input-list {
  margin-bottom: 15px;
}
.input-list input {
  &:not(.artist) {
    width: 400px;
  }
}
.validate-button {
  margin-top: 20px;
}
</style>
