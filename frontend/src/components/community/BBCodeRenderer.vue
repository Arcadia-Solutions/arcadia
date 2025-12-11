<template>
  <div class="bbcode-content" v-html="parsedContent"></div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { parseBBCode } from '@/services/bbcode'

const props = defineProps<{
  content: string | null | undefined
}>()

const parsedContent = computed(() => {
  return parseBBCode(props.content || '')
})
</script>

<style scoped>
.bbcode-content {
  width: 100%;
  overflow-wrap: break-word;
  word-break: break-word;
}

.bbcode-content :deep(img) {
  max-width: 100%;
  height: auto;
}

.bbcode-content :deep(blockquote) {
  border-left: 3px solid var(--surface-border);
  margin-left: 0;
  padding-left: 1rem;
  color: var(--text-color-secondary);
}

.bbcode-content :deep(pre) {
  background-color: var(--surface-ground);
  padding: 1rem;
  border-radius: 6px;
  overflow-x: auto;
}
</style>
