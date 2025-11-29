<template>
  <div v-if="isPageReady">
    <FloatLabel class="wiki-title" variant="in">
      <InputText v-model="wikiArticle.title" name="title" :format="false" />
      <label for="title">{{ t('general.title') }}</label>
    </FloatLabel>
    <BBCodeEditor :rows="30" :initial-value="wikiArticle.body" :label="t('wiki.article_body')" @value-change="wikiArticle.body = $event">
      <template #buttons>
        <Button v-if="isEditMode" :label="t('wiki.validate_article_edit')" :loading @click="editArticle" />
        <Button v-else :label="t('wiki.create_article')" :loading />
      </template>
    </BBCodeEditor>
  </div>
</template>

<script setup lang="ts">
import { editWikiArticle, getWikiArticle, type EditedWikiArticle } from '@/services/api/wikiService'
import { computed, ref } from 'vue'
import { onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import BBCodeEditor from '@/components/community/BBCodeEditor.vue'
import { useI18n } from 'vue-i18n'
import { Button, FloatLabel, InputText } from 'primevue'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()
const isEditMode = computed(() => useRoute().path.endsWith('/edit'))

const loading = ref(false)
const isPageReady = ref(false)
const wikiArticle = ref<EditedWikiArticle>({ id: 0, body: '', title: '' })

const editArticle = () => {
  loading.value = true
  editWikiArticle(wikiArticle.value)
    .then((data) => {
      router.push(`/wiki/article/${data.id}`)
    })
    .catch(() => (loading.value = false))
}

onMounted(async () => {
  if (isEditMode.value) {
    wikiArticle.value = await getWikiArticle(parseInt(route.params.id as string))
  }
  isPageReady.value = true
})
</script>

<style scoped>
.wiki-title {
  margin-bottom: 20px;
  input {
    width: 100%;
  }
}
</style>
