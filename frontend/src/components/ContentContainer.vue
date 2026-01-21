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
    <div class="content-body">
      <slot></slot>
    </div>
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
  border-radius: 10px;
  overflow: hidden;
  background-color: var(--color-background-secondary);
  display: block !important;
}
.content-body {
  padding: 10px;
  padding-top: 0;
}
.top {
  display: flex;
  justify-content: space-between;
  padding: 10px;
  padding-bottom: 5px;
}
.container-title {
  color: var(--color-primary);
  margin-bottom: 10px;
}
</style>
