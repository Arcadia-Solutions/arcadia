<template>
  <div id="user-stats-filters">
    <FloatLabel>
      <Select
        v-model="timeRange"
        :options="timeRangeOptions"
        optionLabel="label"
        optionValue="value"
        size="small"
        input-id="userTimeRangeSelect"
        style="min-width: 10em"
      />
      <label for="userTimeRangeSelect">{{ t('stats.time_range') }}</label>
    </FloatLabel>
    <FloatLabel>
      <Select
        v-model="interval"
        :options="intervalOptions"
        optionLabel="label"
        optionValue="value"
        size="small"
        input-id="userIntervalSelect"
        style="min-width: 10em"
      />
      <label for="userIntervalSelect">{{ t('stats.interval') }}</label>
    </FloatLabel>
  </div>
  <ProgressSpinner v-if="loading" />
  <div v-else-if="userStats">
    <div id="user-stats-summary">
      <ContentContainer :containerTitle="t('stats.new_users')">
        {{ formatNumber(userStats.new_users) }}
      </ContentContainer>
    </div>
    <h3>{{ t('stats.user_flux') }}</h3>
    <Chart class="chart" :options="userFluxChartOptions" />
  </div>
</template>

<script setup lang="ts">
import ContentContainer from '@/components/ContentContainer.vue'
import { Chart } from 'highcharts-vue'
import Highcharts from 'highcharts'
import ProgressSpinner from 'primevue/progressspinner'
import Select from 'primevue/select'
import FloatLabel from 'primevue/floatlabel'
import { useI18n } from 'vue-i18n'
import { computed, onMounted, ref, watch } from 'vue'
import { getUserStats, StatsInterval, type UserStatsResponse } from '@/services/api-schema'
import { formatDateToLocalString, formatDateTimeLabel, formatNumber } from '@/services/helpers'

const { t } = useI18n()

type TimeRange = 'this_week' | 'this_month' | 'this_year' | 'all_time'

const timeRange = ref<TimeRange>('this_year')
const interval = ref<StatsInterval>(StatsInterval.Month)

const timeRangeOptions = [
  { label: t('stats.this_week'), value: 'this_week' },
  { label: t('stats.this_month'), value: 'this_month' },
  { label: t('stats.this_year'), value: 'this_year' },
  { label: t('stats.all_time'), value: 'all_time' },
]

const intervalOptions = [
  { label: t('stats.hour'), value: StatsInterval.Hour },
  { label: t('stats.day'), value: StatsInterval.Day },
  { label: t('stats.week'), value: StatsInterval.Week },
  { label: t('stats.month'), value: StatsInterval.Month },
  { label: t('stats.year'), value: StatsInterval.Year },
]

const dateRangeFromSelection = computed(() => {
  const now = new Date()
  const to = now
  let from: Date
  switch (timeRange.value) {
    case 'this_week':
      from = new Date(now.getFullYear(), now.getMonth(), now.getDate() - 7)
      break
    case 'this_month':
      from = new Date(now.getFullYear(), now.getMonth(), now.getDate() - 30)
      break
    case 'this_year':
      from = new Date(now.getFullYear() - 1, now.getMonth(), now.getDate())
      break
    case 'all_time':
      // arcadia wasn't possibly used before this date :)
      from = new Date(2025, 0, 1)
      break
  }
  return { from, to }
})

const loading = ref(false)
const userStats = ref<UserStatsResponse>()

const CHART_COLOR = '#3B82F6'

const textColor = () => getComputedStyle(document.documentElement).getPropertyValue('color') || '#ccc'

const userFluxChartOptions = computed<Highcharts.Options>(() => {
  if (!userStats.value) return {}
  const data = userStats.value.data
  return {
    chart: { backgroundColor: 'transparent', type: 'line' },
    title: { text: undefined },
    credits: { enabled: false },
    legend: { enabled: false },
    xAxis: {
      categories: data.map((d) => formatDateTimeLabel(d.period, interval.value)),
      labels: { style: { color: textColor() } },
    },
    yAxis: {
      title: { text: undefined },
      labels: { style: { color: textColor() } },
    },
    series: [
      {
        type: 'line',
        name: t('stats.new_users'),
        data: data.map((d) => d.count),
        color: CHART_COLOR,
        marker: { enabled: false, states: { hover: { enabled: true, radius: 5 } } },
      },
    ],
    tooltip: {
      formatter() {
        const point = this as unknown as Highcharts.Point
        return `<b>${point.category}</b><br/>${point.series.name}: ${formatNumber(point.y ?? 0)}`
      },
    },
  }
})

const fetchUserStats = () => {
  const { from, to } = dateRangeFromSelection.value

  loading.value = true
  getUserStats({
    from: formatDateToLocalString(from),
    to: formatDateToLocalString(to),
    interval: interval.value,
  })
    .then((data) => {
      userStats.value = data
    })
    .finally(() => {
      loading.value = false
    })
}

onMounted(() => {
  fetchUserStats()
})

watch([timeRange, interval], () => {
  fetchUserStats()
})
</script>

<style scoped>
#user-stats-filters {
  display: flex;
  justify-content: center;
  gap: 15px;
  margin-bottom: 15px;
}

#user-stats-summary {
  display: flex;
  justify-content: center;
  flex-wrap: wrap;
  gap: 15px;
  margin-bottom: 25px;
  :deep(.content-body) {
    font-size: 1.2em;
    font-weight: bold;
    text-align: center;
  }
}

.chart {
  height: 30vh;
}

h3 {
  text-align: center;
  margin-bottom: 10px;
  font-weight: bold;
}
</style>
