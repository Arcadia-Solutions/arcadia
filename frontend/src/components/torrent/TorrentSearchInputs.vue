<template>
  <ContentContainer>
    <div id="torrent-search-inputs">
      <div class="line">
        <FloatLabel>
          <InputText class="title-group-name" size="small" v-model="searchForm.title_group.name" name="title_group_name" />
          <label for="title_group_name">{{ t('general.search_terms') }}</label>
        </FloatLabel>
      </div>
      <!-- <FloatLabel>
        <InputText class="tags" size="small" v-model="searchForm.tags" name="tags" />
        <label for="tags">{{ t('general.tags_comma_separated') }}</label>
      </FloatLabel> -->
      <div class="line">
        <div class="dropdown">
          <label for="sortByDropdown">{{ t('general.sort_by') }}</label>
          <Dropdown v-model="searchForm.sort_by" :options="sortByOptions" optionLabel="label" optionValue="value" size="small" input-id="sortByDropdown" />
        </div>
        <div class="dropdown">
          <label for="orderDropdown">{{ t('general.order_by') }}</label>
          <Dropdown v-model="searchForm.order" :options="orderOptions" optionLabel="label" optionValue="value" size="small" input-id="orderDropdown" />
        </div>
      </div>
      <div class="staff-options line" v-if="showStaffOptions">
        <div class="dropdown">
          <label>{{ t('torrent.staff_checked') }}</label>
          <Dropdown
            v-model="searchForm.torrent.staff_checked"
            :options="staffOptionChoices"
            optionLabel="label"
            optionValue="value"
            :placeholder="t('general.both')"
            size="small"
          />
        </div>
        <div class="dropdown">
          <label>{{ t('general.reported') }}</label>
          <Dropdown
            v-model="searchForm.torrent.reported"
            :options="staffOptionChoices"
            optionLabel="label"
            optionValue="value"
            :placeholder="t('general.both')"
            size="small"
          />
        </div>
      </div>
      <div class="flex justify-content-center" style="margin-top: 15px">
        <Button :loading :label="t('general.search')" @click="emit('search', searchForm)" />
      </div>
    </div>
  </ContentContainer>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import ContentContainer from '../ContentContainer.vue'
import InputText from 'primevue/inputtext'
import FloatLabel from 'primevue/floatlabel'
import Button from 'primevue/button'
import { Dropdown } from 'primevue'
import type { TorrentSearch } from '@/services/api/torrentService'
import { onMounted } from 'vue'

const { t } = useI18n()

const props = defineProps<{
  loading: boolean
  initialForm: TorrentSearch
  showStaffOptions: boolean
}>()

const emit = defineEmits<{
  search: [form: TorrentSearch]
}>()

const sortByOptions = ref([
  { label: t('torrent.created_at'), value: 'torrent_created_at' },
  { label: t('torrent.size'), value: 'torrent_size' },
  { label: t('title_group.original_release_date'), value: 'title_group_original_release_date' },
  { label: t('torrent.snatched_at'), value: 'torrent_snatched_at' },
])
const orderOptions = [
  { label: t('general.ascending'), value: 'asc' },
  { label: t('general.descending'), value: 'desc' },
]
const staffOptionChoices = ref([
  { label: t('general.yes'), value: true },
  { label: t('general.no'), value: false },
  { label: t('general.both'), value: null },
])

const searchForm = ref<TorrentSearch>({
  title_group: { name: '', include_empty_groups: false },
  torrent: {},
  page: 1,
  page_size: 4,
  sort_by: 'torrent_created_at',
  order: 'desc',
})
const changePage = (to: 'previous' | 'next') => {
  searchForm.value.page = to === 'previous' ? searchForm.value.page - 1 : searchForm.value.page + 1
  emit('search', searchForm.value)
}
defineExpose({
  searchForm,
  changePage,
})

onMounted(async () => {
  searchForm.value = props.initialForm
})
</script>

<style>
.title-group-name {
  width: 40em;
}
.tags {
  width: 30%;
}
.line {
  margin-bottom: 15px;
}
.dropdown {
  display: flex;
  align-items: center;
  margin-right: 10px;
  label {
    margin-right: 5px;
  }
}
.staff-options {
  display: flex;
}
</style>
