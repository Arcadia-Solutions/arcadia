<template>
  <div class="manage-allowed-posters">
    <UserSearchBar :placeholder="t('forum.add_allowed_poster')" :clickableUserLink="false" :clearInputOnSelect="true" modelValue="" @userSelected="addPoster" />
    <div v-for="user in allowedPosters" :key="user.id" class="poster-row">
      <UsernameEnriched :user="user" />
      <i class="pi pi-times cursor-pointer" @click="removePoster(user)" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import UserSearchBar from '@/components/user/UserSearchBar.vue'
import UsernameEnriched from '@/components/user/UsernameEnriched.vue'
import { getForumSubCategoryAllowedPosters, addForumSubCategoryAllowedPoster, removeForumSubCategoryAllowedPoster, type UserLite } from '@/services/api-schema'
import { showToast } from '@/main'

const { t } = useI18n()

const props = defineProps<{
  subCategoryId: number
}>()

const allowedPosters = ref<UserLite[]>([])

onMounted(() => {
  getForumSubCategoryAllowedPosters(props.subCategoryId).then((users) => {
    allowedPosters.value = users
  })
})

const addPoster = (user: UserLite) => {
  if (allowedPosters.value.some((p) => p.id === user.id)) return
  addForumSubCategoryAllowedPoster({ forum_sub_category_id: props.subCategoryId, user_id: user.id }).then(() => {
    allowedPosters.value.push(user)
    showToast('', t('forum.allowed_poster_added'), 'success', 2000)
  })
}

const removePoster = (user: UserLite) => {
  removeForumSubCategoryAllowedPoster({ forum_sub_category_id: props.subCategoryId, user_id: user.id }).then(() => {
    allowedPosters.value = allowedPosters.value.filter((p) => p.id !== user.id)
    showToast('', t('forum.allowed_poster_removed'), 'success', 2000)
  })
}
</script>

<style scoped>
.manage-allowed-posters {
  display: flex;
  flex-direction: column;
  gap: 10px;
}
.poster-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
}
</style>
