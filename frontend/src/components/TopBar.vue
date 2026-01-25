<template>
  <div id="top-bar">
    <div class="left">
      <div class="wrapper-center logo-wrapper" style="display: flex; flex-direction: column">
        <RouterLink to="/">
          <img src="@/assets/logo.svg" alt="Site Logo" class="logo" />
        </RouterLink>
        <span id="logo-subtitle" v-if="publicArcadiaSettings.logo_subtitle">{{ publicArcadiaSettings.logo_subtitle }}</span>
      </div>
    </div>
    <div class="user-stats">
      <div class="stat" v-tooltip.bottom="'Uploaded'">
        <i class="pi pi-upload" />
        {{ bytesToReadable(user.uploaded) }}
      </div>
      <span class="stat" v-tooltip.bottom="'Downloaded'"> <i class="pi pi-download" />{{ bytesToReadable(user.downloaded) }} </span>
      <span class="stat" v-tooltip.bottom="'Ratio'"> <i class="pi pi-wave-pulse" />{{ (user.uploaded / user.downloaded).toFixed(2) }} </span>
      <RouterLink to="/shop" v-tooltip.bottom="'Bonus points'" class="stat clickable-stat"> <i class="pi pi-money-bill" />{{ user.bonus_points }} </RouterLink>
      <!-- <span class="stat" v-tooltip.bottom="'Freeleech tokens'"> <i class="pi pi-ticket" />{{ user.freeleech_tokens }} </span> -->
    </div>
    <div class="right">
      <NavMenu />
    </div>
  </div>
</template>

<script setup lang="ts">
import { useUserStore } from '@/stores/user'
import { bytesToReadable } from '@/services/helpers'
import NavMenu from './nav_menu/NavMenu.vue'
import { usePublicArcadiaSettingsStore } from '@/stores/publicArcadiaSettings'
import { RouterLink } from 'vue-router'

const user = useUserStore()
const publicArcadiaSettings = usePublicArcadiaSettingsStore()
</script>

<style scoped>
#top-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background-color: var(--color-background-secondary);
  height: 45px;
  padding: 0 7px;
  width: 100%;
}

.left .logo {
  height: 40px;
  vertical-align: middle;
}

#logo-subtitle {
  font-size: 0.85em;
  margin-top: -4px;
  margin-bottom: -2px;
  font-weight: bold;
}

.user-stats {
  font-size: 0.85em;
  display: flex;
  align-items: center;

  .stat {
    margin: 0px 10px;
    display: flex;
    align-items: center;
  }

  i {
    margin-right: 7px;
  }

  .clickable-stat {
    cursor: pointer;
    text-decoration: none;
    color: inherit;
    transition: color 0.2s;

    &:hover {
      color: var(--color-primary);
    }
  }
}
</style>
