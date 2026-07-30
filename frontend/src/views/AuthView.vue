<template>
  <div class="auth-view">
    <div class="title">{{ siteName }}</div>
    <div class="auth-links">
      <RouterLink to="/apply" v-if="route.path !== '/apply' && route.path !== '/register'">
        {{ t('user.apply') }}
      </RouterLink>
      <RouterLink to="/login" v-if="route.path !== '/login'">{{ t('user.login') }}</RouterLink>
    </div>
    <LoginForm v-if="route.path.includes('login')" />
    <RegisterForm v-if="route.path.includes('register')" />
    <ApplyForm v-if="route.path.includes('apply')" />
  </div>
</template>
<script lang="ts" setup>
import { config } from '@/config'
import ApplyForm from '@/components/auth/ApplyForm.vue'
import LoginForm from '@/components/auth/LoginForm.vue'
import RegisterForm from '@/components/auth/RegisterForm.vue'
import { useI18n } from 'vue-i18n'
import { RouterLink } from 'vue-router'
import { useRoute } from 'vue-router'

const route = useRoute()
const { t } = useI18n()
const siteName = config.site_name
</script>

<style>
.auth-view {
  height: 100vh;
  align-items: center;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.form-item {
  margin-top: 10px;
}

.auth-links {
  text-align: center;
  margin-bottom: 1em;
}
</style>
