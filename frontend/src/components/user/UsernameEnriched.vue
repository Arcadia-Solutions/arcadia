<template>
  <div class="username-enriched">
    <RouterLink :to="`/user/${user.id}`">
      {{ user.username }}
    </RouterLink>
    <i v-if="user.banned" v-tooltip.top="t('user.banned')" class="danger pi pi-ban" />
    <i v-if="!user.banned && user.warned" v-tooltip.top="t('user.warned')" class="warning pi pi-exclamation-triangle" />
  </div>
</template>

<script setup lang="ts">
import { RouterLink } from 'vue-router'
import { useI18n } from 'vue-i18n'
import type { UserLite } from '@/services/api-schema'

const { t } = useI18n()

defineProps<{
  user: UserLite
}>()
</script>
<style scoped>
.username-enriched {
  display: inline;
}
.pi {
  margin-left: 4px;
}
</style>
