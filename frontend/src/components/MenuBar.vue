<template>
  <div class="menu-bar">
    <RouterLink v-for="item in menuItems" :key="item.label" class="item" :to="item.route">
      <Button severity="secondary" :label="item.label" size="small" />
    </RouterLink>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Button } from 'primevue'
import { onMounted } from 'vue'
import { useUserStore } from '@/stores/user'

const userStore = useUserStore()

const menuItems = ref([
  { label: 'Torrents', route: '/torrents' },
  { label: 'Series', route: '/series' },
  { label: 'Collages', route: '/collages' },
  { label: 'Requests', route: '/torrent-requests' },
  { label: 'Forum', route: '/forum' },
  { label: 'IRC', route: '' },
  { label: 'Top', route: '' },
  { label: 'Rules', route: '/wiki/article/1' },
  { label: 'Wiki', route: '/wiki/article/1' },
  { label: 'Tags', route: '/title-group-tags' },
])

onMounted(() => {
  if (userStore.class === 'staff') {
    menuItems.value.push({ label: 'Staff Dashboard', route: '/staff-dashboard' })
  }
})
</script>

<style scoped>
/* on mobile, the menu items here appear in MobileNavMenu.vue */
.menu-bar {
  display: none;
}

.item {
  margin: 0px 7px;
}

@media only screen and (min-width: 768px) {
  .menu-bar {
    display: flex;
    justify-content: center;
  }
}
</style>
