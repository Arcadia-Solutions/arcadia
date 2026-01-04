<template>
  <div class="diff-viewer">
    <div v-for="(change, field) in edits" :key="field">
      <b>{{ field }}</b>
      <div>old: {{ formatValue(change.old) }}</div>
      <div>new: <span v-html="computeInlineDiff(change.old, change.new)"></span></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { diffChars } from 'diff'

defineProps<{
  edits: Record<string, { old: unknown; new: unknown }>
}>()

const formatValue = (value: unknown): string => {
  if (value === null || value === undefined) return 'null'
  if (typeof value === 'object') return JSON.stringify(value)
  return String(value)
}

const computeInlineDiff = (oldVal: unknown, newVal: unknown): string => {
  const oldStr = formatValue(oldVal)
  const newStr = formatValue(newVal)

  return diffChars(oldStr, newStr)
    .map((part) => {
      const escaped = part.value.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;')
      if (part.removed) return `<span class="removed">${escaped}</span>`
      if (part.added) return `<span class="added">${escaped}</span>`
      return escaped
    })
    .join('')
}
</script>

<style scoped>
.diff-viewer {
  font-family: monospace;
  font-size: 0.9em;
  white-space: pre-wrap;
  word-break: break-word;
}

:deep(.removed) {
  background: rgba(255, 0, 0, 0.2);
  text-decoration: line-through;
}

:deep(.added) {
  background: rgba(0, 255, 0, 0.2);
}
</style>
