<template>
  <div></div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { getTorrentTitleGroupId } from '@/services/api-schema'

const route = useRoute()
const router = useRouter()

onMounted(() => {
  const torrentId = Number(route.params.id)

  getTorrentTitleGroupId(torrentId)
    .then((response) => {
      router.replace({
        path: `/title-group/${response.title_group_id}`,
        query: { torrentId: String(torrentId) },
      })
    })
    .catch(() => {
      router.replace('/404')
    })
})
</script>

<style scoped></style>
