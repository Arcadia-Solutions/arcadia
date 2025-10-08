<template>
  <pre class="mediainfo" v-html="purifyHtml(props.mediainfo)"></pre>
</template>

<script setup lang="ts">
import { defineProps } from 'vue'
import DOMPurify from 'dompurify'

const props = defineProps<{
  mediainfo: string
}>()

const purifyHtml = (html: string) => {
  return DOMPurify.sanitize(html)
}
</script>

<style scoped>
.mediainfo {
  white-space: pre-wrap;
  word-break: break-word;
  font-size: 13px;
  font-family: 'Courier New', Courier, monospace;
  background-color: var(--surface-200);
  padding: 10px;
  border-radius: 5px;
  border: 1px solid var(--surface-400);
  max-height: 400px;
  overflow: auto;
}
</style>
