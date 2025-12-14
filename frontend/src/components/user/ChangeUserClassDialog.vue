<template>
  <div class="change-user-class">
    <FloatLabel>
      <Select v-model="selectedClassName" :options="availableClasses" optionLabel="name" optionValue="name" class="class-select" />
      <label>{{ t('user.class') }}</label>
    </FloatLabel>
    <Button :label="t('general.save')" size="small" :loading @click="saveClass()" />
  </div>
</template>

<script setup lang="ts">
import { showToast } from '@/main'
import { changeUserClass, getAllUserClasses, type UserClass, type UserClassChange } from '@/services/api-schema'
import { FloatLabel, Select } from 'primevue'
import Button from 'primevue/button'
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps<{
  userId: number
  currentClassName: string
}>()

const selectedClassName = ref<string>(props.currentClassName)
const availableClasses = ref<UserClass[]>([])
const loading = ref(false)

const emit = defineEmits<{
  saved: [className: string]
}>()

onMounted(async () => {
  availableClasses.value = (await getAllUserClasses()) as unknown as UserClass[]
})

const saveClass = () => {
  loading.value = true
  const classChange: UserClassChange = {
    class_name: selectedClassName.value,
  }
  changeUserClass({
    id: props.userId,
    UserClassChange: classChange,
  }).then(() => {
    loading.value = false
    showToast('Success', t('user.class_changed_success'), 'success', 4000)
    emit('saved', selectedClassName.value)
  })
}
</script>

<style scoped>
.change-user-class {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20px;
}
.class-select {
  min-width: 20em;
}
</style>
