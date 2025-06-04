<template>
  <h1 class="title">Search</h1>

  <Form class="wrapper" @submit="onSubmit">
    <InputText name="title" type="text" :placeholder="t('general.title')" v-model="searchForm.title" size="small"
      class="input" />
    <Button type="submit">button</Button>
  </Form>
</template>

<script setup lang="ts">
import InputText from 'primevue/inputtext'
import { useI18n } from 'vue-i18n'
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { Button } from 'primevue'
import { Form, type FormSubmitEvent } from '@primevue/forms'

const { t } = useI18n()
const router = useRouter()

const searchForm = ref({
  title: '',
})

const onSubmit = (e: FormSubmitEvent<Record<string, string>>) => {
  const { title } = e.values

  const shouldRedirect = router.currentRoute.value.path === "/forum"

  if (shouldRedirect) {
    return router.push(`/forum/search?title=${title}`)
  }
}

</script>

<style scoped>
.wrapper {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 10px;
}

.title-search {
  margin-bottom: 10px;
}

.input {}
</style>
