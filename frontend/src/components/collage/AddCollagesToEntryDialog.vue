<template>
  <div id="add-entries-to-collage-dialog">
    <div class="entries">
      <!--we need to use a key that's specific to each entry, otherwise removing an entry might show the wrong collage link-->
      <div v-for="(collage, index) in newCollageEntries" :key="collage.idForTemplate" class="entry">
        <CollageSearchBar
          :placeholder="t('collage.collage_url_or_search_by_name')"
          @collage-selected="(collage: CollageLite) => (collageLinks[index] = `https://${getHostname()}/collage/${collage.id}`)"
          @url-entered="(url: string) => (collageLinks[index] = url)"
        />
        <InputText class="note" :placeholder="t('collage.note')" v-model="newCollageEntries[index].note" />
        <Button v-if="index == 0" @click="addCollageEntry" icon="pi pi-plus" size="small" />
        <Button v-if="newCollageEntries.length > 1" @click="removeCollageEntry(index)" icon="pi pi-minus" size="small" />
      </div>
    </div>
    <div class="wrapper-center" style="margin-top: 10px">
      <Button :label="t('collage.add_collage_to_entry', 2)" size="small" :loading @click="sendCollageEntries" />
    </div>
  </div>
</template>
<script setup lang="ts">
import { InputText, Button } from 'primevue'
import { useI18n } from 'vue-i18n'
import { ref } from 'vue'
import { onMounted } from 'vue'
import CollageSearchBar from './CollageSearchBar.vue'
import { getHostname } from '@/services/helpers'
import { insertsEntriesIntoACollage, type CollageEntry, type CollageLite, type UserCreatedCollageEntry } from '@/services/api-schema'

const { t } = useI18n()

const emit = defineEmits<{
  addedEntries: [CollageEntry[]]
}>()
const props = defineProps<{
  titleGroupId: number
}>()

const loading = ref(false)
const newCollageEntries = ref<(UserCreatedCollageEntry & { idForTemplate: string })[]>([])
const collageLinks = ref<string[]>([])

const sendCollageEntries = async () => {
  loading.value = true
  newCollageEntries.value.forEach((entry, index) => {
    entry.collage_id = parseInt(collageLinks.value[index].split('/').pop() as string)
  })
  insertsEntriesIntoACollage(newCollageEntries.value)
    .then((data) => {
      emit('addedEntries', data)
    })
    .finally(() => (loading.value = false))
}

const addCollageEntry = () => {
  collageLinks.value.push('')
  newCollageEntries.value.push({ collage_id: 0, title_group_id: props.titleGroupId, note: null, idForTemplate: new Date().getTime().toString() })
}
const removeCollageEntry = (index: number) => {
  console.log(index, newCollageEntries.value[index], collageLinks.value[index])
  newCollageEntries.value.splice(index, 1)
  collageLinks.value.splice(index, 1)
}

onMounted(() => addCollageEntry())
</script>
<style scoped>
#add-entries-to-collage-dialog {
  width: 70vw;
  display: flex;
  flex-direction: column;
  align-items: center;
}
.entries {
  width: 100%;
}
.entry {
  display: flex;
  width: 100%;
  > * {
    margin: 5px;
  }
  .note {
    width: 60%;
  }
}
</style>
