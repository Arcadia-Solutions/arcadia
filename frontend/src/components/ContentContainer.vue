<template>
  <div class="content-container">
    <div class="top">
      <div>
        <slot name="top-left"></slot>
        <RouterLink v-if="containerTitleLink" :to="containerTitleLink">
          <div class="container-title" v-if="containerTitle">{{ containerTitle }}</div>
        </RouterLink>
        <div v-else-if="containerTitle" class="container-title">{{ containerTitle }}</div>
      </div>
      <slot name="top-right"></slot>
    </div>
    <slot></slot>
  </div>
</template>

<script setup lang="ts">
import { RouterLink } from 'vue-router'
defineProps<{
  containerTitle?: string
  containerTitleLink?: string
}>()
</script>

<style scoped>
.content-container {
  padding: 12px;
  border-radius: 10px;
  background-color: var(--color-background-secondary);
  display: block !important;
}
.top {
  display: flex;
  justify-content: space-between;
}
.container-title {
  color: var(--color-primary);
  margin-bottom: 10px;
}
</style>
