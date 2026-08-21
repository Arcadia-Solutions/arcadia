<template>
  <div class="emojis-manager">
    <p>{{ t('emoji_management.description') }}</p>
    <DataTable :value="emojis" size="small" dataKey="id" :rowClass="(emoji) => (emoji.enabled ? '' : 'emoji-disabled')">
      <Column :header="t('arcadia_settings.emoji_preview')">
        <template #body="slotProps">
          <img
            v-if="!slotProps.data.unicode_character"
            :src="emojiImageUrl(slotProps.data.id, slotProps.data.image_version)"
            :alt="slotProps.data.name"
            class="emoji-preview"
          />
          <span v-else>{{ slotProps.data.unicode_character }}</span>
        </template>
      </Column>
      <Column :header="t('arcadia_settings.emoji_name')" field="name" />
      <Column :header="t('general.uses')">
        <template #body="slotProps">{{ reactionsAmountOf(slotProps.data.id) }}</template>
      </Column>
      <Column :header="t('emoji_management.enabled')">
        <template #body="slotProps">
          <ToggleSwitch
            :modelValue="slotProps.data.enabled"
            :disabled="updatingEnabled"
            v-tooltip.top="t('emoji_management.enabled_hint')"
            @update:modelValue="toggleEnabled(slotProps.data, $event)"
          />
        </template>
      </Column>
      <Column :header="t('emoji_management.order')">
        <template #body="slotProps">
          <Button
            icon="pi pi-arrow-up"
            size="small"
            text
            rounded
            :disabled="reordering || isFirst(slotProps.data)"
            v-tooltip.top="t('emoji_management.move_up')"
            @click="moveUp(slotProps.data)"
          />
          <Button
            icon="pi pi-arrow-down"
            size="small"
            text
            rounded
            :disabled="reordering || isLast(slotProps.data)"
            v-tooltip.top="t('emoji_management.move_down')"
            @click="moveDown(slotProps.data)"
          />
        </template>
      </Column>
      <Column :header="t('general.action', 2)">
        <template #body="slotProps">
          <i class="pi pi-pen-to-square cursor-pointer" v-tooltip.top="t('general.edit')" @click="startEditing(slotProps.data)" style="margin-right: 10px" />
          <i class="pi pi-trash cursor-pointer" v-tooltip.top="t('general.delete')" @click="openDeleteDialog(slotProps.data)" />
        </template>
      </Column>
    </DataTable>

    <div class="emoji-form">
      <InputText v-model="name" :placeholder="t('arcadia_settings.emoji_name')" size="small" />
      <InputText v-model="unicodeCharacter" :placeholder="t('arcadia_settings.emoji_unicode_character')" size="small" :disabled="unicodeInputDisabled" />
      <FileUpload
        :key="fileUploadKey"
        mode="basic"
        accept="image/png,image/webp,image/gif,image/svg+xml"
        :maxFileSize="32768"
        :auto="false"
        :disabled="imageUploadDisabled"
        :chooseLabel="t('arcadia_settings.emoji_image')"
        :chooseButtonProps="{ size: 'small' }"
        @select="onImageSelected"
      />
      <Button
        v-if="unicodeCharacter.trim() || image"
        icon="pi pi-times"
        size="small"
        text
        rounded
        v-tooltip.top="t('emoji_management.clear_representation')"
        @click="clearRepresentation"
      />
      <Button :label="emojiBeingEdited ? t('general.edit') : t('general.create')" size="small" :loading="saving" :disabled="!canSave" @click="saveEmoji" />
      <Button v-if="emojiBeingEdited" :label="t('general.cancel')" size="small" severity="secondary" @click="resetForm" />
    </div>

    <Dialog closeOnEscape modal :header="t('general.delete')" v-model:visible="deleteDialogVisible">
      <DeleteDialog
        :message="deleteConfirmMessage"
        :action="deletePendingEmoji"
        :successMessage="t('arcadia_settings.emoji_deleted_success')"
        @deleted="onEmojiDeleted"
      />
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import ToggleSwitch from 'primevue/toggleswitch'
import { Button, Dialog, FileUpload, InputText } from 'primevue'
import type { FileUploadSelectEvent } from 'primevue/fileupload'
import { deleteEmoji, getEmojis, getEmojisUsage, reorderEmojis, setEmojiEnabled, type Emoji, type EmojiUsage } from '@/services/api-schema'
import { createEmoji, editEmoji } from '@/services/api/emojiService'
import { emojiImageUrl } from '@/services/emojis'
import { useEmojisStore } from '@/stores/emojis'
import { showToast } from '@/main'
import DeleteDialog from '@/components/DeleteDialog.vue'

