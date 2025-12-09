<template>
  <div class="affiliated-entity">
    <ImagePreview :imageLink="affiliatedEntity.entity.pictures[0] ?? ''" />
    <RouterLink :to="`/entity/${affiliatedEntity.entity.id}`">
      <div class="name">{{ affiliatedEntity.entity.name }}</div>
    </RouterLink>
    <div class="roles">
      <template v-for="(role, i) in affiliatedEntity.roles" :key="role">
        <span class="role">{{ t(`entity.role.${role}`) }}</span>
        <br v-if="(i + 1) % 2 === 0 && i < affiliatedEntity.roles.length - 1" />
        <template v-else-if="i < affiliatedEntity.roles.length - 1"> , </template>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import ImagePreview from '../ImagePreview.vue'
import type { AffiliatedEntityHierarchy } from '@/services/api-schema'

const { t } = useI18n()

defineProps<{
  affiliatedEntity: AffiliatedEntityHierarchy
}>()
</script>
<style scoped>
.affiliated-entity {
  display: flex;
  flex-direction: column;
  align-items: center;
}
.roles {
  font-size: 0.8em;
  text-align: center;
}
.role {
  font-weight: bold;
  color: var(--color-primary);
}
</style>

<style>
.affiliated-entity img {
  height: 5em !important;
  border-radius: 7px;
}
</style>
