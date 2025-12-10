<template>
  <div class="not-found-container">
    <div class="not-found-content">
      <div class="error-code">
        <span class="four">4</span>
        <span class="zero">
          <i class="pi pi-spinner spin"></i>
        </span>
        <span class="four">4</span>
      </div>
      <h2 class="error-title">{{ t('error.page_not_found') }}</h2>
      <p class="error-message">{{ t('error.no_seeders') }}</p>
      <div class="error-stats">
        <span class="stat seeders"> <i class="pi pi-arrow-up"></i> 0 </span>
        <span class="stat leechers"> <i class="pi pi-arrow-down"></i> 1 </span>
      </div>
      <router-link to="/" class="home-button">
        <i class="pi pi-home"></i>
        {{ t('error.back_to_home') }}
      </router-link>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
</script>

<style scoped>
.not-found-container {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 60vh;
  padding: 2rem;
}

.not-found-content {
  text-align: center;
  max-width: 500px;
}

.error-code {
  font-size: 8rem;
  font-weight: bold;
  line-height: 1;
  margin-bottom: 1rem;
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 0.5rem;
}

.four {
  color: var(--p-primary-color, #60a5fa);
}

.zero {
  color: var(--p-red-500, #ef4444);
  display: flex;
  align-items: center;
  justify-content: center;
}

.zero i {
  font-size: 6rem;
}

.spin {
  animation: spin 2s linear infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.error-title {
  font-size: 1.75rem;
  margin-bottom: 0.5rem;
  color: var(--p-text-color);
}

.error-message {
  font-size: 1.1rem;
  color: var(--p-text-muted-color, #9ca3af);
  margin-bottom: 1.5rem;
}

.error-stats {
  display: flex;
  justify-content: center;
  gap: 2rem;
  margin-bottom: 2rem;
  font-size: 1.25rem;
}

.stat {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.seeders {
  color: var(--p-green-500, #22c55e);
}

.leechers {
  color: var(--p-red-500, #ef4444);
}

.home-button {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.5rem;
  background-color: var(--p-primary-color, #60a5fa);
  color: var(--p-primary-contrast-color, #fff);
  border-radius: 0.5rem;
  text-decoration: none;
  font-weight: 500;
  transition: background-color 0.2s;
}

.home-button:hover {
  background-color: var(--p-primary-hover-color, #3b82f6);
}
</style>