const { t } = useI18n()
const emojisStore = useEmojisStore()

const emojis = ref<Emoji[]>([])
const usage = ref<EmojiUsage[]>([])
const emojiBeingEdited = ref<Emoji | null>(null)
const name = ref('')
const unicodeCharacter = ref('')
const image = ref<File | null>(null)
const saving = ref(false)
const reordering = ref(false)
const updatingEnabled = ref(false)
// Bumped to force-remount the FileUpload whenever the selected file needs clearing, since it
// does not expose a public method to do so.
const fileUploadKey = ref(0)
const deleteDialogVisible = ref(false)
const emojiPendingDeletion = ref<Emoji | null>(null)

const reactionsAmountOf = (emojiId: number) => usage.value.find((entry) => entry.emoji_id === emojiId)?.reactions_amount ?? 0

// A representation is required to create an emoji, but optional when editing one, since leaving
// both blank keeps the emoji's current representation.
const canSave = computed(
  () => Boolean(name.value.trim()) && (emojiBeingEdited.value !== null || Boolean(unicodeCharacter.value.trim()) || image.value !== null),
)

// An emoji is either a unicode character or an image, never both, so picking one disables the
// other input rather than silently overwriting it.
const unicodeInputDisabled = computed(() => image.value !== null)
const imageUploadDisabled = computed(() => Boolean(unicodeCharacter.value.trim()))

const deleteConfirmMessage = computed(() =>
  emojiPendingDeletion.value ? t('arcadia_settings.confirm_delete_emoji', { n: reactionsAmountOf(emojiPendingDeletion.value.id) }) : '',
)

const isFirst = (emoji: Emoji) => emojis.value[0]?.id === emoji.id
const isLast = (emoji: Emoji) => emojis.value[emojis.value.length - 1]?.id === emoji.id

const loadCatalogue = () => {
  Promise.all([getEmojis(), getEmojisUsage()])
    .then(([emojisData, usageData]) => {
      emojis.value = emojisData
      usage.value = usageData
    })
    // The axios interceptor already reports the failure; caught only so it is not an unhandled
    // rejection. Same reasoning for the other .catch(() => {}) below.
    .catch(() => {})
}

onMounted(loadCatalogue)

const onImageSelected = (event: FileUploadSelectEvent) => {
  const files = Array.isArray(event.files) ? event.files : [event.files]
  image.value = files[0] ?? null
}

// The only way to clear a chosen file back out, since FileUpload in basic mode offers no clear
// affordance of its own once a file is selected; also available for the unicode character field,
// which can otherwise already be cleared by deleting its text.
const clearRepresentation = () => {
  unicodeCharacter.value = ''
  image.value = null
  fileUploadKey.value++
}

const resetForm = () => {
  emojiBeingEdited.value = null
  name.value = ''
  unicodeCharacter.value = ''
  image.value = null
  fileUploadKey.value++
}

const startEditing = (emoji: Emoji) => {
  emojiBeingEdited.value = emoji
  name.value = emoji.name
  unicodeCharacter.value = emoji.unicode_character ?? ''
  image.value = null
  fileUploadKey.value++
}

const saveEmoji = () => {
  saving.value = true
  const fields = {
    name: name.value.trim(),
    unicode_character: unicodeCharacter.value.trim() || undefined,
    image: image.value ?? undefined,
  }
  const editing = emojiBeingEdited.value
  const request = editing ? editEmoji({ id: editing.id, ...fields }) : createEmoji(fields)
  request
    .then(() => {
      showToast('', editing ? t('arcadia_settings.emoji_edited_success') : t('arcadia_settings.emoji_created_success'), 'success', 2000)
      resetForm()
      // The picker store would otherwise keep serving the catalogue from before this change.
      emojisStore.invalidate()
      loadCatalogue()
    })
    .catch(() => {})
    .finally(() => {
      saving.value = false
    })
}

