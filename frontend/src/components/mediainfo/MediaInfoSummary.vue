<template>
  <!-- Row layout: 3 sections on one line -->
  <div class="mi-row">
    <!-- General -->
    <div class="mi-card">
      <div class="mi-card__header">General</div>
      <DataTable :value="summary.general" size="small" class="no-header mi-nowrap-table">
        <Column field="label" header="Label" body-class="font-medium w-1/2" />
        <Column field="value" header="Value" />
      </DataTable>
    </div>

    <!-- Video -->
    <div class="mi-card">
      <div class="mi-card__header">Video</div>
      <DataTable :value="summary.video" size="small" class="no-header mi-nowrap-table">
        <Column field="label" header="Label" body-class="font-medium w-1/2" />
        <Column field="value" header="Value" />
      </DataTable>
    </div>

    <!-- Audio -->
    <div class="mi-card">
      <div class="mi-card__header">Audio</div>
      <div v-if="summary.audioLines?.length" class="mi-audio">
        <div v-for="line in summary.audioLines" :key="line" class="mi-audio__line">
          {{ line }}
        </div>
      </div>
      <div v-else-if="summary.audioLine" class="mi-audio">
        {{ summary.audioLine }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import { normalizeMediaInfo, type Summary } from '@/services/fileinfo/mediainfo/normalizeMediaInfo'

const props = defineProps<{ source: unknown }>()
const summary = computed<Summary>(() => normalizeMediaInfo(props.source))
</script>

<style scoped>
/* Row layout */
.mi-row {
  display: flex;
  flex-direction: row;
  gap: 1rem;
  align-items: stretch;
}

/* Cards */
.mi-card {
  flex: 1 1 0;
  position: relative;
  border: 1px solid var(--surface-700);
  border-radius: 0.375rem;
  background: transparent;
  min-width: 0;
}

/* Headers */
.mi-card__header {
  padding: 0.5rem 0.75rem;
  font-weight: 700;
  font-size: 1.125rem;
  border-bottom: 1px solid var(--surface-700);
  white-space: nowrap;
}

/* DataTable tweaks */
:deep(.no-header .p-datatable-thead) {
  display: none;
}
:deep(.p-datatable .p-datatable-tbody > tr > td) {
  padding: 0.35rem 0.75rem;
  border: 0;
}
:deep(.p-datatable .p-datatable-tbody > tr:not(:last-child) > td) {
  border-bottom: 1px solid var(--surface-700);
}

/* Keep General/Video rows on one line */
.mi-nowrap-table :deep(.p-datatable-tbody > tr > td) {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* Audio allows wrapping */
.mi-audio {
  padding: 0.5rem 0.75rem 2rem;
  font-size: 0.95rem;
  line-height: 1.35rem;
  white-space: normal;
  word-break: break-word;
}
.mi-audio__line + .mi-audio__line {
  margin-top: 0.25rem;
}
</style>
