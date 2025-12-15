<template>
  <div id="add-entries-to-collage-dialog">
    <div class="entries">
      <div v-for="(_link, index) in newCollageEntries" :key="index" class="entry">
        <InputText placeholder="title group link" v-model="entryLinks[index]" />
        <InputText class="note" :placeholder="t('collage.note')" v-model="newCollageEntries[index].note" />
        <Button v-if="index == 0" @click="addCollageEntry" icon="pi pi-plus" size="small" />
        <Button v-if="newCollageEntries.length > 0" @click="removeCollageEntry(index)" icon="pi pi-minus" size="small" />
      </div>
    </div>
    <div class="wrapper-center" style="margin-top: 10px">
      <Button :label="t('collage.add_entry_to_collage', 2)" size="small" :loading @click="sendCollageEntries" />
    </div>
  </div>
</template>
<script setup lang="ts">
import { InputText, Button } from 'primevue'
import { useI18n } from 'vue-i18n'
import { ref } from 'vue'
import { onMounted } from 'vue'
import { insertsEntriesIntoACollage, type CollageEntry, type UserCreatedCollageEntry } from '@/services/api-schema'

const { t } = useI18n()

const emit = defineEmits<{
  addedEntries: [CollageEntry[]]
}>()
const props = defineProps<{
  collageId: number
}>()

const loading = ref(false)
const newCollageEntries = ref<UserCreatedCollageEntry[]>([])
const entryLinks = ref<string[]>([])

const sendCollageEntries = async () => {
  loading.value = true
  newCollageEntries.value.forEach((entry, index) => {
    const id = parseInt(entryLinks.value[index].split('/').pop() as string)
    entry.title_group_id = id
  })
  insertsEntriesIntoACollage(newCollageEntries.value)
    .then((data) => {
      emit('addedEntries', data)
    })
    .finally(() => (loading.value = false))
}

const addCollageEntry = () => {
  entryLinks.value.push('')
  newCollageEntries.value.push({ collage_id: props.collageId, note: null, title_group_id: 0 })
}
const removeCollageEntry = (index: number) => {
  newCollageEntries.value.splice(index, 1)
  entryLinks.value.splice(index, 1)
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
    width: 50%;
  }
}
</style>
