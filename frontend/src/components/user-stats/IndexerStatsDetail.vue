<template>
  <div id="indexer-stats-detail">
    <div class="wrapper-center">
      <MultiSelect
        v-model="selectedValuesForLineChart"
        :options="displayableOnLineChart"
        filter
        display="chip"
        placeholder="Charts to display"
      />
    </div>
    <div class="charts">
      <Chart
        v-for="value in selectedValuesForLineChart"
        :key="value"
        type="line"
        :data="chartData(value)"
        :options="chartOptions(value)"
      />
    </div>
  </div>
</template>
<script lang="ts" setup>
import type { UserProfileVec, UserProfileScrapedVec } from '@/services/api/userStatsService'
import Chart from 'primevue/chart'
import 'chartjs-adapter-date-fns'
import { MultiSelect } from 'primevue'
import { ref } from 'vue'

const props = defineProps<{
  userStats: UserProfileVec
}>()

const chartOptions = (value: keyof UserProfileScrapedVec) => {
  let unit = ''
  switch (value) {
    case 'uploaded':
    case 'downloaded':
      unit = 'GiB'
      break
  }
  return {
    scales: {
      x: {
        type: 'time',
        time: {
          unit: 'month',
        },
      },
      y: {
        title: {
          display: true,
          text: unit,
        },
      },
    },
  }
}
const displayableOnLineChart = ref<(keyof UserProfileScrapedVec)[]>([
  'downloaded',
  'uploaded',
  'leeching',
  'seeding',
])
const selectedValuesForLineChart = ref<(typeof displayableOnLineChart.value)[number][]>([
  'uploaded',
  'downloaded',
])

const chartData = (value: (typeof displayableOnLineChart.value)[number]) => {
  let data: number[] = []
  switch (value) {
    case 'uploaded':
    case 'downloaded':
      data = props.userStats.profile[value].map((val) => val / 1024 / 1024 / 1024)
      break
    default:
      data = props.userStats.profile[value] as number[]
  }
  return {
    labels: props.userStats.scraped_at.map((date) => new Date(date).toISOString()),
    datasets: [
      {
        label: value,
        data: data,
        title: {
          display: true,
          text: 'Weight (kg)',
        },
      },
    ],
  }
}
</script>
<style scoped>
.charts {
  display: flex;
  justify-content: center;
  margin-top: 20px;
  .p-chart {
    background-color: var(--background-primary);
    padding: 10px;
    border-radius: 7px;
    width: 40%;
    margin: 10px;
  }
}
</style>
