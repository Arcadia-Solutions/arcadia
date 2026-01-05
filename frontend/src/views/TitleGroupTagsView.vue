<template>
  <ContentContainer class="search-form">
    <div class="line">
      <FloatLabel>
        <InputText v-model="searchForm.name" size="small" />
        <label for="name">{{ t('general.name') }}</label>
      </FloatLabel>
      <FloatLabel>
        <label for="sortByDropdown">{{ t('general.sort_by') }}</label>
        <Dropdown
          v-model="searchForm.order_by_column"
          :options="sortByOptions"
          optionLabel="label"
          optionValue="value"
          size="small"
          input-id="sortByDropdown"
        />
      </FloatLabel>
      <FloatLabel>
        <label for="orderByDropdown">{{ t('general.order_by') }}</label>
        <Dropdown
          v-model="searchForm.order_by_direction"
          :options="getOrderByDirectionOptions(t)"
          optionLabel="label"
          optionValue="value"
          size="small"
          input-id="orderByDropdown"
        />
      </FloatLabel>
    </div>
    <div class="wrapper-center">
      <Button :label="t('general.search')" size="small" @click="updateUrl" :loading />
    </div>
  </ContentContainer>
  <PaginatedResults
    v-if="searchResults.length > 0"
    :totalItems="totalResults"
    :pageSize="searchForm.page_size"
    :initialPage="searchForm.page"
    :totalPages="totalPages"
  >
    <DataTable :value="searchResults" size="small">
      <Column :header="t('general.name')">
        <template #body="slotProps">
          {{ slotProps.data.name }}
        </template>
      </Column>
      <Column :header="t('general.created_by')">
        <template #body="slotProps">
          <UsernameEnriched :user="slotProps.data.created_by" />
        </template>
      </Column>
      <Column :header="t('general.uses')">
        <template #body="slotProps">
          {{ slotProps.data.uses }}
        </template>
      </Column>
      <Column :header="t('general.synonym', 2)">
        <template #body="slotProps">
          <span v-if="slotProps.data.synonyms.length > 0">
            {{ slotProps.data.synonyms.join(', ') }}
          </span>
        </template>
      </Column>
      <Column :header="t('general.created_at')">
        <template #body="slotProps">
          <span>
            {{ timeAgo(slotProps.data.created_at) }}
          </span>
        </template>
      </Column>
      <Column :header="t('general.action', 2)" class="actions">
        <template #body="slotProps">
          <i
            class="pi pi-pen-to-square cursor-pointer"
            v-if="userStore.permissions.includes('edit_title_group_tag')"
            v-tooltip.top="t('general.edit')"
            @click="editTag(slotProps.data)"
          />
          <i
            class="pi pi-trash cursor-pointer"
            v-if="userStore.permissions.includes('delete_title_group_tag')"
            v-tooltip.top="t('general.delete')"
            @click="deleteTag(slotProps.data)"
          />
        </template>
      </Column>
    </DataTable>
  </PaginatedResults>
  <Dialog closeOnEscape modal :header="t('general.edit')" v-model:visible="editTagDialogVisible">
    <EditTitleGroupTagDialog v-if="tagBeingEdited" :initialTag="tagBeingEdited" @done="tagEdited" />
  </Dialog>
  <Dialog closeOnEscape modal :header="t('general.delete')" v-model:visible="deleteTagDialogVisible">
    <DeleteTitleGroupTagDialog v-if="tagBeingDeleted" :tag="tagBeingDeleted" @deleted="tagDeleted" />
  </Dialog>
</template>

<script setup lang="ts">
import { onMounted, ref, computed, watch, nextTick } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter, useRoute } from 'vue-router'
import { Button, FloatLabel, InputText, DataTable, Column, Dropdown, Dialog } from 'primevue'
import ContentContainer from '@/components/ContentContainer.vue'
import PaginatedResults from '@/components/PaginatedResults.vue'
import { timeAgo, getOrderByDirectionOptions } from '@/services/helpers'
import { useUserStore } from '@/stores/user'
import EditTitleGroupTagDialog from '@/components/title_group_tag/EditTitleGroupTagDialog.vue'
import DeleteTitleGroupTagDialog from '@/components/title_group_tag/DeleteTitleGroupTagDialog.vue'
import UsernameEnriched from '@/components/user/UsernameEnriched.vue'
import { searchTitleGroupTags, type EditedTitleGroupTag, type SearchTitleGroupTagsQuery, type TitleGroupTagEnriched } from '@/services/api-schema'

const { t } = useI18n()
const router = useRouter()
const route = useRoute()
const userStore = useUserStore()

const sortByOptions = ref([
  { label: t('general.created_at'), value: 'created_at' },
  { label: t('general.uses'), value: 'uses' },
  { label: t('general.name'), value: 'name' },
])
const loading = ref(false)
const searchForm = ref<SearchTitleGroupTagsQuery>({ name: '', order_by_column: 'name', order_by_direction: 'asc', page: 1, page_size: 20 })
const searchResults = ref<TitleGroupTagEnriched[]>([])
const totalResults = ref<number>(0)
const totalPages = computed(() => Math.ceil(totalResults.value / searchForm.value.page_size))
const editTagDialogVisible = ref(false)
const deleteTagDialogVisible = ref(false)
const tagBeingEdited = ref<EditedTitleGroupTag | null>(null)
const tagBeingDeleted = ref<EditedTitleGroupTag | null>(null)

const updateUrl = () => {
  searchForm.value.page = 1
  router.push({ query: searchForm.value })
}

const fetchSearchResultsFromUrl = async () => {
  loading.value = true
  searchForm.value.page = route.query.page ? parseInt(route.query.page as string) : 1
  searchForm.value.page_size = route.query.page_size ? parseInt(route.query.page_size as string) : 20
  searchForm.value.name = route.query.name ? (route.query.name as string) : ''
  // @ts-expect-error what is placed in this query always comes from the form, so there shouldn't be a wrong value
  searchForm.value.order_by_column = route.query.order_by_column ? (route.query.order_by_column as string) : 'created_at'
  // @ts-expect-error what is placed in this query always comes from the form, so there shouldn't be a wrong value
  searchForm.value.order_by_direction = route.query.order_by_direction ? (route.query.order_by_direction as string) : 'desc'
  const response = await searchTitleGroupTags(searchForm.value)
  searchResults.value.length = 0
  await nextTick()
  searchResults.value = response.results
  totalResults.value = response.total_items
  loading.value = false
}

onMounted(async () => {
  await fetchSearchResultsFromUrl()
})

watch(
  () => route.query,
  () => {
    fetchSearchResultsFromUrl()
  },
  { deep: true },
)

const editTag = (tag: TitleGroupTagEnriched) => {
  tagBeingEdited.value = {
    id: tag.id,
    name: tag.name,
    synonyms: tag.synonyms,
  }
  editTagDialogVisible.value = true
}

const deleteTag = (tag: EditedTitleGroupTag) => {
  tagBeingDeleted.value = {
    id: tag.id,
    name: tag.name,
    synonyms: tag.synonyms,
  }
  deleteTagDialogVisible.value = true
}

const tagEdited = () => {
  editTagDialogVisible.value = false
  fetchSearchResultsFromUrl()
}

const tagDeleted = () => {
  deleteTagDialogVisible.value = false
  fetchSearchResultsFromUrl()
}
</script>

<style scoped>
.search-form {
  margin-bottom: 15px;
}
.actions {
  i {
    margin-right: 5px;
  }
}
</style>
