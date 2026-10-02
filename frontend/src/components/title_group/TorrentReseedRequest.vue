<template>
  <div v-if="lastSeededAt" class="reseed-info">
    <span>{{ t('reseed_request.last_seeded', { date: timeAgo(lastSeededAt) }) }}</span>
    <i
      v-if="withinReseedWindow"
      v-tooltip.top="reseedRequestSentAt ? t('reseed_request.already_sent', { time: timeAgo(reseedRequestSentAt) }) : t('reseed_request.button')"
      class="reseed-action pi pi-cart-arrow-down"
      :class="{ 'reseed-action--sent': reseedRequestSentAt }"
      @click="sendReseedRequest"
    />
  </div>
</template>

<script setup lang="ts">
import { timeAgo } from '@/services/helpers'
import { useI18n } from 'vue-i18n'
import { useUserStore } from '@/stores/user'
import { usePublicArcadiaSettingsStore } from '@/stores/publicArcadiaSettings'
import { computed, ref } from 'vue'
import { showToast } from '@/main'
import { requestReseed, type TorrentHierarchy, type TorrentHierarchyLite } from '@/services/api-schema'

// last_seeded_at is only provided by the full title group view
const props = defineProps<{
  torrent: TorrentHierarchyLite | TorrentHierarchy
}>()

const { t } = useI18n()
const userStore = useUserStore()
const publicArcadiaSettings = usePublicArcadiaSettingsStore()

const lastSeededAt = computed(() => ('last_seeded_at' in props.torrent && props.torrent.seeders === 0 ? props.torrent.last_seeded_at : null))

// set locally once the user sends a request this session, so the icon greys out immediately
const justSentAt = ref<string | null>(null)

// the most recent outstanding reseed request for this torrent, if any (backend value, or one just sent)
const reseedRequestSentAt = computed(
  () => justSentAt.value ?? ('reseed_request_sent_at' in props.torrent && props.torrent.seeders === 0 ? props.torrent.reseed_request_sent_at : null),
)

// client side UX only, the backend is authoritative. The action is relevant when the torrent has
// been dead long enough and the user may request a reseed, regardless of whether one was already sent.
const withinReseedWindow = computed(
  () =>
    !!lastSeededAt.value &&
    Date.now() - new Date(lastSeededAt.value).getTime() >= publicArcadiaSettings.reseed_requestable_after_hours * 3600 * 1000 &&
    userStore.permissions.includes('request_reseed'),
)

const sendReseedRequest = () => {
  if (reseedRequestSentAt.value) return
  requestReseed(props.torrent.id).then(() => {
    justSentAt.value = new Date().toISOString()
    showToast('', t('reseed_request.success'), 'success', 3000)
  })
}
</script>

<style scoped>
.reseed-info {
  font-size: 0.85em;
  display: flex;
  align-items: center;
  gap: 8px;
  margin-left: 7px;
}

.reseed-action {
  cursor: pointer;
}

.reseed-action--sent {
  color: grey;
  cursor: default;
}
</style>