// Swaps the emoji at `index` with its neighbour, reflects the new order immediately, then
// persists the whole ordered list so the server always receives a consistent set. On failure
// the previous order is restored and the error is surfaced.
const swapWithNeighbour = (index: number, neighbourIndex: number) => {
  const previousOrder = emojis.value
  const reordered = [...emojis.value]
  ;[reordered[index], reordered[neighbourIndex]] = [reordered[neighbourIndex], reordered[index]]
  emojis.value = reordered
  reordering.value = true
  reorderEmojis({
    emojis: reordered.map((emoji, position) => ({ id: emoji.id, sort_order: position })),
  })
    .then(() =>
      // Reconcile with the server's own ordering rather than assuming ours matches exactly.
      // The reorder itself already succeeded at this point, so a failure here just keeps the
      // optimistic order shown instead of rolling it back to what came before this swap.
      getEmojis()
        .then((emojisData) => {
          emojis.value = emojisData
        })
        .catch(() => {}),
    )
    .then(() => {
      // The picker orders its emojis, and places newly added reaction chips, from this same
      // catalogue: left cached, it would keep the order from before the swap.
      emojisStore.invalidate()
    })
    .catch(() => {
      emojis.value = previousOrder
      showToast('', t('emoji_management.reorder_failed'), 'error', 3000)
    })
    .finally(() => {
      reordering.value = false
    })
}

const moveUp = (emoji: Emoji) => {
  const index = emojis.value.findIndex((entry) => entry.id === emoji.id)
  if (index > 0) {
    swapWithNeighbour(index, index - 1)
  }
}

const moveDown = (emoji: Emoji) => {
  const index = emojis.value.findIndex((entry) => entry.id === emoji.id)
  if (index >= 0 && index < emojis.value.length - 1) {
    swapWithNeighbour(index, index + 1)
  }
}

// Flips the emoji's enabled state immediately, then persists it, reverting and surfacing an
// error on failure, the same way the reorder controls above handle it.
const toggleEnabled = (emoji: Emoji, enabled: boolean) => {
  const previous = emoji.enabled
  emoji.enabled = enabled
  updatingEnabled.value = true
  setEmojiEnabled({ id: emoji.id, enabled })
    .then(() => {
      // A disabled emoji stays offered by an already loaded picker otherwise, and picking it
      // fails with "this emoji is disabled".
      emojisStore.invalidate()
    })
    .catch(() => {
      emoji.enabled = previous
      showToast('', t('emoji_management.enabled_update_failed'), 'error', 3000)
    })
    .finally(() => {
      updatingEnabled.value = false
    })
}

const openDeleteDialog = (emoji: Emoji) => {
  emojiPendingDeletion.value = emoji
  deleteDialogVisible.value = true
  // Reactions may have accumulated since the catalogue was last loaded, so refresh the usage
  // count shown in the confirmation message rather than risking a stale one.
  getEmojisUsage()
    .then((usageData) => {
      usage.value = usageData
    })
    .catch(() => {})
}

const deletePendingEmoji = () => {
  if (!emojiPendingDeletion.value) {
    return Promise.reject(new Error('No emoji selected for deletion'))
  }
  return deleteEmoji(emojiPendingDeletion.value.id)
}

const onEmojiDeleted = () => {
  deleteDialogVisible.value = false
  emojiPendingDeletion.value = null
  // The picker store would otherwise keep serving the catalogue from before this change.
  emojisStore.invalidate()
  loadCatalogue()
}
</script>

<style scoped>
.emojis-manager {
  margin-top: 20px;
}
.emoji-form {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 10px;
  margin-top: 15px;
}
.emoji-preview {
  height: 1.4em;
  width: auto;
}
:deep(tr.emoji-disabled) {
  opacity: 0.5;
}
</style>
