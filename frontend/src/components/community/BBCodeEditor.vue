<template>
  <div class="bbcode-editor">
    <div class="boilerplate-inserters" v-if="!preview">
      <Button size="small" @click="insertBoilerplate('[b]', '[/b]')" v-tooltip.top="t('bbcode.bold')">
        <template #icon>
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="-1.5 0 24 24">
            <path
              d="M21.2 7c0-3.8-3.2-7-7-7h-12a2.2 2.2 0 0 0 0 4.4h1v15.2h-1a2.2 2.2 0 0 0 0 4.4h12a7 7 0 0 0 5-12 7 7 0 0 0 2-5zm-7 2.8H7.4V4.4H14a2.7 2.7 0 0 1 0 5.4zm0 9.8H7.4v-5.4H14a2.7 2.7 0 0 1 0 5.4z"
            />
          </svg>
        </template>
      </Button>
      <Button size="small" @click="insertBoilerplate('[i]', '[/i]')" v-tooltip.top="t('bbcode.italic')">
        <template #icon>
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <path stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 3h10M4 21h10m1-18L9 21" />
          </svg>
        </template>
      </Button>
      <Button size="small" @click="insertBoilerplate('[s]', '[/s]')" v-tooltip.top="t('bbcode.strike')">
        <template #icon>
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <g fill="none" fill-rule="nonzero">
              <path d="M24 0v24H0V0h24ZM12.6 23.3h-.2v.5h.2v-.5Zm.3-.2-.2.1v.5l.2.1v-.6Zm-.8 0v.7h.2v-.5l-.2-.2Z" />
              <path
                fill="#000000"
                d="M19 12a1 1 0 0 1 .1 2H17a4.5 4.5 0 0 1-3.6 7h-1.9c-2 0-4-1.1-5-3l-.1-.2-.2-.3a1 1 0 0 1 0-.6 1 1 0 0 1 1.7-.5v.1l.3.4c.6 1.2 1.8 2 3.1 2l.2.1h1.7a2.5 2.5 0 0 0 1.3-4.7h-.1l-.5-.3H5a1 1 0 0 1-.1-2H19Zm-6.4-9c2.1 0 4.1 1.2 5.1 3.2l.2.3a1 1 0 0 1 0 1 1 1 0 0 1-.7.5 1 1 0 0 1-1-.5l-.3-.4C15.3 5.8 14 5 12.6 5h-1.7a2.5 2.5 0 0 0-1.2 4.8l2.5 1.2H8A4.5 4.5 0 0 1 11 3h1.7Z"
              />
            </g>
          </svg>
        </template>
      </Button>
      <Button size="small" @click="insertBoilerplate('[u]', '[/u]')" v-tooltip.top="t('bbcode.underline')">
        <template #icon>
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <path stroke="#000000" stroke-linecap="round" stroke-width="2" d="M7 4v7a5 5 0 0 0 10 0V4" />
            <path stroke="#000000" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 20h14" />
          </svg>
        </template>
      </Button>
      <Button
        size="small"
        @click="insertBoilerplate('[img width=&quot;100&quot; height=&quot;50&quot; alt=&quot;&quot; title=&quot;&quot;]', '[/img]')"
        v-tooltip.top="t('bbcode.image')"
      >
        <template #icon>
          <i class="pi pi-image" />
        </template>
      </Button>
      <Button size="small" @click="insertBoilerplate('[url=]', '[/url]', -1)" v-tooltip.top="t('bbcode.link')">
        <template #icon>
          <i class="pi pi-link" />
        </template>
      </Button>
      <Button size="small" @click="insertBoilerplate('[list]\n[*]', '\n[/list]')" v-tooltip.top="t('bbcode.unordered_list')">
        <template #icon>
          <i class="pi pi-list" />
        </template>
      </Button>
      <Button size="small" @click="insertBoilerplate('[color=]', '[/color]', -1)" v-tooltip.top="t('bbcode.color')">
        <template #icon>
          <i class="pi pi-palette" />
        </template>
      </Button>
      <!-- Not supported yet: https://github.com/JiLiZART/BBob/issues/300 -->
      <!-- <Button size="small" @click="insertBoilerplate('[size=]', '[/size]', -1)" v-tooltip.top="t('bbcode.font_size')">
        <template #icon>
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="3 2 20 20">
            <path
              d="M17 12v5h.5a.5.5 0 1 1 0 1h-2a.5.5 0 1 1 0-1h.5v-5h-2v.5a.5.5 0 1 1-1 0v-1q0-.5.5-.5h6q.5 0 .5.5v1a.5.5 0 1 1-1 0V12zm-7-6v11h1.5a.5.5 0 1 1 0 1h-4a.5.5 0 1 1 0-1H9V6H5v1.5a.5.5 0 0 1-1 0v-2q0-.5.5-.5h10q.5 0 .5.5v2a.5.5 0 1 1-1 0V6z"
            />
          </svg>
        </template>
      </Button> -->
      <Button size="small" @click="insertBoilerplate('[left]', '[/left]')" v-tooltip.top="t('bbcode.align_left')">
        <template #icon>
          <i class="pi pi-align-left" />
        </template>
      </Button>
      <Button size="small" @click="insertBoilerplate('[center]', '[/center]')" v-tooltip.top="t('bbcode.align_center')">
        <template #icon>
          <i class="pi pi-align-center" />
        </template>
      </Button>
      <Button size="small" @click="insertBoilerplate('[right]', '[/right]')" v-tooltip.top="t('bbcode.align_right')">
        <template #icon>
          <i class="pi pi-align-right" />
        </template>
      </Button>
      <Button size="small" @click="insertBoilerplate('[quote]', '[/quote]')" v-tooltip.top="t('bbcode.quote')">
        <template #icon>
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32">
            <path
              d="M6.67 26.06c.09 0 8.77-.14 8.77-11.89a7.22 7.22 0 1 0-9.77 6.73v4.16a1 1 0 0 0 1 1M3 14.17a5.22 5.22 0 1 1 10.44 0c0 7.5-3.88 9.31-5.77 9.75v-3.75a1 1 0 0 0-.75-1 5.2 5.2 0 0 1-3.92-5"
            />
            <path
              d="M22.22 26.06c.09 0 8.78-.14 8.78-11.89a7.22 7.22 0 1 0-9.78 6.73v4.16a1 1 0 0 0 1 1m-3.66-11.89a5.22 5.22 0 1 1 10.44 0c0 7.5-3.89 9.31-5.78 9.75v-3.75a1 1 0 0 0-.75-1 5.2 5.2 0 0 1-3.91-5"
            />
          </svg>
        </template>
      </Button>
      <Button size="small" @click="insertBoilerplate('[code]', '[/code]')" v-tooltip.top="t('bbcode.code')">
        <template #icon>
          <i class="pi pi-code" />
        </template>
      </Button>
    </div>
    <FloatLabel style="width: 100%" variant="in" v-if="!preview">
      <Textarea ref="textareaRef" v-model="content" :rows style="width: 100%" @value-change="emit('valueChange', content)" :name="name" />
      <label for="in_label">{{ label }}</label>
    </FloatLabel>
    <div class="message">
      <slot name="message"></slot>
    </div>
    <ContentContainer v-if="preview">
      <BBCodeRenderer :content />
    </ContentContainer>
    <slot name="belowInput"></slot>
    <div class="actions">
      <Button v-tooltip.top="t('general.preview')" :icon="`pi pi-eye${preview ? '-slash' : ''}`" @click="preview = !preview" />
      <slot name="buttons"></slot>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { FloatLabel, Textarea } from 'primevue'
