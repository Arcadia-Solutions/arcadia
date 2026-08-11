<template>
  <div v-if="barVisible" id="page-progress-bar" :style="{ width: `${progress}%`, opacity: progress >= 100 ? 0 : 1 }"></div>
</template>

<script setup lang="ts">
import { barVisible, progress } from '@/services/progressBar'
</script>

<style scoped>
#page-progress-bar {
  position: fixed;
  top: 0;
  left: 0;
  height: 2px;
  z-index: 2000;
  background-color: var(--color-primary);
  box-shadow: 0 0 8px var(--color-primary);
  transition:
    width 0.2s ease-out,
    opacity 0.3s ease-out;
}
</style>
