<template>
  <ContentContainer>
    <Form @submit="() => emit('search')">
      <FloatLabel>
        <InputText v-model="name" name="name" size="small" />
        <label for="name">{{ t('general.name') }}</label>
      </FloatLabel>
      <div class="wrapper-center" style="margin-top: 15px">
        <Button :label="t('general.search')" type="submit" :loading />
      </div>
    </Form>
  </ContentContainer>
</template>
<script setup lang="ts">
import ContentContainer from '../ContentContainer.vue'
import { InputText, Button, FloatLabel } from 'primevue'
import { Form } from '@primevue/forms'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const name = ref('')
const loading = defineModel<boolean>('loading', { default: false })

const emit = defineEmits<{
  search: []
}>()

defineExpose({ name })
</script>
<style scoped></style>