import ContentContainer from '../ContentContainer.vue'
import { watch } from 'vue'
import Button from 'primevue/button'
import { useI18n } from 'vue-i18n'
import BBCodeRenderer from '../community/BBCodeRenderer.vue'
import { onMounted } from 'vue'
import { nextTick } from 'vue'

const props = withDefaults(
  defineProps<{
    label: string
    name?: string
    emptyInput?: boolean
    initialValue?: string
    rows?: number
  }>(),
  {
    rows: 5,
  },
)

const emit = defineEmits<{
  inputEmptied: [boolean]
  valueChange: [string]
}>()

const { t } = useI18n()

const content = ref('')
const preview = ref(false)

const textareaRef = ref<InstanceType<typeof Textarea> | null>(null)
const insertBoilerplate = (openTag: string, closeTag: string, cursorOffset: number = 0) => {
  // @ts-expect-error TODO: properly type the textarea ref
  const textareaEl = textareaRef.value?.$el as HTMLTextAreaElement
  if (!textareaEl) return

  const start = textareaEl.selectionStart
  const end = textareaEl.selectionEnd
  const currentVal = content.value
  const selectedText = currentVal.substring(start, end)

  const replacement = openTag + selectedText + closeTag

  // Update the content
  content.value = currentVal.substring(0, start) + replacement + currentVal.substring(end)

  emit('valueChange', content.value)

  // nextTick ensures the DOM has updated *before* we try to set focus/selection
  nextTick(() => {
    // Restore focus
    textareaEl.focus()

    const newCursorPos = start + openTag.length + cursorOffset

    // Set the new cursor position
    textareaEl.setSelectionRange(newCursorPos, newCursorPos)
  })
}

onMounted(() => {
  content.value = props.initialValue ?? content.value
})

watch(
  () => props.emptyInput,
  (newVal) => {
    if (newVal) {
      content.value = ''
      emit('inputEmptied', true)
    }
  },
)
</script>
<style scoped>
.bbcode-editor {
  width: 100%;
}
.boilerplate-inserters {
  width: 100%;
  margin-bottom: 5px;
  display: flex;
  .p-button {
    margin-right: 4px;
    padding: 0.45em;
  }
}
.message {
  height: 0;
}
.actions {
  text-align: right;
}
</style>

<style>
.bbcode-editor {
  .actions {
    .p-button {
      margin-left: 5px;
    }
  }
}
</style>
