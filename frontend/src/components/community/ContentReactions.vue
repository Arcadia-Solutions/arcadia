<template>
  <div class="content-reactions" v-if="reactionsState.length > 0 || canReact">
    <button
      v-for="reaction in reactionsState"
      :key="reaction.emoji_id"
      type="button"
      class="reaction-chip"
      :class="{ reacted: reaction.reacted_by_current_user }"
      :aria-disabled="!canReact || pendingEmojiIds.has(reaction.emoji_id)"
      @click="toggleReaction(reaction.emoji_id)"
      @mouseenter="onChipHovered($event, reaction.emoji_id)"
      @mouseleave="onChipLeft"
    >
      <img v-if="!reaction.emoji_unicode_character" :src="emojiImageUrl(reaction.emoji_id, reaction.emoji_image_version)" :alt="reaction.emoji_name" />
      <span v-else>{{ reaction.emoji_unicode_character }}</span>
      <span class="amount">{{ reaction.amount }}</span>
    </button>
    <button v-if="canReact" type="button" class="reaction-chip add-reaction" v-tooltip.top="t('reaction.add_reaction')" @click="openPicker">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="12" cy="12" r="9" />
        <line x1="9" y1="9.5" x2="9" y2="9.51" />
        <line x1="15" y1="9.5" x2="15" y2="9.51" />
        <path d="M8 13.5c1 1.7 2.4 2.5 4 2.5s3-.8 4-2.5" />
      </svg>
    </button>
  </div>
  <Popover ref="pickerPopover">
    <div class="emoji-picker">
      <span v-if="pickableEmojis.length === 0">{{ t('reaction.no_emoji_available') }}</span>
      <button
        v-for="emoji in pickableEmojis"
        :key="emoji.id"
        type="button"
        class="picker-emoji"
        :disabled="pendingEmojiIds.has(emoji.id)"
        v-tooltip.top="emoji.name"
        @click="pickEmoji(emoji.id)"
      >
        <img v-if="!emoji.unicode_character" :src="emojiImageUrl(emoji.id, emoji.image_version)" :alt="emoji.name" />
        <span v-else>{{ emoji.unicode_character }}</span>
      </button>
    </div>
  </Popover>
  <Popover ref="usersPopover" @show="usersPopoverVisible = true" @hide="usersPopoverVisible = false">
    <div class="reaction-users" @mouseenter="onPopoverEntered" @mouseleave="onPopoverLeft">
      <template v-if="hoveredEmojiName !== null">
        <div class="reaction-users-emoji-name">{{ hoveredEmojiName }}</div>
        <span v-if="loadingUsers"><i class="pi pi-hourglass" /></span>
        <template v-else-if="hoveredReactors">
          <div class="reaction-users-title">{{ t('reaction.reacted_by') }}</div>
          <div v-for="user in hoveredReactors.users" :key="user.id">
            <UsernameEnriched :user="user" />
          </div>
          <div v-if="hiddenUsersAmount > 0">{{ t('reaction.and_n_others', { n: hiddenUsersAmount }) }}</div>
        </template>
      </template>
    </div>
  </Popover>
</template>

