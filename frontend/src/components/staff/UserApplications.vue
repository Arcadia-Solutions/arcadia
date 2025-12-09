<template>
  <UserApplicationComponent v-for="userApplication in userApplications" :key="userApplication.id" :userApplication @applicationUpdated="applicationUpdated" />
</template>

<script setup lang="ts">
import { getUserApplications, type GetUserApplicationsQuery, type UserApplication } from '@/services/api-schema'
import UserApplicationComponent from './UserApplication.vue'
import { ref } from 'vue'
import { onMounted } from 'vue'

const loading = ref(true)

const filters = ref<GetUserApplicationsQuery>({
  page: 1,
  limit: 50,
  status: null,
})

const userApplications = ref<UserApplication[]>([])

const applicationUpdated = (app: UserApplication) =>
  (userApplications.value = userApplications.value.some((a) => a.id === app.id)
    ? userApplications.value.map((a) => (a.id === app.id ? app : a))
    : [...userApplications.value, app])

onMounted(() => {
  getUserApplications(filters.value)
    .then((data) => (userApplications.value = data))
    .finally(() => (loading.value = false))
})
</script>

<style scoped></style>
