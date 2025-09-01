<template>
  <template v-if="action === 'select'">
    <FloatLabel class="input">
      <Select
        v-model="titleGroupForm.content_type"
        @update:model-value="(content_type) => (titleGroupStore.content_type = content_type)"
        inputId="content_type"
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
    <div class="external-db-inputs-wrapper input" v-if="titleGroupForm.content_type !== null">
      <div class="external-db-inputs" v-if="titleGroupForm.content_type == 'movie'">
        <ExternalDBSearchBar inputPlaceholder="TMDB url" database="tmdb" @dataFound="externalDBDataFound" />
        <!-- or
            <ExternalDBSearchBar inputPlaceholder="IMDB id" database="imdb/movie" @dataFound="externalDBDataFound" /> -->
      </div>
      <!-- <div class="external-db-inputs" v-if="content_type == 'tv_show'">
            <ExternalDBSearchBar inputPlaceholder="TVDB id" database="tvdb" @dataFound="externalDBDataFound" />
            or
            <ExternalDBSearchBar inputPlaceholder="TMDB id" database="tmdb/tv" @dataFound="externalDBDataFound" />
            or
            <ExternalDBSearchBar inputPlaceholder="IMDB id" database="imdb/tv" @dataFound="externalDBDataFound" />
          </div> -->
      <div class="external-db-inputs" v-if="titleGroupForm.content_type == 'music'">
        <ExternalDBSearchBar inputPlaceholder="Musicbrainz url" database="musicbrainz" @dataFound="externalDBDataFound" />
        <!-- or -->
        <!-- <ExternalDBSearchBar inputPlaceholder="Discogs id" database="discogs" @dataFound="externalDBDataFound" /> -->
      </div>
      <div class="external-db-inputs" v-if="titleGroupForm.content_type == 'book'">
        <ExternalDBSearchBar inputPlaceholder="isbn" database="isbn" @dataFound="externalDBDataFound" />
        <ExternalDBSearchBar inputPlaceholder="Comic-Vine url" database="comic_vine" @dataFound="externalDBDataFound" />
      </div>
    </div>
    <TitleGroupSearchBar
      class="name-input input"
      :placeholder="t('general.name')"
      :clearInputOnSelect="false"
      @titleGroupSelected="titleGroupSelected"
      @createNew="createNew"
      :createOption="true"
      :contentType="titleGroupForm.content_type"
      v-model="titleGroupForm.name"
    />
  </template>
  <CreateOrEditTitleGroup v-else-if="action === 'create'" @done="titleGroupCreated" :initial-title-group="titleGroupForm" />
</template>

<script setup lang="ts">
import type { ContentType, TitleGroup, TitleGroupLite, UserCreatedEditionGroup, UserCreatedTitleGroup } from '@/services/api/torrentService'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import CreateOrEditTitleGroup from './CreateOrEditTitleGroup.vue'
import TitleGroupSearchBar from './TitleGroupSearchBar.vue'
import ExternalDBSearchBar from './ExternalDBSearchBar.vue'
import { Select, FloatLabel } from 'primevue'
import { useTitleGroupStore } from '@/stores/titleGroup'
import { getSelectableContentTypes } from '@/services/helpers'
import type { ExternalDBData } from '@/services/api/externalDatabasesService'
import type { Language } from '@/services/api/torrentService'

const { t } = useI18n()
const titleGroupStore = useTitleGroupStore()
const emit = defineEmits<{
  done: [titleGroup: TitleGroup | TitleGroupLite]
  editionGroupDataFound: [editionGroup: UserCreatedEditionGroup]
  siwtchedToCreate: []
}>()

const action = ref<'select' | 'create'>('select')

// this type is used to allow more flexibility in certain fields in the frontend forms
export type UserCreatedTitleGroupForm = Omit<UserCreatedTitleGroup, 'content_type' | 'original_language'> & {
  content_type: ContentType | null
  original_language: Language | null
  id: number
}
const titleGroupForm = ref<UserCreatedTitleGroupForm>({
  id: 0,
  name: '',
  description: '',
  original_language: null,
  original_release_date: '',
  covers: [''],
  screenshots: [''],
  external_links: [''],
  category: null,
  country_from: '',
  name_aliases: [],
  affiliated_artists: [],
  tags: [],
  master_group_id: null,
  platform: null,
  embedded_links: {},
  content_type: null,
})

const titleGroupSelected = (titleGroup: TitleGroupLite) => {
  emit('done', titleGroup)
}
const titleGroupCreated = async (titleGroup: TitleGroup | TitleGroupLite) => {
  titleGroupStore.id = titleGroup.id
  titleGroupStore.content_type = titleGroup.content_type
  emit('done', titleGroup)
}
const createNew = () => {
  action.value = 'create'
  emit('siwtchedToCreate')
}
const updateTitleGroupForm = (form: Partial<UserCreatedTitleGroupForm>) => {
  if (form.affiliated_artists && form.affiliated_artists.length === 0) {
    form.affiliated_artists = titleGroupForm.value.affiliated_artists
  }
  titleGroupForm.value = {
    ...titleGroupForm.value,
    ...form,
  }
  createNew()
}
const externalDBDataFound = (data: ExternalDBData) => {
  if (data.title_group) {
    updateTitleGroupForm(data.title_group)
  }
  if (data.edition_group) {
    emit('editionGroupDataFound', data.edition_group)
  }
}
</script>
<style scoped>
.input {
  margin-bottom: 15px;
}
.name-input {
  width: 40%;
}
.select {
  width: 200px;
}
.external-db-inputs {
  display: flex;
  align-items: center;
  .external-db-input {
    margin-right: 10px;
  }
}
</style>
