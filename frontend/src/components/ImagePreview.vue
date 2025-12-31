<template>
  <Image :src="imageLink" :alt preview loading="lazy">
    <template #previewicon>
      <i class="pi pi-search"></i>
    </template>
  </Image>
</template>
<script setup lang="ts">
import { Image } from 'primevue'

defineProps<{
  imageLink: string
  alt?: string
}>()
</script>
