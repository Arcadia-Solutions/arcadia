<template>
  <ContentContainer>
    <div id="torrent-search-inputs">
      <div class="line">
        <FloatLabel>
          <InputText class="title-group-name" size="small" v-model="searchForm.title_group_name" name="title_group_name" />
          <label for="title_group_name">{{ t('general.search_terms') }}</label>
        </FloatLabel>
      </div>
      <div class="line">
        <FloatLabel>
          <InputNumber size="small" v-model="searchForm.torrent_snatched_by_id" name="snatched_by_user_id" />
          <label for="snatched_by_user_id">{{ t('torrent.snatched_by_user_id') }}</label>
        </FloatLabel>
        <FloatLabel>
          <InputNumber size="small" v-model="searchForm.torrent_created_by_id" name="uploaded_by_user_id" />
          <label for="uploaded_by_user_id">{{ t('torrent.uploaded_by_user_id') }}</label>
        </FloatLabel>
      </div>
      <!-- <FloatLabel>
        <InputText class="tags" size="small" v-model="searchForm.tags" name="tags" />
        <label for="tags">{{ t('general.tags_comma_separated') }}</label>
      </FloatLabel> -->
      <div class="line" style="margin-top: 40px">
        <div class="dropdown">
          <label for="sortByDropdown">{{ t('general.sort_by') }}</label>
          <Dropdown
            v-model="searchForm.order_by_column"
            :options="sortByOptions"
            optionLabel="label"
            optionValue="value"
            size="small"
            input-id="sortByDropdown"
          />
        </div>
        <div class="dropdown">
          <label for="orderDropdown">{{ t('general.order_by') }}</label>
          <Dropdown
            v-model="searchForm.order_by_direction"
            :options="getOrderByDirectionOptions(t)"
            optionLabel="label"
            optionValue="value"
            size="small"
            input-id="orderDropdown"
          />
        </div>
      </div>
      <div class="line">
        <div class="dropdown">
          <label>{{ t('torrent.staff_checked') }}</label>
          <Dropdown
            v-model="searchForm.torrent_staff_checked"
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
            v-model="searchForm.torrent_reported"
            :options="staffOptionChoices"
            optionLabel="label"
            optionValue="value"
            :placeholder="t('general.both')"
            size="small"
          />
        </div>
      </div>
      <div class="flex justify-content-center" style="margin-top: 15px">
        <Button :loading :label="t('general.search')" @click="search" />
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
import { Dropdown, InputNumber } from 'primevue'
import { onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { watch } from 'vue'
import { TorrentSearchOrderByColumn, type TorrentSearch } from '@/services/api-schema'
import { getOrderByDirectionOptions } from '@/services/helpers'

const { t } = useI18n()
const router = useRouter()

const props = defineProps<{
  loading: boolean
  initialForm: TorrentSearch
}>()

const sortByOptions = ref([
  { label: t('torrent.created_at'), value: TorrentSearchOrderByColumn.TorrentCreatedAt },
  { label: t('torrent.size'), value: TorrentSearchOrderByColumn.TorrentSize },
  { label: t('title_group.original_release_date'), value: TorrentSearchOrderByColumn.TitleGroupOriginalReleaseDate },
  { label: t('torrent.snatched'), value: TorrentSearchOrderByColumn.TorrentSnatched },
  { label: t('torrent.seeder', 2), value: TorrentSearchOrderByColumn.TorrentSeeders },
  { label: t('torrent.leecher', 2), value: TorrentSearchOrderByColumn.TorrentLeechers },
])
const staffOptionChoices = ref([
  { label: t('general.yes'), value: true },
  { label: t('general.no'), value: false },
  { label: t('general.both'), value: null },
])

const searchForm = ref<TorrentSearch>({
  title_group_name: '',
  title_group_include_empty_groups: false,
  torrent_created_by_id: null,
  torrent_snatched_by_id: null,
  torrent_staff_checked: false,
  torrent_reported: null,
  page: 1,
  page_size: 4,
  order_by_column: 'torrent_created_at',
  order_by_direction: 'desc',
})
const changePage = (page: number) => {
  searchForm.value.page = page
  search()
}
const search = () => {
  router.push({
    query: Object.fromEntries(Object.entries(searchForm.value).map(([k, v]) => [k, typeof v === 'boolean' ? String(v) : v])),
  })
  // a search will be triggered by the query changes through a watcher
}
defineExpose({
  searchForm,
  changePage,
})

onMounted(async () => {
  searchForm.value = props.initialForm
})

watch(
  () => searchForm.value,
  (newVal, oldVal) => {
    // ignore if only `page` changed
    if (newVal.page === oldVal.page) {
      searchForm.value.page = 1
    }
  },
  { deep: true },
)
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