<script setup lang="ts">
import { ref, computed, onBeforeUnmount, nextTick, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import Popover from 'primevue/popover'
import {
  createForumPostReaction,
  deleteForumPostReaction,
  getForumPostReactionUsers,
  type ContentReaction,
  type Emoji,
  type UserLite,
} from '@/services/api-schema'
import { emojiImageUrl } from '@/services/emojis'
import { useEmojisStore } from '@/stores/emojis'
import { useUserStore } from '@/stores/user'
import { showToast } from '@/main'
import UsernameEnriched from '@/components/user/UsernameEnriched.vue'

const props = defineProps<{ reactions: ContentReaction[]; forumPostId: number }>()

const { t } = useI18n()
const userStore = useUserStore()
const emojisStore = useEmojisStore()

// Each reaction is cloned, not just the array: the optimistic update mutates these objects, and
// the parent keeps its own array across a post edit, so sharing them would leak the optimistic
// (or reverted) amounts into the parent's state for good.
const reactionsState = ref<ContentReaction[]>(props.reactions.map((reaction) => ({ ...reaction })))
const pickerPopover = ref<InstanceType<typeof Popover>>()
const canReact = computed(() => userStore.permissions.includes('react_to_content'))
// Disabled emojis stay visible as existing reaction chips, but the picker only offers ones that
// can still be newly picked.
const pickableEmojis = computed(() => emojisStore.emojis.filter((emoji) => emoji.enabled))
// Emoji ids with a create/delete request in flight, so a second click on the same chip before
// the first request settles can't fire a concurrent, possibly out-of-order request.
const pendingEmojiIds = ref<Set<number>>(new Set())

const openPicker = (event: Event) => {
  // Captured now: by the time loadOnce() resolves, the click has finished dispatching and
  // event.currentTarget is already null, which would leave PrimeVue with no anchor to measure.
  const target = event.currentTarget
  emojisStore
    .loadOnce()
    .then(() => pickerPopover.value?.toggle(event, target))
    .catch(() => showToast('', t('reaction.could_not_load_emojis'), 'error'))
}

const findReaction = (emojiId: number) => reactionsState.value.find((reaction) => reaction.emoji_id === emojiId)

const removeReaction = (emojiId: number) => {
  reactionsState.value = reactionsState.value.filter((reaction) => reaction.emoji_id !== emojiId)
}

// Position of an emoji within the catalogue order (sort_order, then id) — the same order the
// backend returns reactions in. Only used to place a brand new reaction chip, since the
// catalogue is guaranteed loaded whenever one is added (it can only come from the picker).
const catalogueIndex = (emojiId: number) => emojisStore.emojis.findIndex((emoji) => emoji.id === emojiId)

const insertReactionInSortOrder = (reaction: ContentReaction) => {
  const targetIndex = catalogueIndex(reaction.emoji_id)
  const insertBeforeIndex = reactionsState.value.findIndex((existing) => {
    const existingIndex = catalogueIndex(existing.emoji_id)
    return existingIndex === -1 || existingIndex > targetIndex
  })
  if (insertBeforeIndex === -1) {
    reactionsState.value.push(reaction)
  } else {
    reactionsState.value.splice(insertBeforeIndex, 0, reaction)
  }
}

const pickEmoji = (emojiId: number) => {
  pickerPopover.value?.hide()
  toggleReaction(emojiId)
}

const toggleReaction = (emojiId: number) => {
  if (!canReact.value || pendingEmojiIds.value.has(emojiId)) return

  const existing = findReaction(emojiId)
  const existingIndex = existing ? reactionsState.value.indexOf(existing) : -1
  const wasReacted = existing?.reacted_by_current_user ?? false
  const snapshot = existing ? { ...existing } : undefined
  const emoji = existing ? undefined : emojisStore.emojis.find((candidate) => candidate.id === emojiId)

  pendingEmojiIds.value.add(emojiId)
  applyToggle(emojiId, wasReacted, emoji)
  patchCachedUsers(emojiId, !wasReacted)

  const request = wasReacted
    ? deleteForumPostReaction({ forum_post_id: props.forumPostId, emoji_id: emojiId })
    : createForumPostReaction({ forum_post_id: props.forumPostId, emoji_id: emojiId })

  request
    .catch(() => {
      revertToggle(emojiId, wasReacted, snapshot, existingIndex)
      patchCachedUsers(emojiId, wasReacted)
    })
    .finally(() => pendingEmojiIds.value.delete(emojiId))
}

// Optimistic update, reverted by revertToggle if the request fails.
const applyToggle = (emojiId: number, wasReacted: boolean, emoji: Emoji | undefined) => {
  const existing = findReaction(emojiId)
  if (existing) {
    if (wasReacted) {
      existing.amount -= 1
      existing.reacted_by_current_user = false
      if (existing.amount === 0) removeReaction(emojiId)
    } else {
      existing.amount += 1
      existing.reacted_by_current_user = true
    }
  } else if (emoji) {
    insertReactionInSortOrder({
      emoji_id: emoji.id,
      emoji_name: emoji.name,
      emoji_unicode_character: emoji.unicode_character,
      emoji_image_version: emoji.image_version,
      amount: 1,
      reacted_by_current_user: true,
    })
  }
}

const revertToggle = (emojiId: number, wasReacted: boolean, snapshot: ContentReaction | undefined, snapshotIndex: number) => {
  const existing = findReaction(emojiId)
  if (existing) {
    if (wasReacted) {
      existing.amount += 1
      existing.reacted_by_current_user = true
    } else {
      existing.amount -= 1
      existing.reacted_by_current_user = false
      if (existing.amount === 0) removeReaction(emojiId)
    }
  } else if (snapshot) {
    // The chip was removed entirely by the optimistic update (it was the current user's only
    // reaction of this kind): put it back at its original position, not the catalogue order,
    // since the emoji catalogue may not even be loaded on this path (removing a reaction never
    // requires opening the picker).
    reactionsState.value.splice(Math.min(snapshotIndex, reactionsState.value.length), 0, snapshot)
  }
}

const HOVER_DELAY_MILLISECONDS = 300
// Travelling the gap between a chip and the popover above/below it takes a moment: without this
// grace period, the popover's own mouseleave-driven hide (or the chip's) would dismiss it before
// the pointer ever reaches the panel.
const HIDE_GRACE_PERIOD_MILLISECONDS = 200

const usersPopover = ref<InstanceType<typeof Popover>>()
const usersPopoverVisible = ref(false)
const hoveredEmojiId = ref<number | null>(null)
const showTimeout = ref<number | null>(null)
const hideTimeout = ref<number | null>(null)
// Users who reacted to this post, keyed by emoji id. Fetched at most once per component
// instance: every emoji's reactors come back in a single request, so the first hover on any
// chip of this post loads it for all of them.
const cachedUsers = ref<Record<number, { users: UserLite[]; totalAmount: number }> | null>(null)
const loadingUsers = ref(false)

const loadUsersOnce = () => {
  if (cachedUsers.value !== null || loadingUsers.value) return
  loadingUsers.value = true
  getForumPostReactionUsers(props.forumPostId)
    .then((groups) => {
      const usersPerEmoji: Record<number, { users: UserLite[]; totalAmount: number }> = {}
      groups.forEach((group) => {
        usersPerEmoji[group.emoji_id] = {
          users: group.users,
          totalAmount: group.total_amount,
        }
      })
      cachedUsers.value = usersPerEmoji
    })
    .catch(() => {
      // Left uncached on purpose: cachedUsers stays null, so the next hover retries instead of
      // being stuck without a list forever.
    })
    .finally(() => {
      loadingUsers.value = false
    })
}

const clearShowTimeout = () => {
  if (showTimeout.value !== null) {
    window.clearTimeout(showTimeout.value)
    showTimeout.value = null
  }
}

const clearHideTimeout = () => {
  if (hideTimeout.value !== null) {
    window.clearTimeout(hideTimeout.value)
    hideTimeout.value = null
  }
}

const onChipHovered = (event: Event, emojiId: number) => {
  // Coming back to a chip (or another chip) before the grace period hides the popover: keep it.
  clearHideTimeout()
  // Same reasoning as openPicker: captured before the timeout, since event.currentTarget is
  // already null by the time it fires.
  const target = event.currentTarget
  showTimeout.value = window.setTimeout(() => {
    showTimeout.value = null
    // The chip can be gone before the delay elapses (un-reacting removes it), and its anchor
    // with it: there is nothing left to describe or to position against.
    if (!findReaction(emojiId)) return
    hoveredEmojiId.value = emojiId
    showUsersPopover(event, target)
    loadUsersOnce()
  }, HOVER_DELAY_MILLISECONDS)
}

// PrimeVue only positions the panel from its transition enter hook, which doesn't run while the
// popover is already open: moving straight from one chip to another would swap the content but
// leave the panel anchored to the previous chip. Reopening on the next tick runs the hook again
// against the new anchor.
const showUsersPopover = (event: Event, target: EventTarget | null) => {
  const popover = usersPopover.value
  if (!popover) return
  if (!usersPopoverVisible.value) {
    popover.show(event, target)
    return
  }
  popover.hide()
  nextTick(() => popover.show(event, target))
}

// Hides the popover after a short grace period instead of immediately, so moving the pointer
// from the chip into the popover panel (or back) doesn't dismiss it mid-travel.
const scheduleHide = () => {
  clearShowTimeout()
  clearHideTimeout()
  hideTimeout.value = window.setTimeout(() => {
    hideTimeout.value = null
    usersPopover.value?.hide()
  }, HIDE_GRACE_PERIOD_MILLISECONDS)
}

const onChipLeft = () => {
  scheduleHide()
}

const onPopoverEntered = () => {
  clearHideTimeout()
}

const onPopoverLeft = () => {
  scheduleHide()
}

onBeforeUnmount(() => {
  clearShowTimeout()
  clearHideTimeout()
})

const hoveredReactors = computed(() => (hoveredEmojiId.value === null ? null : (cachedUsers.value?.[hoveredEmojiId.value] ?? null)))

const hoveredEmojiName = computed(() => (hoveredEmojiId.value === null ? null : (findReaction(hoveredEmojiId.value)?.emoji_name ?? null)))

// Un-reacting your own only reaction of a kind removes the very chip the popover is describing.
// Dismiss it instead of leaving an empty panel floating until the grace period or the next hover.
watch(hoveredEmojiName, (emojiName) => {
  if (emojiName !== null) return
  clearShowTimeout()
  clearHideTimeout()
  hoveredEmojiId.value = null
  usersPopover.value?.hide()
})

const hiddenUsersAmount = computed(() => (hoveredReactors.value === null ? 0 : hoveredReactors.value.totalAmount - hoveredReactors.value.users.length))

/**
 * Keeps the cached list in step with the current user's own reaction instead of invalidating
 * it, so toggling a reaction never causes a new request.
 */
const patchCachedUsers = (emojiId: number, reacted: boolean) => {
  if (cachedUsers.value === null) return
  const group = cachedUsers.value[emojiId]
  if (reacted) {
    const currentUser: UserLite = { id: userStore.id, username: userStore.username, warned: userStore.warned, banned: userStore.banned }
    if (group) {
      group.users.push(currentUser)
      group.totalAmount += 1
    } else {
      cachedUsers.value[emojiId] = { users: [currentUser], totalAmount: 1 }
    }
    return
  }
  if (group) {
    group.users = group.users.filter((user) => user.id !== userStore.id)
    group.totalAmount -= 1
    if (group.totalAmount <= 0) {
      delete cachedUsers.value[emojiId]
    }
  }
}
</script>

<style scoped>
.content-reactions {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin-top: auto;
  padding-top: 10px;
  border-top: 1px solid var(--p-content-border-color);
}
.reaction-chip {
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 2px 8px;
  border: 1px solid var(--p-content-border-color);
  border-radius: 12px;
  background: none;
  color: inherit;
  cursor: pointer;
  font-size: 0.9em;
}
/* aria-disabled rather than the disabled attribute: browsers fire no mouseenter/mouseleave on a
   disabled button, which would keep members without the react permission from ever opening the
   reactors popover, and would leave it stuck open when a click disables the chip mid-hover.
   toggleReaction() is what actually refuses the click. */
.reaction-chip[aria-disabled='true'] {
  cursor: default;
}
.reaction-chip.reacted {
  border-color: var(--color-primary);
}
.reaction-chip img,
.picker-emoji img {
  height: 1.2em;
  width: auto;
}
.add-reaction svg {
  height: 1.2em;
  width: 1.2em;
}
.amount {
  color: var(--p-text-muted-color);
}
.emoji-picker {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  max-width: 240px;
}
.picker-emoji {
  border: none;
  background: none;
  cursor: pointer;
  font-size: 1.2em;
  color: inherit;
}
.reaction-users {
  display: flex;
  flex-direction: column;
  gap: 2px;
  max-height: 240px;
  overflow-y: auto;
  font-size: 0.9em;
}
.reaction-users-emoji-name {
  font-weight: bold;
  margin-bottom: 2px;
}
.reaction-users-title {
  color: var(--p-text-muted-color);
  margin-bottom: 4px;
}
</style>
