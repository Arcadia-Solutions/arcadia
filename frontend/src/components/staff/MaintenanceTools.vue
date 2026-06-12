<template>
  <div class="maintenance-tools">
    <div class="tool">
      <div class="tool-info">
        <h3>{{ t('maintenance_tools.rehash_torrents') }}</h3>
        <p>{{ t('maintenance_tools.rehash_torrents_description') }}</p>
      </div>
      <Button size="small" :label="t('maintenance_tools.run')" :loading="rehashing" @click="rehashTorrents" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import Button from 'primevue/button'
import { rehashTorrentsWithSourceTag } from '@/services/api-schema'
import { showToast } from '@/main'

const { t } = useI18n()

const rehashing = ref(false)

const rehashTorrents = () => {
  rehashing.value = true
  // No timeout: rehashing every torrent can take a long time on big trackers.
  rehashTorrentsWithSourceTag({ timeout: 0 })
    .then((data) => {
      showToast('Success', t('maintenance_tools.rehash_torrents_success', { amount: data.updated_torrents_amount }), 'success', 5000)
    })
    .finally(() => {
      rehashing.value = false
    })
}
</script>

<style scoped>
.maintenance-tools {
  display: flex;
  flex-direction: column;
  gap: 20px;
}
.tool {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 20px;
  padding: 15px;
  border: 1px solid var(--color-border);
  border-radius: 5px;
}
.tool-info h3 {
  margin: 0 0 5px;
}
.tool-info p {
  margin: 0;
  opacity: 0.8;
}
</style>
