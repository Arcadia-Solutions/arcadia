<!-- This component is deprecated and should only be used by the torrents search, until it is replaced -->
<template>
  <div class="pagination">
    <Button :disabled="page < 2" @click="emit('changePage', 'previous')" label="<" size="small" />
    {{ t('general.page') }} {{ page }}
    <Button @click="emit('changePage', 'next')" label=">" size="small" />
  </div>
</template>
<script setup lang="ts">
import { Button } from 'primevue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

defineProps<{
  page: number
}>()

const emit = defineEmits<{
  changePage: ['previous' | 'next']
}>()
</script>
<style scoped>
.pagination {
  text-align: center;
  margin: 10px 0;
  .p-button {
    margin: 0 10px;
  }
}
</style>
